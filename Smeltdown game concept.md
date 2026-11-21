# Smeltdown: game concept

## Core concept

- Factorio-inspired 2D puzzle game: Factorio's core loop (mine, craft, transport) squeezed into small, self-contained levels instead of an endlessly growing factory.
- **Levels are procedurally generated**, not hand-made.
- **Space as the main constraint:** small maps with many obstacles (rock formations).
- **Playtime per level:** 5–30 min (easy to hard).
- **No level failure:** the player can rebuild and retry a level infinitely.

## Playthrough

- From the main menu the player presses `Start`, which starts a new playthrough with a random seed. The seed stays the same for the whole playthrough.
- Nine levels over three fixed difficulties, three levels each.
- **Structure:** one tutorial level, generated from a fixed seed, before the nine levels.
- Only one playthrough exists at a time. Its progress is saved, so the player can close the game and resume later. Starting a new playthrough replaces the current one.

## Map

- 2D, grid-based, top-down view.
- Small, with few resource patches; each patch never runs out.
- **Ores:** up to four types. Harder levels use more types up to that cap; beyond it, difficulty comes from deeper recipes with more intermediate items.
- Levels contain only the ores their target needs.
- **Patch count** is up to the generator.
- Fixed delivery point on the map, placed by the generator, fed directly by belts.

## Buildings

- **Two core machine types: miners and assemblers.**
- **Miners:** placed on resource patches; output rate depends on how many ore tiles the miner covers.
- **Assemblers:**
  - Non-square shapes (L, T, S/Z and similar, like Tetris pieces), rotatable.
  - The player gets shapes from one fixed sequence of 20, the same in every level. Each next assembler placed is the next shape in the sequence.
  - One recipe per assembler, chosen by the player.
  - **Buffers:** one capped input buffer per ingredient and one capped output buffer. A machine stops crafting while its output buffer is full.
- **Recipe visuals:** purely visual. An assembler can have a different representation depending on its recipe, e.g. a smelter for ore → plate recipes. No extra toolbar entry.
- **Everything goes over belts**; no direct machine-to-machine transfer.

## Inserters

- Inserters are part of the machine, both in code and visually, at fixed points per shape; the player doesn't place them.
- **Short and long inserters:** each inserter point is marked short or long.
  - **Short:** takes from the belt right next to the machine.
  - **Long:** reaches over that belt to the second belt beyond it.
  - One tile can hold both; the marking is shown on the machine.
  - Together they reach two belts, so up to four lanes and four ingredients per machine.
- **Rotation matters:** inserter points have to face the belts.
- Inserters handle input and output. Output goes through one short inserter at a fixed point per shape and lands on the far lane of the adjacent belt, as in Factorio; there are no long outputs.
- Inserters take only what the recipe needs, until the machine's small buffer is full. Inserter speed is a fixed rate and limits throughput.
- Belts passing several machines feed all of them, Factorio main-bus style.
- Inserters are not separate machines → don't count toward the machine score.

## Transport

- **Two-lane belts**, single belt type.
- **Splitters:** split one belt into two belts, but items keep their lanes.
- **Merging belts:** Factorio-style side-loading.
- **Belt overpasses:** a crossing tile; both lanes pass through unchanged.
- **No underground belts**
- Items back up when a belt is blocked or ends.
- Sushi belts are allowed to deadlock. That's the player's design error, not flagged by the game; the generator always assigns one item type per lane.

## Recipes

- **Pre-made recipes:** each difficulty has a fixed set of possible recipes, Factorio-style (e.g. iron/copper ore → plates → circuits; iron → gears).
- Recipe complexity scales with difficulty: deeper trees and more combinations of intermediate items.
- Up to four input ingredients per recipe.
- Each recipe has exactly one output item.
- Each recipe defines an amount per ingredient, an output amount (e.g. 2 gears per craft) and a crafting time.

## Level generator

- **Inputs:** seed and difficulty. Difficulty controls map size, recipe depth, obstacle density and the number of ore types.
- **Difficulty levels:** exactly three fixed levels: easy, medium, hard.
- **Deterministic:** identical inputs always produce an identical level.
- Also produces a reference solution together with the level, so every level is solvable.
- The reference solution should be challenging but fair: good, but deliberately not optimal, so a skilled player can match or beat its time and machine targets.
- The reference solution uses the same parts the player has: the fixed shape sequence, built-in inserters, belts, splitters, overpasses.
- The reference solution always uses one item type per belt lane.
- The reference solution's target values are measured by running it once through the game's simulation.

