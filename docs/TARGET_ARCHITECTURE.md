# Target Architecture

This document describes the **intended architecture** for `sts2-fast-sim`.

It is aspirational: parts of this design are not implemented yet.

For the architecture that exists in the repository today, see [CURRENT_ARCHITECTURE.md](CURRENT_ARCHITECTURE.md).

## Purpose

`sts2-fast-sim` should be the high-performance simulation engine used for search, planning, and rollout.

It is deliberately separate from the Spirescry oracle.

The responsibilities are:

- **Fast simulator**: cheap state cloning, deterministic transitions, search/rollout throughput.
- **Spirescry oracle**: authoritative execution of the real STS2 model, canonical inspection, replay, and differential validation.

The intended system is:

```text
                   search / MCTS / rollout
                            |
                       compact Action
                            v
                    +----------------+
                    | sts2-fast-sim  |
                    |   Rust core    |
                    +-------+--------+
                            |
                     canonical export
                            |
               +------------+-------------+
               |                          |
               v                          v
        production rollout          differential test
                                           ^
                                           |
                                  +--------+---------+
                                  | Spirescry oracle |
                                  |   real STS2      |
                                  +------------------+
```

The simulator and oracle should share a stable canonical contract, not a shared internal object model.

## Design Goals

The target simulator should provide all of the following.

### Pure transitions at decision boundaries

The semantic interface is:

```text
(state, player decision) -> next decision state
```

The simulator consumes one player decision, automatically drains deterministic/internal execution, then stops at:

- the next player decision; or
- terminal combat state.

No continuation-critical state may exist only on the Rust call stack or in global mutable state.

### Cheap fork / clone

Search must be able to copy a state cheaply.

The hot path should avoid:

- heap allocation;
- pointer-rich object graphs;
- hidden shared mutability;
- async continuations;
- runtime reflection.

### Exact gameplay semantics in exact mode

Exact mode should match STS2 for:

- RNG stream identity and consumption;
- action legality;
- effect ordering;
- hook ordering;
- targeting;
- player selections;
- mutable content state;
- numeric semantics.

Unsupported exact behavior must fail closed.

### Stable external identity

Oracle-facing actions and state should use stable:

- card instance IDs;
- combat entity IDs;
- potion instance IDs where needed;

rather than transient hand/enemy array positions.

### Oracle-driven correctness

Correctness should be defined by differential agreement with authoritative STS2 execution through Spirescry.

Local tests are necessary, but not sufficient.

### Search-oriented internals

The internal state representation should remain optimized for simulation:

- dense arrays;
- integer IDs;
- static definitions;
- compact opcodes;
- fixed-capacity storage where practical.

There is no requirement for the Rust state to resemble the C# game object graph.

### Inspectable divergence

A conformance failure should identify the earliest semantic mismatch, not merely a different final HP total.

## Existing Foundation to Preserve

The target architecture is an evolution of the current core, not a ground-up rewrite.

The following current choices should remain unless measurement proves otherwise:

- flat `State: Copy` representation;
- compact `CardInst` values;
- indexed piles;
- centralized `legal_actions()`;
- static content tables;
- `Op / TOp / EOp / Hook` rule representation;
- centralized integer damage pipeline;
- explicit enemy move/history state.

The main work is to upgrade the execution and correctness contract around those assets.

## Two Public Interfaces

The simulator should expose two API layers over one rule engine.

### Search API

Optimized for the hot path:

```rust
pub fn legal_actions(
    state: &State,
    out: &mut [Action],
) -> usize;

pub fn step_legal(
    state: State,
    action: Action,
) -> State;
```

Properties:

- compact positional IDs are allowed;
- actions are expected to come from `legal_actions()`;
- minimal validation overhead;
- allocation-free;
- suitable for high-volume search.

### Canonical / conformance API

Strict and oracle-facing:

```rust
pub fn try_step(
    state: State,
    action: CanonicalAction,
) -> Result<State, SimFault>;

pub fn export_canonical(
    state: &State,
) -> CanonicalCombatState;
```

Properties:

- stable instance identities;
- checked legality;
- explicit failures;
- canonical RNG state;
- canonical decision state;
- suitable for differential testing.

Both APIs must execute the same underlying game-rule engine.

## Canonical Action Model

The internal action format can remain compact:

```rust
enum Action {
    PlayCard { hand: u8, target: u8 },
    UsePotion { slot: u8, target: u8 },
    EndTurn,
    Choose { index: u8 },
}
```

