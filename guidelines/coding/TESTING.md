# Testing Workflow

Follow these steps in order when testing code for a task:

- Use the [tdd skill](../../.agents/skills/tdd/SKILL.md) to test and implement
  the code and feature in vertical slices. Closely follow the implementation
  plan.
- Once finished implementing the task with passing tests, ensure the strategy is
  comprehensive and effectively validates the functionality of the code.
- Provide feedback on any gaps or improvements that can be made to the testing
  approach.
- Use descriptive test names that clearly state the expected behavior (e.g.,
  "should return user data when given a valid ID").
- Aim for high test coverage, especially for critical business logic and edge
  cases.
- Use test doubles (mocks, stubs, spies) to isolate the unit under test and
  verify interactions with dependencies.
- If improvements are requested by the user, implement them using the steps
  above.
- Run `pnpm test` before considering any task complete. It runs
  `turbo run test --continue` over the JavaScript suites. A crate prints a skip
  line for its own `test`.
- Run `pnpm test:crates:workspace` for the Rust half. It runs nextest and doc
  tests over the whole workspace, as the `tests-rust` and `tests-rust-doc` legs
  of `pr-validation` do.

## Testing across the NAPI boundary

The JS suites import `@stylexswc/rs-compiler`, which loads the prebuilt
`crates/stylex-rs-compiler/dist/*.node`. They do not compile Rust.

- After changing any crate, run `pnpm run --filter=@stylexswc/rs-compiler build`
  before the JS suite. Skip it and the tests silently exercise the previous
  binary -- they pass or fail for reasons unrelated to the edit.
- Put logic that can be tested without the boundary behind a plain function and
  cover it with `cargo nextest` in `src/tests/`; those tests are far faster and
  pin down edge cases (empty input, ambiguity, malformed data) precisely.
- Keep the JS specs for what only they can prove: that a value survives
  serialization across the boundary and reaches the emitted artifact.
