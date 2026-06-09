# Task Index: CLI AXI Support

| Story ID | Story Title | Branch | Dependencies | Parallel-safe | Modules | Status |
| -------- | ----------- | ------ | ------------ | ------------- | ------- | ------ |
| 01-001 | Mode Selection Implementation | feature/current/cli-axi/story-01-001-mode-selection | None | Parallel-safe: true | cli.rs, config.rs | [x] Done |
| 01-002 | TOON Format Implementation | feature/current/cli-axi/story-01-002-toon-format | None | Parallel-safe: true | output.rs, toon.rs (new) | [x] Done |
| 01-003 | Minimal Default Schemas | feature/current/cli-axi/story-01-003-minimal-schemas | None | Parallel-safe: true | cli.rs, output.rs | [x] Done |
| 02-001 | Content Truncation | feature/current/cli-axi/story-02-001-content-truncation | 01-003 | Parallel-safe: true | output.rs | [x] Done |
| 02-002 | Pre-computed Aggregates | feature/current/cli-axi/story-02-002-pre-computed-aggregates | 01-003 | Parallel-safe: true | cli.rs, output.rs | [~] In-Progress |
| 02-003 | Definitive Empty States | feature/current/cli-axi/story-02-003-definitive-empty-states | 01-003 | Parallel-safe: true | cli.rs, output.rs | [ ] Todo |
| 03-001 | Structured Errors & Exit Codes | feature/current/cli-axi/story-03-001-structured-errors | 01-001, 01-002 | Parallel-safe: true | error.rs, main.rs | [ ] Todo |
| 04-001 | Session Hook Infrastructure | feature/current/cli-axi/story-04-001-session-hooks | 01-001, 01-002 | Parallel-safe: true | hooks.rs (new), config.rs | [ ] Todo |
| 04-002 | Installable Agent Skill | feature/current/cli-axi/story-04-002-agent-skill | 04-001 | Parallel-safe: true | skill.rs (new), docs/ | [ ] Todo |
| 05-001 | Content-First No-Args | feature/current/cli-axi/story-05-001-content-first | 01-003, 02-002 | Parallel-safe: true | cli.rs, main.rs | [ ] Todo |
| 05-002 | Contextual Disclosure | feature/current/cli-axi/story-05-002-contextual-disclosure | 01-003, 02-002 | Parallel-safe: true | output.rs, cli.rs | [ ] Todo |
| 06-001 | Integration Testing | feature/current/cli-axi/story-06-001-integration-testing | 01-001, 01-002, 01-003, 02-001, 02-002, 02-003, 03-001, 04-001, 04-002, 05-001, 05-002 | Parallel-safe: false | tests/ | [ ] Todo |
| 06-002 | Documentation Completion | feature/current/cli-axi/story-06-002-documentation | 04-002, 05-001, 05-002 | Parallel-safe: true | docs/, README.md | [ ] Todo |
