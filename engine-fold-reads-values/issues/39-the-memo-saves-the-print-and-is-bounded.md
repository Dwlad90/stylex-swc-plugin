# 39 — The memo saves the print, and is bounded

**What to build:** A printed arrow is printed once as well as parsed once, and
the memo holding it does not grow for the life of the process.

**Half the lever is unrealised.** The spec calls the compiled-arrow memo *"the
single largest lever, because printing and parsing dominate a warm fold"*. Today
`print_fold` runs unconditionally — it deep-clones the `CallExpr`, builds a
`Vec<Pat>` cloning default expressions, walks the whole tree with `DropSpan` and
runs the emitter — and only then is the resulting `String` used as the memo key.
So a file with a thousand style objects sharing one expression shape parses once
and prints a thousand times.

The key can be computed without printing: `stable_hash_unspanned` over the call,
plus the carried parameter names, both already to hand and both already
128-bit-stable. Print only on a miss.

**The memo is unbounded.** `FxHashMap<String, Script>`, keyed by printed source
text, never evicted, beside boa's own source interning — roughly half a kilobyte
per distinct site, self-documented. It grows for the life of the process, which
in watch mode is the life of the dev server. A hash key removes the retained
source strings; a size cap or LRU removes the rest.

**One allocation travels with them.** `print_fold` builds a `CodeFrame` per call
— `Handler::with_tty_emitter(ColorConfig::Auto, …)`, a boxed emitter plus
terminal detection — purely to reach the `OnceLock` `SourceMap`. Printing needs
no handler. Once printing happens only on a miss this is rarer, but it is still
per miss and it is still a tty probe inside a compiler.

**Blocked by:** 36.

**Status:** resolved

- [x] The memo is keyed on the unspanned hash of the call plus a hash of the
      carried parameters; printing happens only on a miss
- [x] The memo has a bound -- `MAX_COMPILED_SCRIPTS`, 2048 entries, stated as
      roughly a megabyte of bytecode per thread -- and is emptied on reaching it
- [x] Printing reaches the source map through `shared_source_map`, without a
      handler and without the tty probe
- [x] The existing `fold_memo` tests still pass, and
      `two_positions_of_one_call_share_a_memo_entry` asserts two structurally
      identical calls at different spans share an entry
- [x] The criterion memo group reads as a pair: the `fold` legs no longer pay the
      print, and on a paired local run they moved while the `engine` controls
      beside them did not

**Resolution:** the key is a `FoldKey` of two 128-bit hashes -- the call through
`stable_hash_unspanned_call`, and the printed parameters through
`Transport::parameters_key`, which walks the same list `parameters` prints. The
parameters half is load-bearing: one call shape reached through two differently
declared callbacks prints two sources, pinned by
`one_call_shape_with_two_printed_callbacks_is_two_entries`. Structure separates a
little more than the printed text -- a literal's own spelling survives into the
key -- which costs a parse rather than risking a shared script.
