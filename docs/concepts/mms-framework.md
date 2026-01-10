# MMS framework (Model · Mechanics · Signals)

MMS is a world-definition and gameplay runtime framework designed for World Alchemist but could also be used in other projects. It replaces ECS-style “iterate everything every frame” with a reactive, causal pipeline inspired by Unreal Engine's Gameplay Ability System (GAS).

The core idea is to have a loop like this: Signals happen → Mechanics evaluate → Effects mutate the Model → new Signals are emitted.

MMS follow World Alchemist design-to-runtime approach and so is intended to scale from pure worldbuilding (cosmology, territories, factions, races, classes, characters, items) to full gameplay (abilities, rules, modifiers, triggers) while remaining:

- **Deterministic-friendly** (replayable timelines)
- **Auditable** (every change has a cause)
- **Tool-first** (introspectable in a command-driven authoring environment)
- **Data-driven** (everything is defined in data, no hardcoded rules)
- **Composable** (modular building blocks, reusable logic)

In the folowing sections, i will describe the components that make up MMS.

## Model

The Model bounded context is where World Alchemist describes the world as data: what kinds of things exist, what specific things currently exist, and what properties and relationships define them. Model data is meant to be stable, inspectable, and serializable. It is also the primary surface that authors interact with when they “define the world,” while still being safe to mutate at runtime via mechanics-driven effects.

### Archetypes

An Archetype is the type-level definition of a world concept. In practice, archetypes are your worldbuilding vocabulary: Character, Race.Elf, Class.Pyromancer, Faction, Territory, Biome, Cosmology.Plane, Item.Sword, and so on. An archetype defines which attributes exist for that concept (their types, defaults, constraints, and editor metadata), which traits are always present, and which aspects classify the concept for downstream gating and routing.

Archetypes should be treated as assets: they are versioned, referenceable, and loaded into a registry so they can be referred to by stable identifiers. Archetypes may form hierarchies or compositions (depending on your authoring model), but the core expectation is that an archetype is declarative and free of gameplay logic; it describes the shape of data and the static composition of an archetype.

### Entities (instances)

An Entity (or often called an instance) is a concrete occurrence of an archetype in the world like “this specific character,” “this specific region,” “this specific sword.” Entities exist in the world timeline and are addressed by stable entity identifiers. They carry per-entity state such as attribute overrides, dynamic trait attachments, and relationships to other entities (ownership, containment, membership, adjacency, parent/child, inventory, faction affiliation, territorial control, and so forth).

Entities may also carry aspects that evolve over time, either because mechanics attaches traits/effects that add aspects, or because the world state crosses a threshold that changes classification. Importantly, while entities store canonical state, they should remain tool-friendly: World Alchemist should be able to inspect an entity, list its traits, show its resolved attributes, and explain how those resolved values were produced.

### Attributes

Attributes are typed fields describing archetypes and entities. They can be simple scalars (numbers, booleans, strings), structured values (lists, sets, maps), and references (to other entities, archetypes, traits, or mechanics assets). Attributes are the foundation for nearly all higher-level constructs: stats, resources, coordinates, reputation values, territory parameters, cosmology constants, and even authoring metadata.

A key design choice in MMS is how attribute values are computed. The recommended approach is layered resolution rather than a single flat value. In a layered model, the resolved value of an attribute can be the result of multiple contributions: the archetype default, one or more attached traits, and one or more active effects, each with defined stacking/priority semantics. This makes provenance and removal clean: detaching a trait or expiring an effect naturally removes its layer without requiring ad-hoc cleanup, and World Alchemist can explain the final value by showing the contributing layers.

Regardless of representation, attribute mutation must remain disciplined: runtime changes to attributes should occur through mechanics-produced effects so that the model stays consistent, replayable, and fully auditable.

### Traits

Traits are MMS’s primary composition primitive inside the Model. They exist to make world definitions reusable and scalable: instead of repeating the same attribute bundles and capability grants across many archetypes and entities, you define a trait once and attach it wherever it applies. Traits are deliberately data-first. They can carry structure, defaults, and parameters, but they must not embed gameplay logic.

