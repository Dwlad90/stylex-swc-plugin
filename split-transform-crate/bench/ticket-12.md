# Ticket 12 — bench A/B

`stylex-declarations` extracted from `stylex-state` and `stylex-transform`.

## Two comparisons, and why the second one is the answer

Ticket 12 asks for an A/B against `develop`. That was run first, and it is
**not** an attribution of this ticket: the branch already carries tickets 01
through 08, two of which are themselves crate splits. A diff against `develop`
measures nine commits at once.

So a second A/B was run against **the parent commit `ba960bcef`**, which
isolates this ticket. That is the number below. The `develop` comparison is
kept in [`ticket-12-ab.log`](./ticket-12-ab.log) for the record.

**A shared `CARGO_TARGET_DIR` across worktrees corrupts this measurement.**
Cargo served one worktree's `stylex_state` rlib to another worktree's build; it
surfaced as `error[E0624]: method 'import_binding' is private` when the branch
was built after the parent in the same directory. A first parent baseline was
taken in that shared directory and had to be **discarded**: it read
`fold-distinct` as −1.18% where a clean build reads +4.43%. Every leg below
uses its own target directory, and the criterion baselines are copied between
them rather than shared.

## Method

| Item     | Value                                                             |
| -------- | ----------------------------------------------------------------- |
| Machine  | Apple M1 Max, 10 cores, 64 GB, macOS 26.6.1                       |
| Profile  | `bench` (`lto = true`, `debug = true`)                            |
| Settings | `--sample-size 20 --warm-up-time 2 --measurement-time 4 --noplot` |
| Baseline | `ba960bcef`, own target dir, saved as `parent-clean`              |
| Test     | this ticket, own target dir, `--baseline parent-clean`            |
| Benches  | the four the moved code can reach; 44 measurements                |

Logs: [`ticket-12-parent-leg.log`](./ticket-12-parent-leg.log) and
[`ticket-12-vs-parent.log`](./ticket-12-vs-parent.log).

## Result

| Measure      | Value                  |
| ------------ | ---------------------- |
| Measurements | 44                     |
| Median       | **+3.04%**             |
| Range        | −4.61% to +6.76%       |
| Faster       | 7 of 44                |
| Within ±4%   | 33 of 44               |
| Above +4%    | 10 of 44, worst +6.76% |

The median sits below the **+3.65%** floor ticket 07 measured on a bench whose
crate was byte-identical between legs. Ten measurements are above +4%, and that
is stated rather than smoothed over.

> **[Ticket 16](./ticket-16.md) prices that floor: it was the method, not
> layout and not the `cdylib`.** A control -- the baseline commit rebuilt and
> measured against its own saved baseline -- reads a median of −2.0% to −3.0%
> and single measurements as far as +12.85%, on code that did not change. This
> +3.04% median and the ten measurements above +4% are inside that floor, so the
> A/B neither confirms nor denies a cost. The reasoning below still holds: no
> executable line of the moved code changed, and the bench that cannot reach it
> moved more than the one that can.

## Why this is layout and not the moved code

`ModuleWalk/no-calls/1x` moved **+4.05%**. `ModuleWalk/calls/1x` moved
**+3.41%**. The no-calls bench compiles a module with no `stylex.*` call in it,
so no declaration lookup and no convertor in the new crate can run on that path
at all.

A cost introduced by the move would land on the benches that call the moved
functions and leave the others flat. The measurement is the opposite: the bench
that **cannot** reach the moved code shifted **more** than the one that does.
That rules the moved functions out as the cause, and leaves function placement
under `-C lto -C codegen-units=1`, which a new crate changes even where no
executable line did.

Two supporting facts. Every one of the nine moved function bodies is
byte-identical to its pre-move form -- only import lines and two visibilities
differ. And `convert_ident_to_expr` kept the `#[inline]` it carried before, so
the one moved function that was marked for inlining still is.