The canonical boundary should use stable identities:

```rust
enum CanonicalAction {
    PlayCard {
        card: CardInstanceId,
        target: Option<CombatEntityId>,
    },
    UsePotion {
        potion: PotionInstanceId,
        target: Option<CombatEntityId>,
    },
    EndTurn,
    SelectCards {
        cards: CardSelection,
    },
    SelectOption {
        option: u16,
    },
}
```

A boundary resolver maps canonical identities to the compact internal representation.

Transient positions must not become protocol identity.

## General Decision Boundaries and Continuations

The target rule is:

> Whenever authoritative STS2 execution reaches a player-controlled decision, automatic execution stops and the complete continuation is represented in `State`.

This must work even when the choice occurs during:

- card resolution;
- a triggered effect;
- an enemy move;
- enemy-turn processing;
- other automatic combat execution.

A real player decision must never be replaced by hidden simulator policy in exact mode.

### Proposed execution state

A practical fixed-capacity design is:

```rust
struct ExecutionState {
    decision: Decision,
    frames: FrameStack,
    effects: EffectStack,
}
```

Possible decision variants:

```rust
enum Decision {
    None,
    MainCombat,
    SelectCards {
        source: Pile,
        min: u8,
        max: u8,
        candidates: CardSet,
    },
    SelectTarget {
        candidates: CreatureSet,
    },
    SelectOption {
        options: OptionSet,
    },
}
```

Possible continuation frames:

```rust
enum Frame {
    Card {
        card: CardIx,
        op_index: u8,
        target: CreatureIx,
    },
    EnemyMove {
        enemy: u8,
        op_index: u8,
    },
    Hook {
        hook: Hook,
        rule_index: u16,
    },
}
```

The exact representation may evolve, but all continuation-critical state must be cloneable with `State`.

## Explicit Effect Execution

Rule execution should not depend on recursive Rust calls as semantic state.

The existing rule IR should evolve into an explicit work stack.

For example:

```rust
enum WorkItem {
    CardOp(CardOpFrame),
    Trigger {
        hook: Hook,
        owner: EntityId,
        context: TriggerContext,
    },
    EnemyOp(EnemyOpFrame),
}
```

Execution becomes:

```text
while work remains:
    pop next work item
    execute it
    enqueue resulting work
    if a player decision opens:
        stop and return state
```

Benefits:

- resumable execution;
- no semantic dependence on native recursion;
- exact mid-resolution choices;
- deterministic continuation;
- better tracing;
- better divergence diagnosis.

A large work-item budget may still protect against accidental infinite loops, but exceeding it must return an explicit fault rather than silently truncate execution.

## Exact STS2 RNG

Exact mode must implement the authoritative STS2 RNG contract.

### Generator

STS2 uses `MegaRandom`, based on xoshiro256**.

SplitMix64 is only used to initialize the four xoshiro state words.

The simulator should reproduce:

- xoshiro256** state transitions;
- `NextInt`;
- `NextUnsignedInt`;
- `NextBool`;
- `NextFloat`;
- `NextDouble`;
- shuffle behavior;
- counter increment timing.

Modulo-based bounded selection is not equivalent unless the corresponding STS2 method uses it.

### Run RNG streams

The run-level streams should match STS2:

```text
UpFront
Shuffle
UnknownMapPoint
CombatCardGeneration
CombatPotionGeneration
CombatCardSelection
CombatEnergyCosts
CombatTargets
MonsterAi
Niche
CombatOrbs
TreasureRoomRelics
```

Seed derivation and stream-name derivation should match STS2.

Player RNG streams should also be modelled where they affect the simulator contract.

### RNG counters are state

RNG counters are first-class semantic state.

Two states with identical visible combat values but different RNG counters are different canonical states.

## Canonical State

The internal `State` should remain optimized for fast simulation.

Add a stable projection instead of reshaping internal storage to match C# objects.

For example:

```rust
struct CanonicalCombatState {
    round: u32,
    side: Side,

    player: CanonicalPlayer,
    enemies: CanonicalEnemies,

    draw: CanonicalPile,
    hand: CanonicalPile,
    discard: CanonicalPile,
    exhaust: CanonicalPile,

    run_rng: CanonicalRunRng,
    player_rng: CanonicalPlayerRng,

    decision: CanonicalDecision,
    execution: CanonicalExecutionSummary,
}
```

Canonical export must include every mutable value that can affect future simulation, including hidden/internal state when required for divergence detection.

The canonical schema should be versioned independently from internal state layout.

