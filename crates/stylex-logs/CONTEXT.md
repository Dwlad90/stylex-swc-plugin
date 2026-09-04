# stylex-logs

The `log` facade's backend for the NAPI-RS compiler: one global initialization
and one record formatter. What it prints crosses into a Node process's stderr,
so the format is an interface, not a detail.

## Language

**Initialization**:
`initialize()` — installs the logger and the process panic hook, once. It is
idempotent because the compiler is called per file and a Node host may load the
addon from several workers. The hook respects the suppression guard in
[stylex-macros](../stylex-macros/CONTEXT.md) and prefixes `[StyleX]` onto a
panic not already carrying it. The default level is `Warn`; `STYLEX_DEBUG`
raises it.
_Avoid_: setup, install, configure

**Record**:
One `log::Record` as it arrives at the formatter, which is the only place level,
colour and the `[StyleX]` brand are decided. Its target column pads to a width
that only grows, so alignment depends on which modules have logged so far.
_Avoid_: entry, line, message