A trait’s job is to declare what an entity has (attributes, classifications, and grants by reference). Mechanics then decides what that means during signal processing.

#### TraitDefinition: what a trait is

A TraitDefinition is an authored asset in the Model context. It typically includes:

- Attribute bundle: definitions (and optional defaults) that the trait contributes.
- Aspect contributions: aspects that become effective while the trait is active.
- Ability grants (by reference): stable references to Mechanics abilities.
- Passive effect grants (by reference): optional references to Mechanics effects that should be active while the trait is active.
- Signal bindings (data-only): optional declarative mappings that emit requests when signals occur.

The key boundary is the grants: traits may name abilities and effects, but they never contain the execution logic for them.

A Rust-shaped sketch (conceptual) could look like:

``` rust
pub struct TraitDef {
    pub id: TraitId,
    pub aspects: AspectSet,


    // Data contributed by the trait
    pub attributes: AttributeBundle,


    // Grants by reference into Mechanics registries
    pub ability_grants: Vec<AbilityId>,
    pub passive_effect_grants: Vec<EffectId>,


    // Data-only reactive mappings
    pub bindings: Vec<SignalBinding>,


    // How multiple entities of the same trait behave
    pub stacking: TraitStacking,

}
```

This struct is intentionally “boring”: it is meant to be authored, validated, diffed, and versioned easily.

#### Trait Instance: attachment to the world

A Trait Instance is the runtime attachment of a trait to either an archetype (static inheritance) or an entity (dynamic state). Trait instances carry the minimum state needed for correctness and provenance:

- `trait_id`: which trait is attached
- `params`: parameter values for parametric traits
- `source`: who or what granted it (for auditing)
- `enabled`: whether it is currently active
- optional `stack_key` / `stack_count` depending on policy

The Model should be able to answer: “which traits does this entity have, and where did each come from?”

#### Parametric traits (high leverage)
Traits become dramatically more powerful when they accept parameters, because you can define a single template and reuse it across many entities.

Examples:

- `ElementalAffinity { element: Fire, bonus: 0.15 }`
- `TerritoryClimate { kind: Taiga, severity: 0.8 }`
- `WeaponProfile { damage: 12, speed: 0.9 }`

Parameters should be stored on the TraitInstance so they are part of the replayable Model state, and any derived behavior should be implemented in Mechanics (for example, how the bonus affects damage or resistances).

#### How traits affect attributes (materialized vs layered)

Traits can contribute attributes in two broad ways.

- In a materialized approach, attaching a trait physically creates or mutates attribute values on the entity. This is straightforward to implement but tends to create cleanup complexity on removal.
- In a layered approach (recommended), traits contribute layers to attribute resolution. The final value is computed from multiple contributors (archetype defaults, traits, active effects) using deterministic stacking rules. Layered resolution makes removals and expirations trivial: when a trait is detached, its layer disappears with no cleanup code. It also improves explainability because World Alchemist can show provenance as a list of layers.

#### Grants: abilities and passive effects

Trait grants exist in two categories.

- Ability grants make an ability available to an entity. The trait stores only the AbilityId references. Whether the ability can be activated at a given moment is still decided by Mechanics using aspect queries and model predicates.

- Passive effect grants are optional and represent “while this trait is active, these effects should be present.” A common pattern is that the trait does not directly apply the effect; instead, the runtime derives the desired set of passive effects from active traits and ensures they exist as effect entities (adding/removing them transactionally as needed). This prevents drift and ensures trait toggles cleanly update the world.

#### Signal bindings: data-only reactivity

Traits can include declarative bindings to express “reactive flavor” without code in the Model. A binding observes incoming signals (usually events) and emits a request signal.

A conceptual binding structure:

```rust
pub struct SignalBinding {
    pub when_type: SignalTypeId,
    pub when_aspects: AspectQuery,
    pub scope: BindingScope, // e.g., self, source, target, zone
    pub emit: SignalTemplate, // typically a Request
}
```

