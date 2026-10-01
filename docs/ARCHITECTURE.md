# Fast Simulator Architecture

## Purpose

`sts2-fast-sim` is the high-performance combat simulation engine for Slay the Spire 2.

It is **not** an alternate implementation of the Spirescry oracle. The two projects have different responsibilities:

- **Fast simulator**: cheap state cloning, deterministic transitions, search/rollout throughput.
- **Spirescry oracle**: authoritative execution of the real STS2 model, canonical inspection, replay, and differential validation.

The simulator should be optimized aggressively, but only behind a correctness contract that can be validated against the oracle.

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

The simulator and oracle must share a stable **canonical contract**, not a shared internal object model.

---

## Design Goals

The simulator should provide all of the following:

1. **Pure state transitions at decision boundaries**
   - The future of the simulation is completely determined by the simulator state and the next action.
   - No continuation-critical state may live only on the Rust call stack or in global mutable state.

2. **Cheap fork / clone**
   - Search code must be able to copy a combat state cheaply.
   - Heap allocation and pointer-rich object graphs should stay out of the hot path.

3. **Exact gameplay semantics in exact mode**
   - Random number generation, action legality, effect ordering, hook ordering, targeting, selection, and mutable content state must match STS2.
   - Unsupported behavior must fail closed rather than silently approximating.

4. **Stable external identities**
   - Oracle-facing actions and state must use stable card instance IDs and combat entity IDs, not transient hand or enemy array positions.

5. **Oracle-driven conformance**
   - Correctness is defined by agreement with authoritative STS2 execution.
   - Local unit tests remain valuable, but they are not a substitute for differential validation.

6. **Search-oriented internal representation**
   - Internal state layout may be very different from the game implementation.
   - Dense arrays, integer IDs, static tables, and compact opcodes are preferred where they preserve exact behavior.

7. **Inspectable divergence**
   - When the simulator disagrees with the oracle, the system should identify the earliest semantic divergence rather than only showing a final-state mismatch.

---

## Existing Core to Preserve

The current extracted core already has the correct overall shape in several important areas and should be evolved rather than rewritten.

### Flat copyable state

The current state is based on fixed-size arrays and integer indexes, with `State: Copy`.

This is the right foundation for search:

```rust
let child = step(parent, action);
```

State cloning should remain a fixed-cost value operation with no hidden aliasing.

The 4 KiB state-size check is useful as a performance signal, but it is **not** a semantic requirement. Correctness must not be sacrificed to remain below an arbitrary byte threshold.

### Card instances

Per-card mutable state belongs on card instances, not on static card definitions.

The existing split between:

- static card definition, and
- compact `CardInst`

is correct and should remain.

### Static rule IR

The existing rule representation is a major asset:

- `Op` for card/potion effects,
- `TOp` for triggered effects,
- `EOp` for enemy moves,
- `Hook` for trigger timing,
- static content tables.

This is preferable to a trait-object hierarchy where every card, relic, power, and monster is a heap object.

The goal is to turn the existing rule IR into a resumable exact execution VM, not replace it.

### Central legality

`legal_actions()` belongs in the core.

Search code should not independently reproduce:

- energy checks,
- targeting rules,
- playability gates,
- potion legality,
- selection bounds,
- current decision type.

The same underlying legality rules should support both the compact search API and the checked canonical API.

### Integer damage pipeline

The current damage pipeline already avoids floating-point drift by representing multiplicative modifiers with integer numerators and denominators and rounding at defined points.

This should remain the general approach.

---

## Two Public Interfaces

The simulator should expose two layers over one execution engine.

### 1. Search interface

This interface is optimized for the hot path.

Example:

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
- caller is expected to use actions produced by `legal_actions`;
- minimal validation overhead;
- no allocation;
- intended for rollout/search code.

### 2. Canonical / conformance interface

This interface is strict and oracle-facing.

Example:

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
- explicit errors;
- canonical RNG state;
- canonical decision state;
- suitable for differential testing.

Both interfaces must execute the **same rule engine**.

---

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

The oracle-facing format should use stable identities:

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

A boundary resolver converts canonical actions into compact internal actions.

Transient array positions are implementation details and must not become part of the long-lived conformance protocol.

---

## Decision Boundaries and Continuations

The current `Pending` mechanism is a good first step, but it currently models only a limited set of card selection cases.

The exact simulator requires a general rule:

> Whenever real STS2 execution reaches a player-controlled decision, automatic execution stops and the complete continuation is represented in `State`.

This applies even when the decision occurs in the middle of:

- card resolution,
- a triggered effect,
- an enemy move,
- enemy turn processing,
- reward-like combat decisions.

A player decision must never be replaced by an implicit simulator policy.

For example, a mid-enemy-turn choice must not be represented as a `curse_policy` field that automatically chooses an outcome. The simulator must pause and expose that decision.

### Proposed execution state

A practical fixed-size representation is:

```rust
struct ExecutionState {
    decision: Decision,
    frames: FrameStack,
    effects: EffectStack,
}
```

