# Testing Workflow

Follow these steps in order when testing code for a task:

- Use the tdd skill in [.agents/skills/tdd/SKILL.md](../../.agents/skills/tdd/SKILL.md) to test and implement the
  code and feature in vertical slices. Closely follow the implementation plan.
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
- Run `pnpm test` before considering any task complete.