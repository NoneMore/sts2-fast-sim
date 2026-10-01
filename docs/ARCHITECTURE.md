# Architecture

The architecture documentation is split into two views so that implemented behavior is not confused with planned design.

## Current architecture

Read [CURRENT_ARCHITECTURE.md](CURRENT_ARCHITECTURE.md) for the architecture that exists in the repository today.

It describes:

- the current flat `State: Copy` model;
- card instances and fixed-capacity piles;
- the current `Action`, `Pending`, `Op / TOp / EOp / Hook` model;
- current enemy-turn and damage execution;
- the current solver-oriented RNG;
- recursive hook execution and `MAX_HOOK_DEPTH`;
- current testing and known architectural gaps.

Treat this document as descriptive.

## Target architecture

Read [TARGET_ARCHITECTURE.md](TARGET_ARCHITECTURE.md) for the intended exact fast-simulator architecture.

It describes the direction discussed for evolving the current core toward:

- exact STS2 RNG compatibility;
- a general decision/continuation model;
- an explicit effect stack;
- stable canonical actions and state;
- strict exact-vs-approximate separation;
- Spirescry differential conformance;
- semantic tracing and differential fuzzing.

Treat this document as prescriptive: it contains planned behavior and interfaces that may not exist yet.

## Relationship

The intended project relationship is:

```text
Search / MCTS / rollout
          |
          v
+-------------------+
| sts2-fast-sim     |
| fast Rust engine  |
+---------+---------+
          |
          | canonical state/action contract
          v
+-------------------+
| conformance layer |
+---------+---------+
          ^
          |
+---------+---------+
| Spirescry oracle  |
| authoritative STS2|
+-------------------+
```

The fast simulator is a separate implementation optimized for search. Spirescry remains the authoritative oracle used to validate it.

## Issue tracker

The target-architecture work is tracked in GitHub issues #1 through #7. The detailed mapping lives in [TARGET_ARCHITECTURE.md](TARGET_ARCHITECTURE.md#tracking-issues).
