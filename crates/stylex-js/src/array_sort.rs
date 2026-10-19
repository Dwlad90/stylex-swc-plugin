//! `Array.prototype.sort` with a comparator, as V8 sorts.
//!
//! The language leaves the sort algorithm open, so when a comparator is not a
//! consistent order (one that answers `NaN`, or `a < b` and `b < a`), the
//! result is what one engine's algorithm makes of it. The output a port must
//! match is the output Node writes, so this is V8's TimSort
//! (`third_party/v8/builtins/array-sort.tq` at 13.6.233, the V8 of Node 24),
//! function by function with V8's names: each comparison is the one V8 makes,
//! with the same two operands in the same order. Rust's `sort_by` visits other
//! pairs, so it gives another order for such a comparator.
//!
//! The comparator answers a JavaScript number. As in V8's
//! `SortCompareUserFn`, `NaN` reads as 0, and only the sign of the answer is
//! used after that.

use std::cmp::{max, min};

/// When the merge goes into galloping mode, it stays there until both runs win
/// fewer than this many times in a row (`kMinGallopWins`).
const MIN_GALLOP_WINS: usize = 7;

/// `array.sort(compare)`. The items are sorted in a copy and written back only
/// when the sort completes, as V8 sorts in its work array: when `compare`
/// returns an error, the sort stops at that comparison and `items` keeps its
/// original order.
pub fn array_tim_sort<T: Copy, E>(
  items: &mut [T],
  mut compare: impl FnMut(&T, &T) -> Result<f64, E>,
) -> Result<(), E> {
  if items.len() < 2 {
    return Ok(());
  }

  // The comparator is called through a reference, so the sort is compiled
  // once per item type and not once per comparator.
  let mut state = SortState {
    work: items.to_vec(),
    temp: Vec::new(),
    pending_runs: Vec::new(),
    min_gallop: MIN_GALLOP_WINS,
    compare: &mut compare,
  };

  state.array_tim_sort_impl()?;
  items.copy_from_slice(&state.work);

  Ok(())
}

/// One sorted run on the pending stack: where it starts and how long it is.
#[derive(Clone, Copy)]
struct Run {
  base: usize,
  length: usize,
}

/// The array a gallop searches: the work array, or the temporary copy of the
/// run a merge moved out of the way.
#[derive(Clone, Copy)]
enum Source {
  Work,
  Temp,
}

/// Where a merge ends: every element is in place but the rest of the copied
/// run, or one run has one element left, which goes after the other run.
enum MergeEnd {
  Succeed,
  CopyLast,
}

/// A comparator: the JavaScript number it answers for two items, or the error
/// that stops the sort.
type Comparator<'c, T, E> = &'c mut dyn FnMut(&T, &T) -> Result<f64, E>;

struct SortState<'c, T, E> {
  work: Vec<T>,
  temp: Vec<T>,
  pending_runs: Vec<Run>,
  min_gallop: usize,
  compare: Comparator<'c, T, E>,
}

/// `ComputeMinRunLength`: `n` itself below 64, else a length from 32 to 64
/// that splits `n` into runs of near equal length.
fn compute_min_run_length(length: usize) -> usize {
  let mut n = length;
  // Becomes 1 when any 1 bit is shifted off.
  let mut r = 0;

  while n >= 64 {
    r |= n & 1;
    n >>= 1;
  }

  n + r
}