## Runs

- **Build, then run**, not a live simulation like Factorio.
- **Run button:** always usable, even with an unfinished layout.
- The player can't edit the map during a run.
- Stopping a run clears all items, buffers and progress, so every new run starts clean.
- The simulation runs in ticks: discrete steps in which every belt, inserter and machine updates once.
- **Fixed simulation step, variable playback speed:** fast-forward runs more ticks per second, not bigger ones.
- **Playback:** by default the run plays in real time, with buttons to speed it up, plus a "solve now" button that runs it headless straight to the result. Run evaluation works the same in all three modes; while watching, the result appears as soon as the factory loop is found.

## Run evaluation

- **Goal:** production rate of the target item, e.g. 20 circuits per minute.
- **How a run is judged:** the simulation runs until it detects that the factory is repeating itself. After that, its output stays the same, so the rate can be measured exactly.
- **Why it always repeats:** the simulation is deterministic and the factory has only finitely many possible states (finite belts, capped buffers, ore that never runs out). So at some point a state must start repeating itself; that repeating part is the factory loop.
- **Measuring the factory loop:** the simulation records when the loop starts, how long one loop lasts, and how many items are delivered per loop. From these three values:
  - **Steady rate:** e.g. a 10 s loop that delivers 4 circuits means 24 circuits per minute, forever.
  - **Level won:** steady rate ≥ target rate.
  - **Time score:** the moment the loop starts, shown in seconds.
- **Deadlocks:** a deadlocked factory counts as a loop with zero output, since nothing changes anymore, and fails. As a consequence, a factory that reaches the rate only for a short time and then deadlocks fails too (needs no extra code).
- **Cutoff:** if no factory loop is found in time, the run fails (a factory that takes too long to settle isn't efficient). The limit is 10 × (the reference solution's loop start + loop length), at least 10 minutes. E.g. a reference that settles after 90 s and repeats every 30 s gives a 20-minute cutoff. This is simulated time, not real time: with "solve now" it's reached in seconds.
- **Timing design rule:** crafting times, miner rates, inserter speeds and belt speed can be any values, as long as their least common multiple stays well under the cutoff, so factories loop early. E.g. 2.4, 3.6 and 4.8 s repeat every 14.4 s; 2.3, 3.7 and 4.9 s only after about 7 minutes. A unit test should catch timings that don't line up.
- **How the loop is detected:** after every tick, the simulation saves a fast 128-bit hash of the whole factory state.
  - **Warning:** nothing is allowed to use the global tick counter. Every entity must keep its own timers and update every tick. Otherwise loop detection will break.
  - The state includes hidden values that affect the future:
    - **Splitters:** which output they use next.
    - **Inserters:** swing progress, and whether and which item they're holding.
    - **Machines:** crafting progress, input and output buffers.
    - **Miners:** progress on the current ore.
    - **Belts:** each item's exact position within a tile.
  - Run statistics like the global tick counter or total items delivered are left out, since they only grow and would never match.
  - Collisions are practically impossible, so a hash match with an earlier tick means the factory loop has been found.
- **Slow fill-up:** belts or buffers that slowly fill up delay the loop start, which costs time score even when the output already looks steady.

## Scoring

- **Time:** the loop start (see "Run evaluation"), shown in seconds, not as raw ticks.
- **Machines:** machine count compared with the generator's reference solution. Fewer than the reference earns bonus points, more costs points.
- **Shown to the player:**
  - The goal: target item and rate.
  - Machine count compared with the target.
  - Time of the last run compared with the target.
  - The run controls.

## UI

- **Recipe menu:** Shows the recipe tree and all numbers per recipe: inputs and amounts, outputs and amounts, crafting time.
- **No calculators:** the game doesn't compute machine counts or ratios for the player. Working those out is part of the puzzle.
- **Hover tooltips** on the map show the recipe of the hovered machine.
- **Undo and redo function** in build mode.
- **Build toolbar:** access to everything the player can place (miners, the next assembler shape, belts, splitters, overpasses).
- **Rendering is decoupled from ticks:** the frame rate is independent of the simulation's tick rate. Everything on screen (moving items, inserters, the player's own actions) must be drawn smoothly between ticks.

