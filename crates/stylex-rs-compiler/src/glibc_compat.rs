//! Keeps the linux-gnu addon loadable on glibc 2.34.
//!
//! glibc 2.35 added a new version of `hypot`, and a link against glibc 2.35 or
//! later records that version as a requirement. A host with an older glibc then
//! refuses to load the addon, and Node reports only "Cannot find native
//! binding". Amazon Linux 2023 (Vercel, AWS CodeBuild) ships glibc 2.34. The
//! JavaScript engine calls `hypot` for `Math.hypot`.
//!
//! Each block below defines a local `hypot` that jumps to the old version. The
//! linker binds every call in the addon to this local symbol, so the binary
//! needs only the old version. In a new glibc, the old version runs the same
//! computation and only sets `errno` differently, so results do not change.
//!
//! The definition, the version binding and the jump are all in one assembly
//! block, so they always go into one object file. A version binding applies
//! only to references in its own object file, and a debug build splits the
//! crate into many. `.hidden` keeps the symbol out of the addon's exports.
//!
//! The blocks have no `endbr64` (x86_64 CET) or `bti c` (aarch64 BTI) landing
//! pad. The addon is not built with CET or BTI, so the loader does not enforce
//! them. If a build turns them on, put the landing pad first in each block,
//! because a call through a function pointer to `hypot` then needs it.
//!
//! `.github/scripts/check-glibc-floor.mjs` fails the release when a symbol
//! needs a glibc newer than 2.34. Add a block here when it names a new one.

// `GLIBC_2.2.5` is the first glibc version on x86_64.
#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
  ".symver __hypot_compat, hypot@GLIBC_2.2.5",
  ".pushsection .text.hypot, \"ax\", @progbits",
  ".globl hypot",
  ".hidden hypot",
  ".type hypot, @function",
  "hypot:",
  "  jmp __hypot_compat@PLT",
  ".size hypot, . - hypot",
  ".popsection",
);

// `GLIBC_2.17` is the first glibc version on aarch64.
#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
  ".symver __hypot_compat, hypot@GLIBC_2.17",
  ".pushsection .text.hypot, \"ax\", @progbits",
  ".globl hypot",
  ".hidden hypot",
  ".type hypot, @function",
  "hypot:",
  "  b __hypot_compat",
  ".size hypot, . - hypot",
  ".popsection",
);

// The tests call `hypot` through the shim. The module exists only where the
// shim is linked.
#[cfg(test)]
#[path = "tests/glibc_compat_tests.rs"]
mod glibc_compat_tests;