impl<T: Copy, E> SortState<'_, T, E> {
  /// `sortState.Compare`: the comparator's answer, with `NaN` read as 0.
  fn compare(&mut self, x: T, y: T) -> Result<f64, E> {
    let order = (self.compare)(&x, &y)?;

    Ok(if order.is_nan() { 0.0 } else { order })
  }

  fn element(&self, source: Source, index: usize) -> T {
    match source {
      Source::Work => self.work[index],
      Source::Temp => self.temp[index],
    }
  }

  /// `ArrayTimSortImpl`: finds or makes each run, pushes it, and merges the
  /// stack while its length invariant does not hold.
  fn array_tim_sort_impl(&mut self) -> Result<(), E> {
    let mut remaining = self.work.len();
    let mut low = 0;
    let min_run_length = compute_min_run_length(remaining);

    while remaining != 0 {
      let mut current_run_length = self.count_and_make_run(low, low + remaining)?;

      // A short run is extended to the minimum run length.
      if current_run_length < min_run_length {
        let forced_run_length = min(min_run_length, remaining);
        self.binary_insertion_sort(low, low + current_run_length, low + forced_run_length)?;
        current_run_length = forced_run_length;
      }

      self.pending_runs.push(Run {
        base: low,
        length: current_run_length,
      });
      self.merge_collapse()?;

      low += current_run_length;
      remaining -= current_run_length;
    }

    self.merge_force_collapse()
  }

  /// `BinaryInsertionSort`: sorts `low..high`, of which `low..start` is
  /// already sorted, inserting each element after the last one it does not
  /// sort before. V8 also handles `low == start`; the one caller here passes
  /// a run, which is never empty, so `start` is always past `low`.
  fn binary_insertion_sort(&mut self, low: usize, start: usize, high: usize) -> Result<(), E> {
    let mut start = start;

    while start < high {
      let mut left = low;
      let mut right = start;
      let pivot = self.work[right];

      while left < right {
        let mid = left + ((right - left) >> 1);
        let order = self.compare(pivot, self.work[mid])?;

        if order < 0.0 {
          right = mid;
        } else {
          left = mid + 1;
        }
      }

      self.work.copy_within(left..start, left + 1);
      self.work[left] = pivot;
      start += 1;
    }

    Ok(())
  }

  /// `CountAndMakeRun`: the length of the run that starts at `low`. A run is
  /// non-descending, or strictly descending and then reversed in place.
  fn count_and_make_run(&mut self, low_arg: usize, high: usize) -> Result<usize, E> {
    let low = low_arg + 1;
    if low == high {
      return Ok(1);
    }

    let mut run_length = 2;
    let element_low = self.work[low];
    let element_low_pred = self.work[low - 1];
    let is_descending = self.compare(element_low, element_low_pred)? < 0.0;

    let mut previous_element = element_low;
    for index in low + 1..high {
      let current_element = self.work[index];
      let order = self.compare(current_element, previous_element)?;

      let run_ends = if is_descending {
        order >= 0.0
      } else {
        order < 0.0
      };
      if run_ends {
        break;
      }

      previous_element = current_element;
      run_length += 1;
    }

    if is_descending {
      self.work[low_arg..low_arg + run_length].reverse();
    }

    Ok(run_length)
  }

  /// `RunInvariantEstablished`: whether run `n - 2` is longer than runs
  /// `n - 1` and `n` together.
  fn run_invariant_established(&self, n: usize) -> bool {
    if n < 2 {
      return true;
    }

    let runs = &self.pending_runs;
    runs[n - 2].length > runs[n - 1].length + runs[n].length
  }

  /// `MergeCollapse`: merges runs on the stack until the invariant holds.
  fn merge_collapse(&mut self) -> Result<(), E> {
    while self.pending_runs.len() > 1 {
      let mut n = self.pending_runs.len() - 2;

      if !self.run_invariant_established(n + 1) || !self.run_invariant_established(n) {
        if self.pending_runs[n - 1].length < self.pending_runs[n + 1].length {
          n -= 1;
        }

        self.merge_at(n)?;
      } else if self.pending_runs[n].length <= self.pending_runs[n + 1].length {
        self.merge_at(n)?;
      } else {
        break;
      }
    }

    Ok(())
  }

  /// `MergeForceCollapse`: merges every run on the stack into one, the top
  /// two each time. V8 merges one place lower when run `n - 1` is shorter
  /// than run `n + 1`. That cannot occur here: `merge_collapse` leaves each
  /// run longer than the two above it together, and a merge of the top two
  /// keeps it so.
  fn merge_force_collapse(&mut self) -> Result<(), E> {
    while self.pending_runs.len() > 1 {
      self.merge_at(self.pending_runs.len() - 2)?;
    }

    Ok(())
  }

  /// `MergeAt`: merges runs `i` and `i + 1`. The elements of the first run
  /// that already sort before the second run, and the elements of the second
  /// that already sort after the first, stay where they are.
  fn merge_at(&mut self, i: usize) -> Result<(), E> {
    let stack_size = self.pending_runs.len();
    let Run {
      base: mut base_a,
      length: mut length_a,
    } = self.pending_runs[i];
    let Run {
      base: base_b,
      length: mut length_b,
    } = self.pending_runs[i + 1];

    self.pending_runs[i].length = length_a + length_b;
    if i + 3 == stack_size {
      self.pending_runs[i + 1] = self.pending_runs[i + 2];
    }
    self.pending_runs.truncate(stack_size - 1);

    let key_right = self.work[base_b];
    let k = self.gallop_right(Source::Work, key_right, base_a, length_a, 0)?;

    base_a += k;
    length_a -= k;
    if length_a == 0 {
      return Ok(());
    }

    let key_left = self.work[base_a + length_a - 1];
    length_b = self.gallop_left(Source::Work, key_left, base_b, length_b, length_b - 1)?;
    if length_b == 0 {
      return Ok(());
    }

    if length_a <= length_b {
      self.merge_low(base_a, length_a, base_b, length_b)
    } else {
      self.merge_high(base_a, length_a, base_b, length_b)
    }
  }

  /// `GallopLeft`: where `key` goes in the sorted `base..base + length` of
  /// `source`, before every element equal to it. The search starts at `hint`
  /// and widens by powers of two before the binary search.
  fn gallop_left(
    &mut self,
    source: Source,
    key: T,
    base: usize,
    length: usize,
    hint: usize,
  ) -> Result<usize, E> {
    let (length, hint) = (length as isize, hint as isize);
    let at = |offset: isize| base.wrapping_add_signed(offset);
    let mut last_ofs: isize = 0;
    let mut offset: isize = 1;

    let base_hint_element = self.element(source, at(hint));

    if self.compare(base_hint_element, key)? < 0.0 {
      // a[base + hint] < key: gallop right until
      // a[base + hint + lastOfs] < key <= a[base + hint + offset].
      let max_ofs = length - hint;
      while offset < max_ofs {
        let offset_element = self.element(source, at(hint + offset));
        if self.compare(offset_element, key)? >= 0.0 {
          break;
        }

        last_ofs = offset;
        offset = (offset << 1) + 1;
      }

      offset = min(offset, max_ofs);
      last_ofs += hint;
      offset += hint;
    } else {
      // key <= a[base + hint]: gallop left until
      // a[base + hint - offset] < key <= a[base + hint - lastOfs].
      let max_ofs = hint + 1;
      while offset < max_ofs {
        let offset_element = self.element(source, at(hint - offset));
        if self.compare(offset_element, key)? < 0.0 {
          break;
        }

        last_ofs = offset;
        offset = (offset << 1) + 1;
      }

      offset = min(offset, max_ofs);
      let tmp = last_ofs;
      last_ofs = hint - offset;
      offset = hint - tmp;
    }

    // Now a[base + lastOfs] < key <= a[base + offset]: a binary search
    // between the two.
    last_ofs += 1;
    while last_ofs < offset {
      let m = last_ofs + ((offset - last_ofs) >> 1);

      if self.compare(self.element(source, at(m)), key)? < 0.0 {
        last_ofs = m + 1;
      } else {
        offset = m;
      }
    }

    Ok(offset as usize)
  }

  /// `GallopRight`: like [`Self::gallop_left`], but `key` goes after every
  /// element equal to it.
  fn gallop_right(
    &mut self,
    source: Source,
    key: T,
    base: usize,
    length: usize,
    hint: usize,
  ) -> Result<usize, E> {
    let (length, hint) = (length as isize, hint as isize);
    let at = |offset: isize| base.wrapping_add_signed(offset);
    let mut last_ofs: isize = 0;
    let mut offset: isize = 1;

    let base_hint_element = self.element(source, at(hint));

    if self.compare(key, base_hint_element)? < 0.0 {
      // key < a[base + hint]: gallop left until
      // a[base + hint - offset] <= key < a[base + hint - lastOfs].
      let max_ofs = hint + 1;
      while offset < max_ofs {
        let offset_element = self.element(source, at(hint - offset));
        if self.compare(key, offset_element)? >= 0.0 {
          break;
        }

        last_ofs = offset;
        offset = (offset << 1) + 1;
      }

      offset = min(offset, max_ofs);
      let tmp = last_ofs;
      last_ofs = hint - offset;
      offset = hint - tmp;
    } else {
      // a[base + hint] <= key: gallop right until
      // a[base + hint + lastOfs] <= key < a[base + hint + offset].
      let max_ofs = length - hint;
      while offset < max_ofs {
        let offset_element = self.element(source, at(hint + offset));
        if self.compare(key, offset_element)? < 0.0 {
          break;
        }

        last_ofs = offset;
        offset = (offset << 1) + 1;
      }

      offset = min(offset, max_ofs);
      last_ofs += hint;
      offset += hint;
    }

    last_ofs += 1;
    while last_ofs < offset {
      let m = last_ofs + ((offset - last_ofs) >> 1);

      if self.compare(key, self.element(source, at(m)))? < 0.0 {
        offset = m;
      } else {
        last_ofs = m + 1;
      }
    }

    Ok(offset as usize)
  }

  /// `MergeLow`: merges two adjacent runs, the first no longer than the
  /// second, left to right, with the first run copied out of the way.
  fn merge_low(
    &mut self,
    base_a: usize,
    length_a: usize,
    base_b: usize,
    length_b: usize,
  ) -> Result<(), E> {
    let (mut length_a, mut length_b) = (length_a, length_b);

    self.temp.clear();
    self
      .temp
      .extend_from_slice(&self.work[base_a..base_a + length_a]);

    let mut dest = base_a;
    let mut cursor_temp = 0;
    let mut cursor_b = base_b;

    self.work[dest] = self.work[cursor_b];
    dest += 1;
    cursor_b += 1;

    let end = 'merge: {
      length_b -= 1;
      if length_b == 0 {
        break 'merge MergeEnd::Succeed;
      }
      if length_a == 1 {
        break 'merge MergeEnd::CopyLast;
      }

      let mut min_gallop = self.min_gallop;
      loop {
        // The number of times A, and B, won in a row.
        let mut nof_wins_a = 0;
        let mut nof_wins_b = 0;

        // One pair at a time, until one run wins consistently.
        loop {
          let order = self.compare(self.work[cursor_b], self.temp[cursor_temp])?;

          if order < 0.0 {
            self.work[dest] = self.work[cursor_b];
            dest += 1;
            cursor_b += 1;
            nof_wins_b += 1;
            length_b -= 1;
            nof_wins_a = 0;

            if length_b == 0 {
              break 'merge MergeEnd::Succeed;
            }
            if nof_wins_b >= min_gallop {
              break;
            }
          } else {
            self.work[dest] = self.temp[cursor_temp];
            dest += 1;
            cursor_temp += 1;
            nof_wins_a += 1;
            length_a -= 1;
            nof_wins_b = 0;

            if length_a == 1 {
              break 'merge MergeEnd::CopyLast;
            }
            if nof_wins_a >= min_gallop {
              break;
            }
          }
        }

        // Galloping, until neither run wins consistently any more.
        min_gallop += 1;
        let mut first_iteration = true;
        while nof_wins_a >= MIN_GALLOP_WINS || nof_wins_b >= MIN_GALLOP_WINS || first_iteration {
          first_iteration = false;

          min_gallop = max(1, min_gallop - 1);
          self.min_gallop = min_gallop;

          nof_wins_a =
            self.gallop_right(Source::Temp, self.work[cursor_b], cursor_temp, length_a, 0)?;
          if nof_wins_a > 0 {
            self.work[dest..dest + nof_wins_a]
              .copy_from_slice(&self.temp[cursor_temp..cursor_temp + nof_wins_a]);
            dest += nof_wins_a;
            cursor_temp += nof_wins_a;
            length_a -= nof_wins_a;

            if length_a == 1 {
              break 'merge MergeEnd::CopyLast;
            }
            // A comparator that is not a consistent order can leave A empty.
            if length_a == 0 {
              break 'merge MergeEnd::Succeed;
            }
          }
          self.work[dest] = self.work[cursor_b];
          dest += 1;
          cursor_b += 1;
          length_b -= 1;
          if length_b == 0 {
            break 'merge MergeEnd::Succeed;
          }

          nof_wins_b =
            self.gallop_left(Source::Work, self.temp[cursor_temp], cursor_b, length_b, 0)?;
          if nof_wins_b > 0 {
            self.work.copy_within(cursor_b..cursor_b + nof_wins_b, dest);
            dest += nof_wins_b;
            cursor_b += nof_wins_b;
            length_b -= nof_wins_b;

            if length_b == 0 {
              break 'merge MergeEnd::Succeed;
            }
          }
          self.work[dest] = self.temp[cursor_temp];
          dest += 1;
          cursor_temp += 1;
          length_a -= 1;
          if length_a == 1 {
            break 'merge MergeEnd::CopyLast;
          }
        }

        // A penalty for leaving galloping mode.
        min_gallop += 1;
        self.min_gallop = min_gallop;
      }
    };

    match end {
      MergeEnd::Succeed => {
        self.work[dest..dest + length_a]
          .copy_from_slice(&self.temp[cursor_temp..cursor_temp + length_a]);
      },
      MergeEnd::CopyLast => {
        // The last element of A goes after what is left of B.
        self.work.copy_within(cursor_b..cursor_b + length_b, dest);
        self.work[dest + length_b] = self.temp[cursor_temp];
      },
    }

    Ok(())
  }

  /// `MergeHigh`: merges two adjacent runs, the second shorter than the
  /// first, right to left, with the second run copied out of the way. A
  /// cursor can move one place before its run, so the cursors are signed.
  fn merge_high(
    &mut self,
    base_a: usize,
    length_a: usize,
    base_b: usize,
    length_b: usize,
  ) -> Result<(), E> {
    let (mut length_a, mut length_b) = (length_a as isize, length_b as isize);
    let base_a = base_a as isize;
    let at = |index: isize| index as usize;

    self.temp.clear();
    self
      .temp
      .extend_from_slice(&self.work[base_b..base_b + at(length_b)]);

    let mut dest = base_b as isize + length_b - 1;
    let mut cursor_temp = length_b - 1;
    let mut cursor_a = base_a + length_a - 1;

    self.work[at(dest)] = self.work[at(cursor_a)];
    dest -= 1;
    cursor_a -= 1;

    let end = 'merge: {
      // V8 goes to `Succeed` here when A is now empty. A was longer than B,
      // so it still holds at least one element.
      length_a -= 1;
      if length_b == 1 {
        break 'merge MergeEnd::CopyLast;
      }

      let mut min_gallop = self.min_gallop;
      loop {
        let mut nof_wins_a = 0;
        let mut nof_wins_b = 0;

        loop {
          let order = self.compare(self.temp[at(cursor_temp)], self.work[at(cursor_a)])?;

          if order < 0.0 {
            self.work[at(dest)] = self.work[at(cursor_a)];
            dest -= 1;
            cursor_a -= 1;
            nof_wins_a += 1;
            length_a -= 1;
            nof_wins_b = 0;

            if length_a == 0 {
              break 'merge MergeEnd::Succeed;
            }
            if nof_wins_a >= min_gallop {
              break;
            }
          } else {
            self.work[at(dest)] = self.temp[at(cursor_temp)];
            dest -= 1;
            cursor_temp -= 1;
            nof_wins_b += 1;
            length_b -= 1;
            nof_wins_a = 0;

            if length_b == 1 {
              break 'merge MergeEnd::CopyLast;
            }
            if nof_wins_b >= min_gallop {
              break;
            }
          }
        }

        min_gallop += 1;
        let mut first_iteration = true;
        while nof_wins_a >= MIN_GALLOP_WINS || nof_wins_b >= MIN_GALLOP_WINS || first_iteration {
          first_iteration = false;

          min_gallop = max(1, min_gallop - 1);
          self.min_gallop = min_gallop;

          let k = self.gallop_right(
            Source::Work,
            self.temp[at(cursor_temp)],
            at(base_a),
            at(length_a),
            at(length_a - 1),
          )?;
          nof_wins_a = at(length_a) - k;

          if nof_wins_a > 0 {
            let wins = nof_wins_a as isize;
            dest -= wins;
            cursor_a -= wins;
            self
              .work
              .copy_within(at(cursor_a + 1)..at(cursor_a + 1 + wins), at(dest + 1));

            length_a -= wins;
            if length_a == 0 {
              break 'merge MergeEnd::Succeed;
            }
          }
          self.work[at(dest)] = self.temp[at(cursor_temp)];
          dest -= 1;
          cursor_temp -= 1;
          length_b -= 1;
          if length_b == 1 {
            break 'merge MergeEnd::CopyLast;
          }

          let k = self.gallop_left(
            Source::Temp,
            self.work[at(cursor_a)],
            0,
            at(length_b),
            at(length_b - 1),
          )?;
          nof_wins_b = at(length_b) - k;

          if nof_wins_b > 0 {
            let wins = nof_wins_b as isize;
            dest -= wins;
            cursor_temp -= wins;
            self.work[at(dest + 1)..at(dest + 1 + wins)]
              .copy_from_slice(&self.temp[at(cursor_temp + 1)..at(cursor_temp + 1 + wins)]);

            length_b -= wins;
            if length_b == 1 {
              break 'merge MergeEnd::CopyLast;
            }
            // A comparator that is not a consistent order can leave B empty.
            if length_b == 0 {
              break 'merge MergeEnd::Succeed;
            }
          }
          self.work[at(dest)] = self.work[at(cursor_a)];
          dest -= 1;
          cursor_a -= 1;
          length_a -= 1;
          if length_a == 0 {
            break 'merge MergeEnd::Succeed;
          }
        }

        min_gallop += 1;
        self.min_gallop = min_gallop;
      }
    };

    match end {
      MergeEnd::Succeed => {
        if length_b > 0 {
          let start = at(dest - (length_b - 1));
          self.work[start..start + at(length_b)].copy_from_slice(&self.temp[..at(length_b)]);
        }
      },
      MergeEnd::CopyLast => {
        // The first element of B goes before what is left of A.
        dest -= length_a;
        cursor_a -= length_a;
        self
          .work
          .copy_within(at(cursor_a + 1)..at(cursor_a + 1 + length_a), at(dest + 1));
        self.work[at(dest)] = self.temp[at(cursor_temp)];
      },
    }

    Ok(())
  }
}

#[cfg(test)]
#[path = "tests/array_sort_tests.rs"]
mod tests;
