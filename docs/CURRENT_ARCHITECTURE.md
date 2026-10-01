# Current Architecture

This document describes the architecture that is **implemented in the repository today**.

It is intentionally descriptive rather than aspirational. Planned changes belong in [TARGET_ARCHITECTURE.md](TARGET_ARCHITECTURE.md).

## Scope

The repository currently contains a compact Rust combat simulation core extracted from the original `sts2core` project.

The current core is optimized for:

- cheap combat-state copies;
- deterministic solver/search execution;
- allocation-free hot paths;
- static rule tables;
- compact card/enemy/power state.

It was originally designed for a combat solver/advisor, not for exact replay compatibility with STS2. Several current behaviors are therefore intentionally solver-oriented rather than oracle-exact.

## Core Files

The combat core is concentrated in:

```text
src/
  state.rs
  step.rs
  ops.rs
  damage.rs
  content.rs
  asc.rs
  lib.rs
```

Broadly:

- `state.rs`: compact mutable combat state and pile/card/entity storage;
- `step.rs`: actions, legality, turn flow, card play, enemy execution, hook dispatch;
- `ops.rs`: rule opcodes and static rule-definition types;
- `damage.rs`: damage/block/status arithmetic;
- `content.rs`: cards, powers, relics, potions, enemies, and rule tables;
- `asc.rs`: ascension adjustments;
- `lib.rs`: crate exports.

## State Model

The current simulator uses a flat, fixed-capacity state representation.

Important properties:

- `State` is `Copy`;
- piles are fixed arrays of compact card indexes;
- enemies live in fixed arrays;
- card instances are compact values rather than heap objects;
- the combat core does not rely on `Vec`, `Box`, or `HashMap` in the hot state/step path;
- cloning/forking a state is a value copy.

The project currently treats roughly 4 KiB as a desirable upper bound for `State`, and the smoke tests enforce that size target.

That size limit is currently a performance invariant of the extracted core, not a statement about STS2 semantics.

## Card Instances and Piles

Cards are split into:

- static definitions in the content tables;
- mutable per-combat `CardInst` values in `State`.

Piles store compact indexes into the card-instance array rather than object references.

Conceptually:

```text
cards[card_index] -> CardInst

hand    = [card_index, ...]
draw    = [card_index, ...]
discard = [card_index, ...]
exhaust = [card_index, ...]
```

This gives the current core cheap copies and stable internal references while avoiding a pointer-rich object graph.

## Entity and Power Representation

Players and enemies use compact `Entity` values.

Most powers/statuses are represented as a dense indexed amount array:

```text
status[StatusId] -> i16 amount
```

This is fast for amount-only statuses and allows direct indexed access.

The current representation does not generically preserve power-instance provenance or arbitrary per-instance mutable power state. Mechanics that need more than an amount are handled with dedicated state fields, special rules, approximation, or are not fully modelled.

## Action Model

The current public combat action type is compact and position-oriented:

```rust
pub enum Action {
    PlayCard { hand: u8, target: u8 },
    UsePotion { slot: u8, target: u8 },
    EndTurn,
    Choose { hand: u8 },
}
```

The indexes refer to the current compact state layout.

This is efficient for search, but these positions are not stable external identities suitable for a long-lived oracle protocol.

## Legal Actions

`legal_actions()` is implemented inside the simulation core.

It is responsible for exposing actions that are legal in the current state, including:

- playable cards;
- valid targets;
- potion actions;
- end turn;
- active card-selection choices.

This keeps search code from reimplementing core legality rules.

The implementation also performs duplicate-choice pruning for equivalent card instances in some selection states to reduce search branching.

## Transition Model

The central transition API is currently:

```rust
pub fn step(s: State, a: Action) -> State
```

The function consumes a copied state and returns the resulting copied state.

Illegal actions currently return the original state unchanged.

This is convenient for solver use, but it does not distinguish:

- illegal external input;
- unsupported behavior;
- invalid target;
- unknown identity;
- a legitimate no-op.

The current exactness work tracks replacing this behavior at the canonical/conformance boundary while retaining a compact trusted search path.

## Pending Player Choices

The simulator currently has an explicit `Pending` enum for a limited family of card-selection decisions.

Current variants include cases such as:

- exhaust cards from hand;
- fetch cards from discard;
- move a discard card to the top of the draw pile;
- put hand cards on top of the draw pile;
- upgrade cards in hand.

While a `Pending` selection is active, `legal_actions()` exposes only the associated `Choose` actions.

The core also contains `close_pending_if_stuck()`, which prevents unsatisfiable pending selections from leaving the state permanently unable to advance.

This is an important existing safety property.

## Mid-Resolution Choices

`Pending` is not currently a general continuation mechanism.

Some real player decisions that occur in the middle of automatic execution are represented as solver policy instead of a paused decision.

The clearest current example is `State::curse_policy`.

A choice that occurs during an enemy turn is resolved from this policy field because `EndTurn` is treated as an atomic simulator action.

