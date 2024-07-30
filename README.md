# Game of Life

Rust implementation of [Conway's Game of Life](https://conwaylife.com/wiki/Conway%27s_Game_of_Life).

## Implementation assumptions

### Universe size

In Game of Life, the "universe" (grid) is unbounded, infinite. Implementing this in a computer is impossible given the finite memory constraint. There are algorithms that allow for optimizing and simulating the "infiniteness" of the game. However, that is beyond the scope of this exercise. To account for a finitely sized grid, the following adaptation will be made to this implementation of the game: every "border" cell (outside of the main grid) is assumed to be dead and cannot be turned back to life.

### Examples

Some examples are provided to visualize some common Life patterns without running the main app. The examples provided are a [blinker](https://conwaylife.com/wiki/Blinker), a [glider](https://conwaylife.com/wiki/Glider) and a [pulsar](https://conwaylife.com/wiki/Pulsar).

The examples can be run with:

```bash
cargo run --examples blinker
```

```bash
cargo run --examples glider
```

```bash
cargo run --examples pulsar
```