## Power and Mutable Model State

The dense amount array should remain for common amount-only statuses.

Not every STS2 power can be represented as:

```text
power kind -> amount
```

Some mechanics require:

- source/provenance;
- per-instance auxiliary state;
- mutable saved properties;
- applier identity;
- cleanup tied to another entity.

Do not replace all dense status access with a general hash map.

Prefer a hybrid:

1. dense amount-only status array;
2. dedicated packed fields for common auxiliary state;
3. small fixed-capacity instance storage for powers that genuinely need provenance.

For example:

```rust
struct PowerInstance {
    id: PowerId,
    amount: i16,
    source: EntityId,
    aux: i16,
}
```

Only state that can affect future transitions needs to be preserved.

## Exact vs Approximate Modes

Exact and approximate execution must never be silently mixed.

A possible explicit mode boundary is:

```rust
enum SimulationMode {
    Exact,
    Approximate,
}
```

### Exact mode

If a mechanic cannot be simulated faithfully:

```text
return Unsupported / SimFault
```

Exact mode must not:

- partially translate a card;
- ignore an unknown effect;
- invent a conservative result;
- auto-select a player choice;
- silently truncate a trigger chain.

### Approximate mode

Approximate execution may remain useful for:

- search experiments;
- heuristic evaluation;
- temporarily unsupported content;
- injected enemy-damage workflows.

Approximate behavior must be explicitly selected, clearly marked, and excluded from conformance claims.

## Unsupported Content Policy

Content support must fail closed in exact mode.

If a card has one known clause and one unsupported clause, it is not exactly supported.

Prefer:

```text
Unsupported(CardId, reason)
```

over:

```text
execute known half and ignore unknown half
```

The existing code/comments that describe behavior as approximate, conservative, unknown, unmodelled, or unverified should be audited systematically.

## Content Generation

Keep the static-table runtime design, but generate mechanically extractable metadata from the decompiled STS2 source where practical.

Good code-generation candidates include:

- model IDs;
- card costs/types;
- targeting metadata;
- simple keywords;
- enemy HP constants;
- move metadata;
- relic/potion IDs;
- ascension constants.

Complex gameplay semantics should remain reviewed handwritten overrides when they cannot be safely derived.

Do not attempt a general C#-to-Rust gameplay transpiler.

Generated content must still obey fail-closed exactness rules.

## Conformance Architecture

Correctness should be validated against Spirescry.

The workflow is:

```text
test case / replay prefix
          |
   +------+------+
   |             |
   v             v
simulator      oracle
   |             |
   v             v
canonical     canonical
state         state
   \             /
    +-----diff---+
```

At each decision boundary compare, in priority order:

1. decision kind;
2. legal action set;
3. RNG seeds/counters;
4. pile contents and exact order;
5. HP/block/energy and other creature state;
6. powers and mutable per-instance state;
7. enemy move/history state;
8. continuation/execution state;
9. terminal result.

### Hash first, dump on divergence

For throughput:

- compare canonical digests after each transition;
- request full dumps only after a mismatch.

### Replay-oriented scenarios

Prefer:

- seed/scenario + action history; or
- checkpoint + action history.

Avoid making arbitrary hand-constructed game-engine object graphs the primary oracle input because they can violate hidden STS2 invariants.

## Differential Fuzzing

Once the canonical contract is stable, generate or consume random legal traces and run them against both implementations.

On mismatch:

1. record the first differing decision boundary;
2. request full canonical dumps;
3. rerun with semantic tracing;
4. minimize the action prefix/scenario when practical;
5. commit a focused regression case before fixing the simulator.

Each discovered divergence should become a permanent test.

## Semantic Trace

The simulator should provide an optional trace facility that is compiled out of normal production builds.

Example:

```text
ACTION      PlayCard card=#17 target=#2
OP          Damage base=8 hits=1
HOOK        BeforeDamage
RNG         CombatTargets counter 17 -> 18, value=...
STATE       enemy#2.hp 43 -> 31
HOOK        AfterDamage
DECISION    SelectCards candidates=[#4,#8,#11]
```

Tracing must not change simulation behavior.

## Fault Model

Exact external execution should return explicit faults rather than silent no-op fallbacks.

For example:

```rust
enum SimFault {
    IllegalAction,
    InvalidTarget,
    UnknownCard,
    UnsupportedMechanic,
    InvalidContinuation,
    CapacityExceeded,
    WorkBudgetExceeded,
    CanonicalIdentityNotFound,
}
```

