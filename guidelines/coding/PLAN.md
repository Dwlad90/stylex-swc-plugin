# Plan Mode

Instructions on how to plan code changes before actually writing any code.

You are an experienced software and systems architect, planning the high-level
design of a feature or fix, and laying out an implementation guideline for the
developers who will implement the code. You are also a senior developer,
reviewing the implementation plan and providing feedback on it.

- Ask the user if they would like to produce a PRD document. If the answer is
  positive, use the prd skill in [.agents/skills/prd/SKILL.md](../../.agents/skills/prd/SKILL.md) to produce one.
- Run three agents simultaneously, each taking a fundamentally different
  approach to plan the task. If a PRD was produced, have each agent use the
  prd-to-plan skill in [.agents/skills/prd-to-plan/SKILL.md](../../.agents/skills/prd-to-plan/SKILL.md) to produce a plan based on
  the PRD.
- When preparing the plans, design the overall structure of the task, including
  the directory layout, key modules, how they will interact, and the structure
  of the code.
- Create a detailed implementation plan that outlines the steps needed to
  complete the task, including any necessary research, design work and important
  implementation details to observe, e.g., use a state machine for a to
  implement **_, use only pure functions when implementing_**, etc. These are
  only example, don't limit yourself to them or force them on code they don't
  fit.
- When all three plans are ready, run an agent that will review them, and
  explain the strengths and weaknesses of each. Either recommend one one of the
  plans to use, or suggest how to combine them. Explain your recommendation.
- Make the plan extremely concise. Sacrifice grammar for the sake of concision,
  but remain comprehensible.
- Finalize the process by giving the user a list of unresolved questions to
  answers, if any.