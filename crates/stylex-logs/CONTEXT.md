# stylex-logs

The `log` facade's backend for the NAPI-RS compiler: one global initialization
and one record formatter. What it prints crosses into a Node process's stderr,
so the format is an interface, not a detail.

## Language

**Initialization**:
`initialize()` — installs the logger process-wide, once. It is idempotent
because the compiler is called per file and a Node host may load the addon from
several workers.
_Avoid_: setup, install, configure

**Record**:
One `log::Record` as it arrives at the formatter. The formatter is the only
place level, colour and the `[StyleX]` brand are decided, so a message's shape
never varies by call site.
_Avoid_: entry, line, message