Bindings must never apply effects directly. They only emit signals so that Mechanics remains the sole decision-maker and Effects remain the sole mutation mechanism.

#### Stacking, uniqueness, and conflicts

Traits need explicit stacking semantics to avoid ambiguity.

Common policies include:

**Unique**: only one entity may exist; reapply refreshes provenance/time.
**Stacking**: multiple entities allowed up to a max; magnitude may scale with stack count.
**Refresh**: reapply refreshes duration/enablement without adding another stack.
**Merge**: parameters merge deterministically (careful; define exact rules).

Stacking is a rule decision, but the result must be represented in the Model deterministically so replay remains stable.

#### Attachment and detachment (Effects-only)

Attaching or detaching a trait is a state change and must occur only through Effects so it remains transactional and auditable.

Rust-shaped examples:

```rust
Effect::AddTrait { target: EntityId, trait_id: TraitId, params: TraitParams, source: EntityId }
Effect::RemoveTrait { target: EntityId, trait_id: TraitId, source: EntityId }
```

On commit, the runtime may emit canonical events such as Signal::TraitAdded { .. } and Signal::TraitRemoved { .. }, as well as derived changes (ability availability changes, aspect changes, and attribute layer changes). World Alchemist should be able to show these commits as a clear causal story: which signal caused the trait change, which effect performed it, and what the resulting state differences are.

#### Tooling guidance for World Alchemist

World Alchemist should treat traits as first-class authoring objects with strong validation:

- All granted AbilityId and EffectId references must resolve.
- Trait stacking policies must be coherent.
- Attribute bundles should validate types and namespaces.
- Aspect contributions should be registered and consistent.
Bindings must be explicit and deterministic.

Traits should be easy to browse and reuse: the tool should show where a trait is used (archetypes and entities), what it grants, and what attributes/aspects it contributes. It should also be able to explain why a given entity currently has a trait (source, path of grants, and timeline location).

### Aspects

Aspects are hierarchical, namespaced identifiers used to classify and reason about objects across all three bounded contexts: Model, Mechanics, and Signals. They provide a shared vocabulary for gating, routing, stacking decisions, analytics, and debugging—without embedding gameplay logic into data.

The most important principle is that aspects are descriptive, not imperative. They do not “do” anything by themselves. Mechanics uses them to make decisions, and Signals uses them to route messages, but the **aspect** system remains a compact, deterministic layer that World Alchemist can expose cleanly.

#### Authoring paths vs runtime IDs (Rust conventions)

In World Alchemist, developers work with human-readable aspect paths like ``damage.fire`` or ``class.mage.pyromancer``. At runtime, these paths should compile into stable, strongly-typed identifiers for performance and determinism.

The registry is the bridge between authoring and runtime:

- Authoring: strings like "damage.fire"
- Runtime: interned AspectIds resolved via AspectRegistry

For replay safety, the registry must be deterministic: the same set of authored aspect paths must compile to the same IDs in the same build/config.

#### Where aspects live

Aspects attach to nearly everything, but with different intent.

In the Model, archetypes, traits, and entities carry aspect sets to describe identity and classification (for example, entity.character, race.elf, zone.territory.north). These aspects are typically used for rule checks (who can do what), filtering (which entities match a query), and explainability (“why is this considered hostile?”).

In Mechanics, abilities and effects carry aspects primarily for gating and interaction semantics. Abilities commonly define requirements and blockers as AspectQuery values. Effects use aspects to drive stacking, immunity, cancellation, and analytics (for example, status.dot.fire, modifier.resistance.fire, grant.ability).

In Signals, aspects behave like routable topics. A signal might carry aspects such as signal.combat.damage and damage.fire so that multiple subsystems can subscribe broadly (combat) or narrowly (fire).

#### Hierarchy and matching

Aspects are hierarchical by default. Conceptually, damage.fire implies damage, and class.mage.pyromancer implies class.mage.

There are two good ways to implement this deterministically:

