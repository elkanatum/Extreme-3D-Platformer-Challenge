# Extreme 3D Platformer Challenge

A physics-based 3D platformer built with Rust and Bevy, featuring exponential difficulty scaling where platforms reach 1,500+ units high while shrinking to microscopic sizes.

!<img width="959" height="539" alt="image" src="https://github.com/user-attachments/assets/83e7247b-d8f0-44f7-b449-30193fb7f78c" />


## Features

- **Exponential Difficulty**: Platforms scale from 24 to 1,500+ units high, shrinking from 4×4 to 0.2×0.2 units
- **Physics-Based Movement**: Realistic jumping, momentum, and collision detection
- **Moving Platforms**: Horizontal and vertical platforms with speeds up to 18+ units/second
- **Dynamic Camera**: Adapts automatically to extreme heights
- **5+ Challenging Levels**: Each level multiplies difficulty by 2.5×
- **Lives & Scoring**: Height-based bonuses and bonus lives every 50 points

## Technologies

- **Rust** - High-performance systems language
- **Bevy 0.12** - ECS game engine
- **Rapier3D 0.23** - Physics simulation
- **SIMD Optimizations** - For enhanced performance

## Quick Start

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.70+)

### Run
```bash
# Clone repository
git clone https://github.com/elkanatum/extreme-3d-platformer.git
cd extreme-3d-platformer

# Run (release mode recommended)
cargo run --release
```

## Controls

- **WASD / Arrow Keys** - Move
- **Space** - Jump
- **R** - Reset game

## Difficulty Progression

| Level | Height Multiplier | Platform Size | Speed | Max Height |
|-------|------------------|---------------|-------|------------|
| 1     | 1.0×             | 100%          | 6.0   | 24 units   |
| 2     | 3.5×             | 65%           | 9.0   | 147 units  |
| 3     | 6.0×             | 30%           | 12.0  | 432 units  |
| 4     | 8.5×             | 15%           | 15.0  | 918 units  |
| 5     | 11.0×            | 10%           | 18.0  | 1,518 units|

## Gameplay

Collect all items in each level while avoiding hazards. Each level features:
- Smaller platforms requiring precise jumps
- Higher altitudes demanding careful navigation
- Faster moving platforms testing timing skills
- Elevated hazards increasing danger

Bonus lives awarded at 50 and 100 points, plus level completion bonuses.

## Technical Highlights

### Dynamic Difficulty Scaling
```rust
let height_multiplier = 1.0 + (level - 1.0) * 2.5;
let platform_size = 1.0 - (level - 1.0) * 0.35;
```

### Key Features
- **ECS Architecture**: Clean, maintainable code using Bevy's Entity-Component-System
- **Adaptive Camera**: Dynamically adjusts to player height (up to 1,500+ units)
- **Kinematic Platforms**: Smooth movement players can ride on
- **Emissive Materials**: High-altitude platforms glow for visibility
- **60 FPS**: SIMD-optimized physics calculations

## Project Structure
```
extreme-3d-platformer/
├── src/
│   └── main.rs          # Complete game code
├── Cargo.toml           # Dependencies
└── README.md
```

## Tips

- Use running jumps for maximum distance
- Time moving platforms carefully
- Plan routes through multiple platforms
- Higher collectibles = more points
- Stay centered on small platforms

## Known Issues

- Camera may clip at extreme heights (100+ units)
- Fast platforms can cause clipping at level 5+

## Future Plans

- [ ] Checkpoint system
- [ ] Power-ups (double jump, speed boost)
- [ ] Sound effects and music
- [ ] Leaderboard
- [ ] Level editor

## License

MIT License - see [LICENSE](LICENSE) file.

## Author

**Elkana Tum**
- GitHub: https://github.com/elkanatum
- Email: elkanatum@gmail.com

## Acknowledgments

- [Bevy Engine](https://bevyengine.org/)
- [Rapier Physics](https://rapier.rs/)
- Rust Community