The compact trusted search API may skip some checks when operating on actions produced by `legal_actions()`.

## State Size and Performance

State size should be treated as a performance metric, not a semantic requirement.

Exact RNG streams, continuation frames, effect stacks, and provenance may increase `State` beyond the current 4 KiB target.

That is acceptable until profiling proves state-copy cost is a material bottleneck.

The priority order is:

1. exact semantics;
2. differential validation;
3. benchmark;
4. optimize measured bottlenecks.

Hot/cold state splitting, copy-on-write, undo logs, or other advanced state techniques should be introduced only if measurement justifies them.

## Testing Strategy

The target test stack should have three layers.

### Local unit/invariant tests

Examples:

- RNG vectors;
- damage rounding;
- pile operations;
- legality;
- state invariants;
- content-specific regression cases.

### Determinism tests

For identical state and action:

```text
step(state, action) == step(state, action)
```

Necessary, but not sufficient.

### Oracle conformance tests

The primary semantic gate.

CI should contain a representative deterministic corpus of oracle-validated traces.

Larger randomized differential runs may live in a slower/manual/nightly workflow.

## Recommended Migration Order

### Phase 0 — Canonical contract

Add:

- `CanonicalAction`;
- `CanonicalCombatState`;
- stable instance identities;
- checked `try_step`;
- canonical digest/export.

Avoid changing gameplay semantics in this phase.

### Phase 1 — Exact STS2 RNG

Replace the custom solver RNG in exact mode with:

- xoshiro256** `MegaRandom`;
- exact seed derivation;
- exact stream set;
- exact counters;
- exact integer/float/shuffle behavior.

Add direct vector tests against the oracle/game implementation.

### Phase 2 — General continuation

Replace policy-driven mid-resolution decisions with explicit `Decision + FrameStack`.

Use an existing mid-enemy-turn player choice as the first architecture test.

### Phase 3 — Explicit effect stack

Remove recursive hook execution as semantic state.

Replace silent hook-depth truncation with explicit work items and an erroring safety budget.

### Phase 4 — Exactness audit

Audit all known approximations, conservative rules, unknown content, and unmodelled mechanics.

For each:

- implement exact semantics;
- add required state;
- or mark it unsupported in exact mode.

### Phase 5 — Differential fuzzing

Continuously compare simulator and oracle at decision boundaries.

Minimize failures and turn them into regression tests.

### Phase 6 — Performance tuning

After correctness is measurable, benchmark:

- state-copy cost;
- transition cost;
- legal action generation;
- effects per action;
- rollout throughput.

Optimize only measured bottlenecks.

## Architectural Invariants

The following should be long-term project rules:

1. **All continuation-critical state lives in `State`.**
2. **Exact mode never silently approximates unsupported semantics.**
3. **Player decisions are never replaced by hidden simulator policy.**
4. **RNG stream identity and counters are simulation state.**
5. **External protocol uses stable instance identity, not array position.**
6. **Search and conformance APIs share one rule engine.**
7. **Internal optimization must preserve canonical behavior.**
8. **Work-budget and capacity failures are explicit, never silent truncation.**
9. **Canonical state stays stable even when internal layout changes.**
10. **Oracle disagreement creates a reproducible regression case.**

These invariants are more important than preserving a particular state size, file layout, or implementation technique.

## Tracking Issues

The initial implementation work is tracked in:

1. [#1 — Implement exact STS2 RNG streams, counters, and MegaRandom semantics](https://github.com/NoneMore/sts2-fast-sim/issues/1)
2. [#2 — Replace policy-driven mid-resolution choices with a general continuation model](https://github.com/NoneMore/sts2-fast-sim/issues/2)
3. [#3 — Replace recursive hook execution and MAX_HOOK_DEPTH truncation with an explicit effect stack](https://github.com/NoneMore/sts2-fast-sim/issues/3)
4. [#4 — Add canonical state/action APIs with stable instance identities and checked transition errors](https://github.com/NoneMore/sts2-fast-sim/issues/4)
5. [#5 — Build a Spirescry differential-conformance harness and make it the semantic CI gate](https://github.com/NoneMore/sts2-fast-sim/issues/5)
6. [#6 — Audit approximations and make exact mode fail closed on unsupported mechanics](https://github.com/NoneMore/sts2-fast-sim/issues/6)
7. [#7 — Model power provenance and complex mutable power state without abandoning the dense hot path](https://github.com/NoneMore/sts2-fast-sim/issues/7)