- Closure expansion: when an aspect is inserted into a set, the set also receives all ancestors.
- Precomputed parent tables: the registry provides fast “is-a” checks without expanding sets.

World Alchemist treat hierarchy as a first-class authoring tool: authors can browse and validate aspect trees, and the runtime can match rules at the right level of generality.

Matching should remain intentionally small and predictable. The recommended runtime surface is all_of, any_of, and none_of. Tooling may offer wildcard conveniences (like class.mage.*), but those should compile into explicit queries so that runtime matching stays deterministic.

#### Declared aspects vs effective aspects

It is useful to distinguish between what an object declares and what it effectively has at runtime.

An archetype may declare entity.character, but an entity’s effective aspects may also include aspects contributed by attached traits and active effects (for example, state.stunned, status.burning, faction.hostile). In a layered model, these contributions should be explainable: World Alchemist should be able to show which trait or effect introduced a given aspect, and when it will be removed.

This is especially important for gating and routing. If an ability is blocked by state.silenced, that aspect must appear in the entity’s effective aspect set in a traceable way (typically via a persistent effect or trait that contributes the aspect).

#### Practical usage patterns

Aspects become powerful when you standardize their use.

- **Gating (abilities and rules)**: abilities express requirements and blockers as aspect queries. Example: an ability might require class.mage and be blocked by state.silenced.

- **Effect interaction (stacking / immunity / cancellation)**: effects classify themselves and declare how they interact. Example: status.burning may be cancelled by status.wet, and blocked by status.immunity.fire.

- **Signal routing (topics)**: signals attach aspects that describe their category and payload nature. Example: signal.combat.damage plus damage.fire lets listeners subscribe to all damage or only fire damage.

### Taxonomy guidance for World Alchemist

To keep the system coherent and prevent “tag soup,” World Alchemist enforce conventions:

- Prefer lowercase dot-separated paths (status.burning, not StatusBurning).
- Keep namespaces stable and meaningful (entity.*, race.*, class.*, state.*, status.*, damage.*, signal.*, ability.*, effect.*).
- Use aspects for classification, not for arbitrary data (store magnitudes as attributes/effect params).
- Avoid negative aspects (use none_of queries instead).
- Validate that authored assets only use registered aspect paths and that namespaces do not collide.

From a tooling perspective, aspects should be searchable and explorable. World Alchemist should be able to print the aspect tree, show where an aspect is used (archetypes/traits/abilities/effects/signals), and explain why an entity currently matches a given query.

## Mechanics

Mechanics is the execution layer of MMS: it is where the world’s rules are defined, where decisions are made, and where state changes are authored as explicit, replayable operations. Mechanics is deliberately separated from the Model so that “what the world is” (data) does not get entangled with “how the world behaves” (logic). The only way Mechanics is allowed to alter the world is by producing Effects, which are then applied transactionally to the Model. This keeps gameplay evolution traceable and makes World Alchemist capable of explaining why the world changed, not just what changed.

### Abilities

An Ability is a reusable Mechanics asset that represents a gameplay verb. Abilities exist as authored definitions (assets) and are activated through signals (typically a request such as TryActivateAbility). Traits and other model constructs may grant abilities, but always by stable reference; the ability’s behavior stays in Mechanics.

Conceptually, an ability definition includes three layers of meaning:

First, it declares eligibility—who can use it and under what conditions. This is expressed through AspectQueries (requirements and blockers) evaluated against the current Model. Eligibility commonly includes both caster-side checks (class, equipment, status effects, permissions) and target-side checks (target type, immunity, faction relation, line-of-sight constraints expressed as model queries). Because these checks must be deterministic and explainable, they should be expressed as explicit rule predicates rather than hidden code paths.

Second, it declares interaction semantics—how it selects targets and how it interprets the world. Targeting may be as simple as “self” or “single entity,” but it may also cover collections (cone, radius, chain, territory scope) and selection rules (nearest hostile, lowest health ally, entities with a specific aspect). Even when selection is complex, the result should be a concrete, serializable target set so it can be replayed or audited.