Example decision variants:

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

Example continuation frames:

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

All stacks should remain fixed-capacity unless profiling demonstrates that a different layout is required.

---

## Explicit Effect Execution

The current hook engine uses recursive Rust calls and a hard `MAX_HOOK_DEPTH` guard.

That is acceptable as a temporary solver safeguard, but it is not exact simulator semantics. A legitimate trigger chain deeper than the threshold is silently truncated.

The execution engine should move to an explicit work stack:

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
    if a player decision is opened:
        stop and return the state
```

Benefits:

- no semantic dependence on the Rust call stack;
- resumable execution;
- exact mid-resolution player choices;
- easier tracing;
- easier first-divergence diagnosis;
- no silent trigger-depth truncation.

A very large work-item budget can still protect against accidental infinite loops, but exceeding it must return an explicit `SimFault`, never silently drop effects.

---

## Exact RNG Compatibility

The current three-stream SplitMix64 design is optimized for solver comparability, not STS2 replay compatibility.

Exact mode must implement STS2 RNG semantics.

### Generator

STS2 uses `MegaRandom`, based on **xoshiro256\*\***.

SplitMix64 is used only to initialize the four-word xoshiro state.

The simulator must reproduce:

- xoshiro256** state transitions,
- `NextInt`,
- `NextUnsignedInt`,
- `NextBool`,
- `NextFloat`,
- `NextDouble`,
- shuffle behavior,
- counter increment timing.

Using `random % n` is not equivalent to the game implementation and is not sufficient.

### Run RNG streams

The run-level stream set must match STS2:

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

The simulator should use the same seed derivation and stream-name derivation as STS2.

Player RNG streams should also be represented where they are relevant to the simulator contract.

### Counters are state

RNG counters are first-class canonical state.

A state with identical visible combat values but different RNG counters has already diverged and must compare unequal in conformance testing.

---

## Canonical State

Internal `State` should remain optimized for simulation. It should not be redesigned to look like C# game objects.

Instead, add a stable canonical projection:

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

The canonical state must include every simulation-relevant mutable value necessary to detect divergence, including values not normally visible to a player.

The canonical schema should be versioned independently of the internal `State` layout.

---

## Power and Mutable Model State

The current dense `status: [i16; N_STATUS]` representation is excellent for common amount-only powers, but not every STS2 power can be represented as only:

```text
power kind -> amount
```

Some mechanics require:

- source/provenance,
- per-instance auxiliary state,
- mutable model properties,
- identity of the applier,
- removal behavior tied to another entity.

Do not replace the entire dense representation with a hash map.

Instead use a hybrid model:

1. **Dense amount-only status**
   - strength, weak, vulnerable, artifact, and similar hot fields.

2. **Dedicated packed fields**
   - small frequently used state that does not need full instance identity.

3. **Small fixed power-instance storage**
   - only for mechanics that genuinely require provenance or per-instance mutable data.

Example:

```rust
struct PowerInstance {
    id: PowerId,
    amount: i16,
    source: EntityId,
    aux: i16,
}
```

This keeps common combat operations fast without throwing away information required for exact behavior.

---

## Exact vs Approximate Modes

The simulator must never silently mix exact and approximate behavior.

The core should distinguish at least:

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

Do not:

- partially translate a card;
- ignore an unknown effect;
- invent a conservative result;
- auto-select a player choice;
- silently truncate a trigger chain.

### Approximate mode

Approximate execution may remain useful for search experiments, heuristics, or content that is not yet implemented.

Approximate behavior must be:

- explicitly selected;
- clearly marked;
- excluded from conformance claims.

Interfaces such as injected enemy incoming damage belong in solver-support / approximate infrastructure rather than the authoritative exact transition path.

---

## Unsupported Content Policy

Content compilation must fail closed.

If a card contains one understood clause and one unsupported clause, it is not an exact implementation of that card.

Prefer:

```text
Unsupported(CardId, reason)
```

over:

```text
execute the known half and ignore the rest
```

A simulator that confidently returns the wrong state is more dangerous than one that refuses the case.

The existing comments containing terms such as "approximation", "conservative", "unmodelled", "unknown", and "unverified" should be systematically audited and classified.

---

## Content Generation

The current static-table design should remain, but mechanically extractable data should eventually be generated from the decompiled STS2 source.

Good code-generation candidates include:

- model IDs;
- card costs and types;
- targeting metadata;
- simple keywords;
- enemy HP constants;
- move metadata;
- relic/potion IDs;
- ascension constants.

Complex semantics should remain in reviewed handwritten overrides when they cannot be safely derived.

Do not attempt a general C#-to-Rust gameplay transpiler.

Generated content must still obey the exact/unsupported rule: partial translation is not exact support.

---

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
5. creature HP/block/energy and related combat values;
6. powers and mutable per-instance state;
7. enemy move state/history;
8. continuation/execution state;
9. terminal result.

### Hash first, dump on divergence

For throughput, conformance can use a two-level interface:

- canonical digest after every transition;
- full canonical dump only when the digest differs.

This keeps the oracle useful at high differential-test volume.

### Replay-based scenarios

The default test-case format should prefer:

- initial seed/scenario + action history, or
- checkpoint + action history.

Arbitrary hand-built internal engine object graphs should not be the primary oracle input because they can violate hidden STS2 invariants.

---

## Differential Fuzzing

Once the canonical contract is stable, the simulator should generate or consume random legal traces and run them against both implementations.

On a mismatch:

1. record the first differing decision boundary;
2. request full canonical dumps;
3. rerun both sides with semantic tracing;
4. minimize the action prefix / scenario where possible;
5. commit a focused regression test before fixing the simulator.

The goal is to turn every discovered discrepancy into a small permanent test.

---

## Semantic Trace

The simulator should have an optional trace facility that is compiled out of normal production builds.

Example trace events:

```text
ACTION      PlayCard card=#17 target=#2
OP          Damage base=8 hits=1
HOOK        BeforeDamage
RNG         CombatTargets counter 17 -> 18, value=...
STATE       enemy#2.hp 43 -> 31
HOOK        AfterDamage
DECISION    SelectCards candidates=[#4,#8,#11]
```

A conformance failure should be reproducible with tracing enabled without changing gameplay semantics.

---

## Fault Model

Exact execution should use explicit faults rather than no-op fallbacks for invalid external inputs.

Example:

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

The compact search API may assume an action produced by `legal_actions()`, but the oracle-facing API must validate inputs and expose the reason for failure.

---

## State Size and Performance

The current state-size discipline is valuable, but byte size is a performance metric, not a correctness contract.

Adding:

- exact RNG streams,
- continuation frames,
- an effect stack,
- power provenance,

may increase state size.

That is acceptable unless profiling shows that copying the larger state becomes a material bottleneck.

The order of work should be:

1. establish exact semantics;
2. establish differential validation;
3. benchmark;
4. optimize measured bottlenecks.

Possible later optimizations include hot/cold state splitting, but they should not be introduced before correctness is stable.

---

## Testing Strategy

The extracted fork currently has only minimal smoke coverage compared with the original project history.

The long-term test stack should contain three layers.

### Local unit and invariant tests

Examples:

- RNG vectors;
- damage rounding;
- pile operations;
- action legality;
- state invariants;
- exact content-specific regression cases.

### Determinism tests

For identical state and action:

```text
step(state, action) == step(state, action)
```

This remains necessary but is not sufficient for correctness.

### Oracle conformance tests

The primary semantic gate.

CI should include a representative deterministic corpus of oracle-validated traces, while larger randomized differential runs can be executed separately when appropriate.

---

## Recommended Migration Order

### Phase 0 — Canonical contract

Add:

- `CanonicalAction`;
- `CanonicalCombatState`;
- stable instance identities;
- checked `try_step`;
- canonical digest/export.

Do not change gameplay behavior yet.

### Phase 1 — Exact STS2 RNG

Replace the custom three-stream RNG in exact mode with:

- xoshiro256** `MegaRandom`;
- exact seed derivation;
- exact stream set;
- exact counters;
- exact integer/float/shuffle semantics.

Add direct RNG vector tests against the oracle/game implementation.

### Phase 2 — General continuation

Replace policy-driven mid-resolution decisions with explicit `Decision + FrameStack`.

Use an existing mid-enemy-turn player choice as the first architecture test.

### Phase 3 — Explicit effect stack

Remove recursive hook execution as semantic state.

Replace `MAX_HOOK_DEPTH` truncation with explicit work items and an erroring safety budget.

### Phase 4 — Exactness audit

Audit every known approximation, conservative behavior, unmodelled mechanic, and unknown content marker.

For each item:

- implement exact semantics;
- add required state;
- or mark the mechanic unsupported in exact mode.

### Phase 5 — Differential fuzzing

Continuously compare simulator and oracle at decision boundaries, minimize failures, and turn them into regression tests.

### Phase 6 — Performance tuning

Only after correctness is measurable:

- benchmark state copy cost;
- benchmark transition cost;
- benchmark legal action generation;
- benchmark rollout throughput;
- optimize the measured hot spots.

---

## Architectural Invariants

The following invariants should be treated as long-term rules for the project:

1. **All continuation-critical state lives in `State`.**
2. **Exact mode never silently approximates unsupported semantics.**
3. **Player decisions are never replaced by hidden simulator policy.**
4. **RNG stream identity and counters are simulation state.**
5. **External protocol uses stable instance identity, not array position.**
6. **Search and conformance APIs share one rule engine.**
7. **Rule execution may be optimized internally but must preserve canonical behavior.**
8. **A work-budget or capacity failure is explicit, never silent truncation.**
9. **Canonical state is stable even when internal state layout changes.**
10. **Oracle disagreement creates a reproducible regression case.**

These rules are more important than preserving a particular state size, file layout, or implementation technique.
