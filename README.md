# Rust Dungeon Simulation

A deterministic, seed-based 2D dungeon crawler written in Rust featuring procedural generation, entity management, and AI pathfinding.

## Quick Start

```bash
cargo run
```
Enter a seed at the prompt to generate the dungeon.

## How It Works

1. **Dungeon Generation**: 20x20 grid generated from seed with random floor/wall placement
2. **Spawning**: Player spawns with 100 health; 3 enemies spawn on random floor tiles
3. **Game Loop**: Each turn, enemies use A* pathfinding to reach the player
4. **Combat**: Enemies attack when adjacent (≤2 tiles), dealing 5 damage per hit

## Win/Loss

- **Lose**: Player health reaches 0
- **"Win"**: All enemies become idle (no path to player)

## Symbols

- `P` → Player
- `E` → Enemy
- `#` → Wall
- `.` → Floor

## Project Layout

```
src/
├── main.rs       # Game loop and entry point
├── dungeon.rs    # Procedural dungeon generation
├── entity.rs     # Player and enemy definitions
├── simulation.rs # A* pathfinding and combat logic
└── render.rs     # ASCII rendering
```

## Dependencies

- `rand` - Random number generation
- `pathfinding` - A* algorithm

## Key Features

- **Deterministic**: Same seed always produces identical dungeon and outcomes
- **Modular**: Clear separation between generation, simulation, and rendering
- **Algorithmic**: Uses A* pathfinding with Manhattan distance heuristic