Third, it declares an execution pipeline—what the ability does when it is accepted. Execution is best represented as a sequence of deterministic steps that yield effects. Typical steps include spending resources, applying cooldowns, spawning transient entities, applying statuses, emitting events, or scheduling delayed pulses. The key is that execution does not “write state” directly; it produces a list of effects that are applied by the runtime in a transaction.

Abilities should also define how they behave as runtime entities. Even when the authored asset is static, the runtime may maintain ephemeral entity state such as remaining cooldown time, channeling progress, charges, or queued follow-ups. That entity state should still be model-addressable and replayable—either as explicit attributes/effects in the Model or as derived state that can be reconstructed from the signal timeline.

### Effects

An Effect is the atomic unit of mutation in MMS. Effects are the only mechanism that can change the Model, which makes them the central point for determinism, auditing, and conflict resolution. Effects may be instantaneous (apply once) or persistent (remain active over time).

Instantaneous effects cover direct state transitions such as spending or granting resources, changing a numeric attribute, moving an entity, spawning or despawning entities, granting or removing traits, or emitting a canonical event. Persistent effects represent ongoing modifiers and statuses: buffs, debuffs, auras, DOTs, immunities, and timed grants. Persistent effects should exist as effect entities with stable identities, clear lifetimes, and explicit expiration behavior so that they can be inspected (“what is affecting this entity right now?”) and replayed faithfully.

Effects should carry provenance and classification. Provenance typically includes a source/instigator, a set of targets, optional magnitude parameters, and an aspect set describing the nature of the effect (for example, status.dot.fire, modifier.resistance.fire, or grant.ability). Classification matters because it drives stacking and cancellation logic in a predictable way.

Stacking and conflict behavior should be formalized rather than ad-hoc. Common policies include additive and multiplicative stacking, priority-based overrides, unique-by-key rules (only the strongest entity applies), refresh-on-reapply rules (reset duration), and mutual exclusion/cancellation rules (for example, status.wet cancels status.burning). The runtime should resolve these policies at transaction time so that a single commit yields a consistent post-state.

Finally, effects should be designed to be reversible or replayable. In a layered attribute model, many persistent effects do not “overwrite” values; they contribute a layer that can be removed cleanly at expiration. This is one of the main reasons MMS prefers layered resolution: detaching a trait or expiring an effect naturally removes its contribution without cleanup code.

### Rules, Policies, and Evaluators

Rules and policies are the decision fabric of Mechanics. They determine whether requests are accepted, how conflicts are resolved, and which effects are permitted to apply. In MMS, rules should be phrased as deterministic evaluations over Model state, aspects, and explicit parameters. Some rules are local to abilities (activation gating, targeting restrictions), while others are global (cosmology constraints, faction permissions, territory governance, class restrictions).

To keep the system inspectable, it is useful to differentiate between gating rules and resolution rules. Gating rules answer “is this allowed?” (requirements, blockers, permissions). Resolution rules answer “if multiple things apply, which one wins?” (priority, strongest-wins, exclusive sets). Both should be expressed in a way that World Alchemist can explain, ideally by returning not only a boolean but also the reasons (which requirement failed, which blocker matched, which policy selected the winner).

Evaluators are the pure computation side of Mechanics. They compute derived values such as damage numbers, mitigation, scaling with level, economy formulas, reputation effects, and any other derived quantity used by abilities and rules. Evaluators should be deterministic and side-effect free, and they should operate on resolved model views (for example, a resolved attribute value that already accounts for trait layers and active effect layers). This separation prevents subtle bugs where computations accidentally depend on evaluation order.

Taken together, Abilities (verbs), Effects (mutations), and Rules/Evaluators (decisions and math) form a cohesive mechanics layer: signals request or announce, mechanics decides and produces effects, and the model commit becomes the new truth that drives the next step in the causal timeline.

## Signals & Bindings

Signals are the connective tissue of MMS. They are the canonical units of causality: every meaningful change to the world begins with a signal, and every meaningful outcome can be recorded as one. This is the foundation for traceability and replay. A signal is always typed, immutable, and serializable, and it can carry aspects that help classify and route it consistently.