## Software architecture

- **Modules:**
  - Level generator (from seed and difficulty)
  - **Building:** placement rules, undo/redo, layout compilation
  - Tick simulation and scoring
  - Saving
  - Rendering, UI and game flow
- **Data flow:**
  - Generator → layout
  - Building → edits layout
  - Layout → network (compiled)
  - Network → simulation → scoring
  - Rendering ← reads layout and run state
  - Saving ↔ layout and progress
- **Build phase:** the player edits the generator-provided layout.
- **Layout compilation:** when Run is pressed, the layout is compiled into a network: a list of belt lanes, what each lane feeds into, how merges resolve, and which lanes each machine's inserters read and write. The simulation never looks at the grid; it only steps the compiled network.
- **Two kinds of state:**
  - **Layout** (the grid the player edits: tiles, buildings, rotations, belts): written by the generator, build mode and loading a save.
  - **Run state** (items, buffers, timers): written only by the simulation. Each run creates a fresh one; stopping discards it.
- **Determinism rules:** all randomness comes from the initial RNG seed, and the simulation only uses integers, not floats, for positions and timers.
- **Tech:** UI framework still to be chosen.

## Team

| Member | Owns |
| --- | --- |
| Aegeontis | Level generator, reference solution, tutorial seed |
| Zuebot | Tick simulation on the compiled network, loop detection and cutoff, rate measurement, scoring |
| Cudoras | Building rules (placement, collision, rotation, inserter points), layout → network compilation, belt merging rules, undo/redo, saving |
| Drill8888 | Framework choice, rendering incl. interpolation, input handling, camera, placement preview, recipe menu, tooltips, game flow (menus, level progression, build/run/result states), end-to-end tests |

## Scope increasers (if time allows)

- **Gameplay extensions:**
  - Generated recipes
  - Generated or per-level assembler shape sequences, with generated input and output points
  - Sliding difficulty that rises from level to level, up to infinite levels
  - Underground belts
- **Separate game modes:**
  - **Limited resources:** ore runs out, so the goal is making the most of a fixed supply
  - **Spoilage:** some items have a short lifespan, so routes and buffers must be short; expired items turn into spoilage, which must be removed
  - **Hidden recipes:** recipes aren't shown, so players work them out by experimenting with machine inputs
- **Polish:**
  - Multiple save slots
  - Fancy names for menus and buildings (e.g. "Smeltbook" instead of "recipe menu")
  - Final art replacing the placeholder assets
- **Platforms:**
  - Web build (WASM), mobile builds

## Questions for the team

1. **\[Transport\] Merging belts:** Factorio-style side-loading or a dedicated merger building? For grading, side-loading is the more interesting code (belt topology, lane assignment, priority between straight and side input). A merger is simpler: one more building with an alternate-inputs rule, but whose turn it is then has to be tracked in the state hash.
2. **\[Map\] Delivery point and wrong items**, three options:
   1. Accept any item; only target items count.
   2. Accept any item, but deduct points for wrong ones.
   3. Accept only target items; a wrong item blocks the belt (which fails the run).
3. **\[Run evaluation\] Win/target rate:** fixed per difficulty, or derived from the generated reference solution? Derived is easier for the generator: the reference solution is simulated anyway, so its rate is always reachable. Fixed is harder: the generator must guarantee a solution that hits the rate on a cramped map.
4. **\[Buildings\] Mirroring assemblers:** can the player mirror shapes, or only rotate them? Mirroring only adds new shapes for asymmetric pieces like L or S (a mirrored T is still a T); without it, the fixed shape sequence matters more.
5. **\[Transport\] Splitter filtering:** should splitters be able to filter items from the start?
6. **\[Scoring\]** **Add belt tiles used as a third score?** The minimum machine count mostly follows from recipe ratios, so it rarely rewards a better layout. Compared against the reference layout?
7. **\[Playthrough\] Level progression:** does a level have to be won before the next one unlocks, or can the player skip ahead?
8. **\[Playthrough\] Saved scores:** are the best time and machine count saved per level, or only the last run?
9. **\[Buildings\] Buffer sizes:** a fixed number of items for every machine, or a multiple of the recipe's amounts (e.g. enough for two crafts, as in Factorio)? Bigger buffers smooth throughput but delay the loop start.