Therefore the current simulator does **not** yet satisfy the stronger rule:

> every real player decision is a simulator decision boundary.

This gap is tracked in issue #2.

## Rule Representation

The project already uses a compact data-oriented rule representation.

The main rule families are:

- `Op`: card and potion operations;
- `TOp`: triggered/power operations;
- `EOp`: enemy-move operations;
- `Hook`: trigger timing;
- static content definition tables.

This is one of the strongest parts of the current architecture.

Rules are primarily represented as static data plus centralized interpreters rather than as one heap-allocated polymorphic object per card, power, relic, or monster.

## Hooks and Trigger Execution

Hooks are dispatched by centralized `fire` / `fire_ctx` logic.

Nested hooks currently execute through ordinary Rust function calls.

To protect the solver from recursive trigger loops, the code uses a hard nesting guard:

```rust
MAX_HOOK_DEPTH = 4
```

If the depth is exceeded, further hook execution is dropped.

This guard fixed real recursive loops in the original solver, but it is not exact gameplay semantics because a valid deeper chain would also be truncated.

This limitation is tracked in issue #3.

## Damage Pipeline

The current damage implementation is centralized and heavily rule-driven.

A notable design choice is that gameplay multipliers are handled with integer numerator/denominator arithmetic and explicit rounding points rather than general floating-point accumulation.

The current implementation contains detailed ordering rules for mechanics such as:

- strength-like additive modifiers;
- weak;
- vulnerable;
- slow;
- damage caps;
- intangible-like mechanics;
- block absorption;
- powered vs unpowered damage.

This pipeline is worth preserving and validating rather than replacing wholesale.

## Enemy State and AI

Enemy combat state is stored explicitly in `State`.

This includes compact data such as:

- enemy definition IDs;
- current move state;
- recent move history;
- a bitset of moves already used;
- enemy-specific statuses and mutable combat values.

Enemy definitions may use either deterministic move sequences or an explicit move machine.

The current core already models a significant amount of monster state rather than deriving enemy behavior only from visible intent labels.

## Enemy-Turn Injection Helpers

The current core contains helper paths such as:

- `end_turn_with_incoming`;
- `end_turn_with_live_incoming`.

These allow an upper layer to supply enemy incoming damage while reusing the core damage/block/trigger pipeline.

They were useful for the original solver and verification workflows.

They are not, by themselves, the authoritative exact enemy execution path because they can bypass parts of enemy move semantics.

## Current RNG Model

The current simulator RNG is intentionally compact and solver-oriented.

It uses three logical streams:

- shuffle;
- enemy;
- generation.

The current generator is based on SplitMix64-style state advancement, and bounded selection uses simulator-specific logic such as modulo reduction.

This is **not** the RNG contract used by STS2.

The game uses named RNG streams backed by `MegaRandom` / xoshiro256**, with stream-specific seed derivation and counters.

Therefore the current RNG is deterministic, but not replay-compatible with the authoritative game RNG.

This gap is tracked in issue #1.

## Exactness Status

The current core contains a mixture of:

- rules reconstructed from decompiled source;
- rules validated from historical gameplay traces;
- solver-oriented approximations;
- conservative fallbacks;
- unknown/unmodelled content;
- behavior that is explicitly documented as not fully verified.

The project currently does not have a single runtime boundary separating exact and approximate semantics.

This is tracked in issue #6.

## Current Testing

After extracting the simulation core from the larger parent project, the current fork has a small smoke suite.

At the time this document was split out, `tests/core_smoke.rs` contains four tests covering basic core contracts such as determinism/copy/size behavior.

CI currently runs:

```text
cargo build --release --lib
cargo test --release --tests
```

The much larger advisor/trace/rollout verification stack from the parent project was intentionally removed with the non-core layers.

The intended replacement for semantic correctness is a new Spirescry-based differential conformance harness, tracked in issue #5.

## Current Strengths

The existing implementation already satisfies several important fast-simulator requirements:

- Rust implementation;
- compact flat state;
- cheap value cloning;
- no async execution model;
- no UI/network dependency;
- no pointer-rich game object graph;
- static content/rule tables;
- centralized legality;
- explicit enemy mutable state;
- explicit handling for several card-selection sub-decisions;
- allocation-light hot path;
- centralized damage semantics.

These are foundations to preserve.

## Current Architectural Gaps

The main gaps between the implemented core and the intended exact fast simulator are:

1. RNG semantics do not match STS2 exactly — issue #1.
2. Player decisions are not universally represented as decision boundaries — issue #2.
3. Hook execution depends on recursive calls and silent depth truncation — issue #3.
4. There is no stable canonical action/state protocol or checked conformance API — issue #4.
5. There is no Spirescry differential harness in the extracted fork — issue #5.
6. Exact and approximate semantics are not explicitly separated — issue #6.
7. Dense status amounts cannot represent every source-dependent or per-instance power mechanic — issue #7.

The target design and migration plan are documented in [TARGET_ARCHITECTURE.md](TARGET_ARCHITECTURE.md).