In MMS, signals are not “callbacks” and they are not “logic.” They are statements: an intent was expressed, an outcome occurred, or time advanced. Mechanics is responsible for interpreting signals against the current Model and deciding what effects to apply. This separation is what makes it possible for World Alchemist to show clean causal chains instead of forcing authors to infer behavior from final state alone.

### Signal families

While signal types are fully customizable, MMS benefits from grouping them into three broad families.

Requests (commands / intent) represent an attempt to do something that may fail due to rules. Examples include TryActivateAbility, TryEquip, TryMove, TryCraft, or TryStartDialogue. A request signal is where permission checks, blockers, and conflict resolution occur. If the request is accepted, mechanics produces effects and emits follow-up events that capture what actually happened. If it is rejected, mechanics may emit a rejection event such as AbilityActivationRejected with explicit reasons.

Events (facts / outcomes) represent something that already happened. They should be emitted as a consequence of committed effects, not as speculative intents. Examples include AbilityActivated, DamageApplied, TraitAdded, CooldownStarted, TerritoryCaptured, ItemEquipped, and EntitySpawned. Events are valuable for secondary reactions (proc effects, quest progression, AI responses) because they are stable facts in the timeline.

Scheduled signals (time / pulses) represent the progression of time and deferred work. Examples include CooldownExpired, EffectExpired, periodic ticks for DOT or regeneration, and delayed follow-ups such as “explode after 3 seconds.” Scheduling should be explicit rather than implicit, so that the same schedule can be replayed deterministically and inspected in tooling.

This intent/fact/time split is not purely stylistic. It makes debugging far simpler because you can see exactly where a decision was made (request handling) and what it produced (events), and you can attribute any late effects to explicit scheduled pulses rather than hidden timers.

### Payloads, identity, and determinism

A signal’s payload should be designed to support replay and audit. At minimum, this usually means including stable references (entity IDs, archetype IDs, ability/effect refs) and any parameters necessary for deterministic interpretation. When a signal implies selection (for example, target acquisition in an area), the output of that selection should become part of the causal record—either by capturing the chosen targets in the resulting events or by making the selection itself explicit. The goal is that a replay does not depend on nondeterministic queries that could pick different results if the world state diverges.

Signals may also carry a unique signal ID (or sequence number) and metadata such as timestamp, instigator/source, and a correlation handle for tracing. None of this metadata should change mechanics semantics unless explicitly modeled; it primarily exists for tooling, analytics, and trace navigation.

### Routing and scopes

Routing determines which mechanics handlers should be asked to react to a signal. Routing is intentionally not gameplay logic; it is selection of listeners. The routing surface usually includes the signal’s type, an optional scope (entity-scoped, zone/territory-scoped, global), and its aspects.

Aspects are particularly powerful in routing because they let the author subscribe to classes of signals without enumerating every type. For example, an ability proc system might listen for signal.combat.damage while a territory governance system listens for signal.world.enter_region and signal.world.territory.*. The important discipline is that routing remains deterministic and explicit so World Alchemist can explain why a handler ran.

### Triggers and bindings

MMS treats “triggers” as declarative conditions over signals and model context that result in new intents. The preferred pattern is: an incoming event is observed, conditions are checked (often using aspect queries and simple model predicates), and then a request signal is emitted to ask mechanics to perform an action.

Traits may contribute bindings that behave like reusable trigger modules. A binding is data-only: it says “when a signal matching this pattern occurs (and optional aspect conditions hold), emit this request.” For example, a trait might state that on signal.combat.crit the entity should attempt to activate abilities.ignite. Another trait might emit a defensive request when health falls below a threshold. This keeps reactive behavior authorable in World Alchemist while preserving the core invariant that the Model does not contain gameplay logic.

Bindings should never apply effects directly. They should only emit signals—typically requests—so that all decisions and mutations still pass through Mechanics and the transaction boundary.

### Scheduling semantics

Scheduling is part of the Signals context because it is fundamentally about timeline management. When Mechanics wants something to happen later, it should schedule a future signal. Persistent effects commonly schedule expirations. Cooldowns schedule an expiry. DOT effects schedule periodic ticks. Because scheduled signals are first-class timeline items, World Alchemist can list pending schedules, advance time deterministically, and replay the same schedule precisely.

A useful convention is to ensure that scheduled signals carry enough provenance to remain explainable: which effect or ability scheduled them, what entity they are scoped to, and what should happen when they fire. This makes “why did I take damage three seconds later?” a trivial question to answer.

### Why Signals matter for World Alchemist

World Alchemist benefits from signals more than most runtime architectures because it is a creator tool, not just a game runtime. Signals provide a user-facing narrative of causality: authors can see what they asked the world to do, what rules accepted or rejected, what effects were applied, and what events resulted. Signals also make simulation controllable: you can inject a request, step the runtime, view the emitted events, and iterate on assets quickly.

In short, Signals are the timeline language of MMS. They make the world’s behavior explicit, replayable, and explainable—exactly what a command-driven world creation environment needs.

## Determinism, Replay, Debugging

MMS is designed so that world evolution can be reconstructed and explained. The cornerstone is the separation between immutable signals (timeline) and explicit effects (mutations). If you persist a signal stream and keep your registries stable, you can replay the same sequence and rebuild the same model state. This is valuable for debugging, automated testing, network synchronization strategies, and authoring confidence.

For auditability, each model commit should be traceable. A commit can record the input signal(s) that were processed, the exact list of effects that were applied, and the resulting diffs. Even when layered attribute resolution is used, the provenance remains explainable because each layer corresponds to a known contributor (archetype default, trait attachment, effect entity). This makes it possible to answer questions like “why did this entity’s fire resistance change?” with a concrete chain of causes.

World Alchemist should expose debugging primitives that operate on this structure. You should be able to trace a signal forward to see what mechanics handled it, what effects were produced, and what commits occurred. You should also be able to inspect an entity’s resolved state and ask for explanations of specific values or derived permissions (for example, which trait grants an ability and which blocker aspect prevents activation). Because routing and evaluation are explicit steps, the tool can present a clear picture of what happened instead of forcing the author to infer behavior from state alone.

## Recommended Defaults

MMS works best when a few defaults are treated as non-negotiable conventions. Model mutations should occur only through effects, and traits must remain data-only while abilities remain mechanics assets referenced by stable IDs. Aspects should be hierarchical and used consistently across archetypes, entities, traits, abilities, effects, and signals so that routing and gating share the same vocabulary.

For state representation, layered attribute resolution is strongly recommended. It aligns naturally with traits and persistent effects, makes detach/expiry semantics clean, and gives World Alchemist a straightforward way to explain provenance. On the runtime side, signal processing should follow explicit phases—ingress, route, evaluate, transact, emit—so that determinism and debugging remain robust and so that the tool can surface meaningful intermediate information.

## Minimal Example (end-to-end)

Consider a character that becomes a pyromancer. In the Model, you define a Character archetype and a Pyromancer trait. The trait adds an attribute contribution such as increased fire power, classifies the entity with an aspect like class.mage.pyromancer, and grants access to an ability reference such as abilities.fireball. The trait does not implement fireball; it only states that the entity should have that mechanic available.

In Mechanics, you define the abilities.fireball asset. Its activation gate requires that the caster match the appropriate aspects (for example, any class.mage.* or a specific trait.pyromancer), and it may be blocked by aspects such as state.silenced. When executed, the ability produces effects in a deterministic sequence: spending mana, applying fire damage, starting a cooldown, and emitting outcome events.

In Signals, the interaction is driven by a request such as TryActivateAbility(caster, ability, target). If mechanics accepts the request, the resulting commit emits events like AbilityActivated and DamageApplied. This small example demonstrates the MMS separation: the Model declares what the entity is and what it has, Mechanics decides what happens and applies effects, and Signals provide the causal timeline that World Alchemist can inspect and replay.