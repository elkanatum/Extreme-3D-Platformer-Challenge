use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "3D Platformer - EXTREME CHALLENGE".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .insert_resource(GameState {
            score: 0,
            lives: 3,
            level: 1,
            collectibles_in_level: 8, // More collectibles for difficulty
        })
        .add_systems(Startup, setup_game)
        .add_systems(
            Update,
            (
                player_movement,
                player_jump,
                camera_follow,
                collect_items,
                check_hazards,
                respawn_player,
                move_platforms,
                update_ui,
                reset_game,
                animate_player,
                check_level_complete,
            ),
        )
        .run();
}

// ===== COMPONENTS =====
#[derive(Component)]
struct Player {
    speed: f32,
    jump_force: f32,
    is_grounded: bool,
    invulnerable_timer: f32,
    animation_timer: f32,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Collectible {
    value: u32,
}

#[derive(Component)]
struct Hazard;

#[derive(Component)]
struct MovingPlatform {
    start_pos: Vec3,
    end_pos: Vec3,
    speed: f32,
    direction: f32,
}

#[derive(Component)]
struct SpawnPoint;

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct LeftArm;

#[derive(Component)]
struct RightArm;

#[derive(Component)]
struct LeftLeg;

#[derive(Component)]
struct RightLeg;

#[derive(Component)]
struct LevelEntity; // Tag for level-specific entities that should be cleaned up

// ===== RESOURCES =====
#[derive(Resource)]
struct GameState {
    score: u32,
    lives: u32,
    level: u32,
    collectibles_in_level: u32,
}
// ===== ENHANCED SETUP SYSTEM =====
fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_state: Res<GameState>,
) {
    // ===== ENHANCED LIGHTING FOR EXTREME HEIGHTS =====
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000.0, // Much brighter for extreme heights
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 80.0, 0.0), // Higher light source
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // Additional lighting for extreme heights
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            range: 150.0, // Much larger range
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
        ..default()
    });

    // Secondary light for better visibility
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 8000.0,
            range: 100.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(50.0, 50.0, 50.0),
        ..default()
    });

    // ===== ENHANCED CAMERA FOR EXTREME LEVELS =====
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 35.0, 50.0) // Much higher and further back
                .looking_at(Vec3::new(0.0, 15.0, 0.0), Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    // ===== ENHANCED PLAYER WITH BETTER VISIBILITY =====
    let spawn_pos = Vec3::new(0.0, 2.0, 0.0);
    
    let player_entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.3, // Slightly larger for better visibility
                sectors: 12,
                stacks: 12,
            })),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()), // Bright green
            transform: Transform::from_translation(spawn_pos + Vec3::new(0.0, 0.8, 0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(0.4),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Player {
            speed: 8.0,
            jump_force: 12.0,
            is_grounded: false,
            invulnerable_timer: 0.0,
            animation_timer: 0.0,
        },
        SpawnPoint,
        ColliderMassProperties::Density(1.0),
        Friction::coefficient(0.7),
        Restitution::coefficient(0.1),
    )).id();
    
    // Enhanced body parts with brighter colors
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.4,
                height: 1.0,
                resolution: 8,
                segments: 1,
            })),
            material: materials.add(Color::rgb(0.2, 0.6, 1.0).into()), // Bright blue
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        RigidBody::Fixed,
    )).set_parent(player_entity);
    
    // Arms (Left and Right)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.1,
                height: 0.8,
                resolution: 6,
                segments: 1,
            })),
            material: materials.add(Color::rgb(1.0, 0.9, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(-0.6, 0.2, 0.0))
                .with_rotation(Quat::from_rotation_z(0.3)),
            ..default()
        },
        RigidBody::Fixed,
        LeftArm,
    )).set_parent(player_entity);
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.1,
                height: 0.8,
                resolution: 6,
                segments: 1,
            })),
            material: materials.add(Color::rgb(1.0, 0.9, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(0.6, 0.2, 0.0))
                .with_rotation(Quat::from_rotation_z(-0.3)),
            ..default()
        },
        RigidBody::Fixed,
        RightArm,
    )).set_parent(player_entity);
    
    // Legs (Left and Right)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.15,
                height: 1.0,
                resolution: 6,
                segments: 1,
            })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.8).into()), // Bright blue pants
            transform: Transform::from_translation(Vec3::new(-0.2, -0.5, 0.0)),
            ..default()
        },
        RigidBody::Fixed,
        LeftLeg,
    )).set_parent(player_entity);
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.15,
                height: 1.0,
                resolution: 6,
                segments: 1,
            })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(0.2, -0.5, 0.0)),
            ..default()
        },
        RigidBody::Fixed,
        RightLeg,
    )).set_parent(player_entity);

    // ===== GROUND PLATFORM =====
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(25.0, 1.0, 25.0))), // Larger ground
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(12.5, 0.5, 12.5),
    ));

    // ===== SPAWN EXTREME LEVEL CONTENT =====
    spawn_level_content(&mut commands, &mut meshes, &mut materials, game_state.level);

    // ===== ENHANCED UI =====
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "🔥 EXTREME Level: ",
                TextStyle {
                    font_size: 42.0,
                    color: Color::RED,
                    ..default()
                },
            ),
            TextSection::new(
                "1",
                TextStyle {
                    font_size: 42.0,
                    color: Color::CYAN,
                    ..default()
                },
            ),
            TextSection::new(
                " | Score: ",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 40.0,
                    color: Color::GOLD,
                    ..default()
                },
            ),
            TextSection::new(
                " | Lives: ",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "3",
                TextStyle {
                    font_size: 40.0,
                    color: Color::RED,
                    ..default()
                },
            ),
            TextSection::new(
                "\n⚠️ NIGHTMARE DIFFICULTY - Press R to Reset ⚠️",
                TextStyle {
                    font_size: 24.0,
                    color: Color::ORANGE_RED,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        GameUI,
    ));
}
// ===== EXTREME DIFFICULTY LEVEL SYSTEM =====
fn spawn_level_content(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    level: u32,
) {
    // EXTREME difficulty parameters - much more aggressive scaling
    let difficulty_multiplier = 1.0 + (level as f32 - 1.0) * 1.2;
    let platform_size_reducer = 1.0 - (level as f32 - 1.0) * 0.35; // Platforms get MUCH smaller
    let height_multiplier = 1.0 + (level as f32 - 1.0) * 2.5; // EXTREME height increases
    
    println!("🔥 Spawning EXTREME Level {} - Height Multiplier: {:.1}x, Platform Size: {:.1}x", 
             level, height_multiplier, platform_size_reducer.max(0.2));
    
    // Original platforms but made EXTREMELY challenging
    let base_platforms = vec![
        (Vec3::new(8.0, 6.0, -5.0), Vec3::new(4.0, 0.5, 4.0)),    // Higher start
        (Vec3::new(-8.0, 12.0, 0.0), Vec3::new(4.0, 0.5, 4.0)),   // Much higher
        (Vec3::new(2.0, 15.0, 8.0), Vec3::new(4.0, 0.5, 4.0)),    // Very high
        (Vec3::new(12.0, 24.0, 2.0), Vec3::new(4.0, 0.5, 4.0)),   // Extreme height
    ];

    // Add progressively more extreme platforms
    let mut all_platforms = base_platforms;
    
    if level >= 2 {
        all_platforms.extend(vec![
            (Vec3::new(-12.0, 30.0, -8.0), Vec3::new(3.0, 0.5, 3.0)),
            (Vec3::new(16.0, 36.0, -2.0), Vec3::new(2.5, 0.5, 2.5)),
            (Vec3::new(-16.0, 42.0, 6.0), Vec3::new(2.0, 0.5, 2.0)),
        ]);
    }
    
    if level >= 3 {
        all_platforms.extend(vec![
            (Vec3::new(20.0, 48.0, -8.0), Vec3::new(1.8, 0.5, 1.8)),
            (Vec3::new(-20.0, 54.0, 10.0), Vec3::new(1.5, 0.5, 1.5)),
            (Vec3::new(24.0, 60.0, 0.0), Vec3::new(1.2, 0.5, 1.2)),
            (Vec3::new(0.0, 66.0, -15.0), Vec3::new(1.0, 0.5, 1.0)),
            (Vec3::new(-24.0, 72.0, 5.0), Vec3::new(1.0, 0.5, 1.0)),
        ]);
    }
    
    if level >= 4 {
        all_platforms.extend(vec![
            (Vec3::new(28.0, 78.0, -10.0), Vec3::new(0.8, 0.5, 0.8)),  // Tiny platforms
            (Vec3::new(-28.0, 84.0, 12.0), Vec3::new(0.8, 0.5, 0.8)),
            (Vec3::new(32.0, 90.0, -5.0), Vec3::new(0.6, 0.5, 0.6)),   // Nearly impossible
            (Vec3::new(0.0, 96.0, 18.0), Vec3::new(0.6, 0.5, 0.6)),
            (Vec3::new(-32.0, 102.0, -8.0), Vec3::new(0.5, 0.5, 0.5)), // Microscopic
            (Vec3::new(36.0, 108.0, 0.0), Vec3::new(0.4, 0.5, 0.4)),   // Ultimate challenge
        ]);
    }

    // INSANE additional platforms for level 5+
    if level >= 5 {
        all_platforms.extend(vec![
            (Vec3::new(40.0, 114.0, -15.0), Vec3::new(0.3, 0.5, 0.3)), // Pinpoint precision
            (Vec3::new(-40.0, 120.0, 20.0), Vec3::new(0.3, 0.5, 0.3)),
            (Vec3::new(0.0, 126.0, -25.0), Vec3::new(0.25, 0.5, 0.25)), // Nearly invisible
            (Vec3::new(44.0, 132.0, 10.0), Vec3::new(0.25, 0.5, 0.25)),
            (Vec3::new(-44.0, 138.0, -10.0), Vec3::new(0.2, 0.5, 0.2)), // IMPOSSIBLE
        ]);
    }

    // Spawn all platforms with extreme modifications
    for (i, (pos, size)) in all_platforms.iter().enumerate() {
        let adjusted_pos = Vec3::new(pos.x, pos.y * height_multiplier, pos.z);
        let adjusted_size = *size * platform_size_reducer.max(0.15); // Even smaller minimum
        
        // Danger-based coloring
        let color = match level {
            1 => Color::GRAY,
            2 => Color::DARK_GRAY,
            3 => Color::rgb(0.7, 0.3, 0.3),  // Red warning
            4 => Color::rgb(0.9, 0.1, 0.1),  // Danger red
            _ => Color::rgb(0.5, 0.0, 0.5),  // Nightmare purple
        };

        // Make higher platforms more visible with emissive materials
        let material = if adjusted_pos.y > 50.0 {
            materials.add(StandardMaterial {
                base_color: color,
                emissive: color * 0.3, // Glowing effect for high platforms
                ..default()
            })
        } else {
            materials.add(color.into())
        };

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(adjusted_size.x, adjusted_size.y, adjusted_size.z))),
                material,
                transform: Transform::from_translation(adjusted_pos),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(adjusted_size.x / 2.0, adjusted_size.y / 2.0, adjusted_size.z / 2.0),
            LevelEntity,
        ));
        
        // Debug print for extreme platforms
        if adjusted_size.x < 1.0 {
            println!("⚠️  Extreme platform {}: Size {:.2}x{:.2} at height {:.1}", 
                     i + 1, adjusted_size.x, adjusted_size.z, adjusted_pos.y);
        }
    }

    // EXTREME moving platforms - much faster and more challenging
    spawn_extreme_moving_platforms(commands, meshes, materials, level, height_multiplier, platform_size_reducer);
    
    // EXTREME collectibles at nearly impossible locations
    spawn_extreme_collectibles(commands, meshes, materials, level, height_multiplier);
    
    // EXTREME hazards everywhere
    spawn_extreme_hazards(commands, meshes, materials, level, height_multiplier);
}

fn spawn_extreme_moving_platforms(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    level: u32,
    height_multiplier: f32,
    platform_size_reducer: f32,
) {
    let moving_platform_speed = 6.0 + (level as f32 - 1.0) * 3.0; // MUCH faster
    
    // Primary moving platform
    let moving_platform_pos = Vec3::new(-4.0, 20.0 * height_multiplier, -8.0);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                3.0 * platform_size_reducer.max(0.3), 
                0.5, 
                1.5 * platform_size_reducer.max(0.3)
            ))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.0, 1.0),
                emissive: Color::rgb(0.3, 0.0, 0.3), // Glowing magenta
                ..default()
            }),
            transform: Transform::from_translation(moving_platform_pos),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(
            1.5 * platform_size_reducer.max(0.3), 
            0.25, 
            0.75 * platform_size_reducer.max(0.3)
        ),
        MovingPlatform {
            start_pos: moving_platform_pos,
            end_pos: Vec3::new(12.0, moving_platform_pos.y, -8.0), // Longer distance
            speed: moving_platform_speed,
            direction: 1.0,
        },
        LevelEntity,
    ));

    // Additional extreme moving platforms for higher levels
    if level >= 3 {
        let second_platform_pos = Vec3::new(20.0, 50.0 * height_multiplier, 15.0);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    1.5 * platform_size_reducer.max(0.2), 
                    0.5, 
                    1.5 * platform_size_reducer.max(0.2)
                ))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 0.5, 0.0),
                    emissive: Color::rgb(0.3, 0.15, 0.0), // Glowing orange
                    ..default()
                }),
                transform: Transform::from_translation(second_platform_pos),
                ..default()
            },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(
                0.75 * platform_size_reducer.max(0.2), 
                0.25, 
                0.75 * platform_size_reducer.max(0.2)
            ),
            MovingPlatform {
                start_pos: second_platform_pos,
                end_pos: Vec3::new(-20.0, second_platform_pos.y, 15.0),
                speed: moving_platform_speed * 1.8, // Even faster
                direction: 1.0,
            },
            LevelEntity,
        ));
    }

    // INSANE vertical moving platform for level 4+
    if level >= 4 {
        let vertical_platform_pos = Vec3::new(0.0, 80.0 * height_multiplier, 0.0);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    1.0 * platform_size_reducer.max(0.15), 
                    0.5, 
                    1.0 * platform_size_reducer.max(0.15)
                ))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 1.0, 0.0),
                    emissive: Color::rgb(0.4, 0.4, 0.0), // Glowing yellow
                    ..default()
                }),
                transform: Transform::from_translation(vertical_platform_pos),
                ..default()
            },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(
                0.5 * platform_size_reducer.max(0.15), 
                0.25, 
                0.5 * platform_size_reducer.max(0.15)
            ),
            MovingPlatform {
                start_pos: vertical_platform_pos,
                end_pos: Vec3::new(0.0, vertical_platform_pos.y + 30.0, 0.0), // Vertical movement
                speed: moving_platform_speed * 0.8,
                direction: 1.0,
            },
            LevelEntity,
        ));
    }
}
// ===== EXTREME COLLECTIBLES SYSTEM =====
fn spawn_extreme_collectibles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    level: u32,
    height_multiplier: f32,
) {
    let collectible_count = 8 + (level - 1) * 4; // Many more collectibles required
    
    // Base collectible positions - all at extreme heights and locations
    let base_collectible_positions = vec![
        Vec3::new(8.0, 8.0, -5.0),        // On first platform
        Vec3::new(-8.0, 14.5, 0.0),       // On second platform  
        Vec3::new(2.0, 12.5, 8.0),        // On third platform
        Vec3::new(12.0, 26.5, 2.0),       // On fourth platform
        Vec3::new(-4.0, 22.0, -8.0),      // On moving platform area
        Vec3::new(-12.0, 32.0, -8.0),     // Higher challenge
        Vec3::new(16.0, 38.5, -2.0),      // Very high
        Vec3::new(-16.0, 44.5, 6.0),      // Extreme height
    ];
    
    let mut all_collectibles = base_collectible_positions;
    
    // Level 2+ - Add more extreme positions
    if level >= 2 {
        all_collectibles.extend(vec![
            Vec3::new(20.0, 50.5, -8.0),      // On tiny platforms
            Vec3::new(-20.0, 56.5, 10.0),
            Vec3::new(24.0, 62.5, 0.0),
        ]);
    }
    
    // Level 3+ - Add insane positions
    if level >= 3 {
        all_collectibles.extend(vec![
            Vec3::new(0.0, 68.5, -15.0),      // Extreme precision required
            Vec3::new(-24.0, 74.5, 5.0),
            Vec3::new(28.0, 80.5, -10.0),     // On microscopic platforms
            Vec3::new(-28.0, 86.5, 12.0),
        ]);
    }
    
    // Level 4+ - Add nearly impossible positions
    if level >= 4 {
        all_collectibles.extend(vec![
            Vec3::new(32.0, 92.5, -5.0),      // On tiny platforms
            Vec3::new(0.0, 98.5, 18.0),
            Vec3::new(-32.0, 104.5, -8.0),    // Microscopic platform
            Vec3::new(36.0, 110.5, 0.0),      // Ultimate challenge
        ]);
    }
    
    // Level 5+ - Add impossible positions
    if level >= 5 {
        all_collectibles.extend(vec![
            Vec3::new(40.0, 116.5, -15.0),    // Pinpoint platforms
            Vec3::new(-40.0, 122.5, 20.0),
            Vec3::new(0.0, 128.5, -25.0),     // Nearly invisible platforms
            Vec3::new(44.0, 134.5, 10.0),
            Vec3::new(-44.0, 140.5, -10.0),   // IMPOSSIBLE difficulty
        ]);
    }

    // Spawn collectibles with enhanced visibility and value
    for (i, pos) in all_collectibles.iter().take(collectible_count as usize).enumerate() {
        let adjusted_pos = Vec3::new(pos.x, pos.y * height_multiplier, pos.z);
        
        // Increase value based on difficulty and height
        let height_bonus = (adjusted_pos.y / 20.0) as u32;
        let collectible_value = 15 + (level - 1) * 10 + height_bonus;
        
        // Color coding based on difficulty/height
        let (collectible_color, emissive_color) = match (level, adjusted_pos.y as u32) {
            (1..=2, 0..=30) => (Color::rgb(1.0, 1.0, 0.0), Color::rgb(0.3, 0.3, 0.0)), // Yellow
            (1..=2, _) => (Color::rgb(1.0, 0.8, 0.0), Color::rgb(0.3, 0.24, 0.0)), // Gold
            (3..=4, 0..=50) => (Color::rgb(1.0, 0.5, 0.0), Color::rgb(0.3, 0.15, 0.0)), // Orange
            (3..=4, _) => (Color::rgb(1.0, 0.3, 0.0), Color::rgb(0.3, 0.09, 0.0)), // Red-orange
            _ => (Color::rgb(1.0, 0.0, 0.5), Color::rgb(0.3, 0.0, 0.15)), // Pink for nightmare
        };
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5, // Larger for better visibility at extreme heights
                    sectors: 12,
                    stacks: 12,
                })),
                material: materials.add(StandardMaterial {
                    base_color: collectible_color,
                    emissive: emissive_color, // Glowing effect
                    ..default()
                }),
                transform: Transform::from_translation(adjusted_pos),
                ..default()
            },
            RigidBody::Fixed,
            Collider::ball(0.5),
            Sensor,
            Collectible { value: collectible_value },
            LevelEntity,
        ));
        
        // Debug info for extreme collectibles
        if adjusted_pos.y > 100.0 {
            println!("💎 EXTREME Collectible at height {:.1} - Value: {} points", 
                     adjusted_pos.y, collectible_value);
        }
    }
}

// ===== EXTREME HAZARDS SYSTEM =====
fn spawn_extreme_hazards(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    level: u32,
    height_multiplier: f32,
) {
    let hazard_count = 8 + (level - 1) * 3; // Many more hazards
    
    // Ground-level hazards (always present)
    let base_hazard_positions = vec![
        Vec3::new(4.0, 1.0, 4.0),
        Vec3::new(-4.0, 1.0, -4.0),
        Vec3::new(0.0, 1.0, -8.0),
        Vec3::new(10.0, 1.0, -2.0),
        Vec3::new(-10.0, 1.0, 6.0),
        Vec3::new(2.0, 1.0, 10.0),
        Vec3::new(-6.0, 1.0, -10.0),
        Vec3::new(14.0, 1.0, -6.0),
    ];
    
    let mut all_hazards = base_hazard_positions;
    
    // Level 2+ - Add elevated hazards
    if level >= 2 {
        all_hazards.extend(vec![
            Vec3::new(-14.0, 1.0, 8.0),
            Vec3::new(18.0, 1.0, 4.0),
            Vec3::new(6.0, 1.0, -12.0),
            // Elevated hazards on platforms - EXTREME difficulty
            Vec3::new(8.0, 8.0, -3.0),        // Near first platform
            Vec3::new(-8.0, 14.0, 2.0),       // Near second platform
        ]);
    }
    
    // Level 3+ - Add high-altitude hazards
    if level >= 3 {
        all_hazards.extend(vec![
            Vec3::new(-18.0, 1.0, -4.0),
            Vec3::new(22.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 14.0),
            // High-altitude hazards
            Vec3::new(0.0, 20.0 * height_multiplier, 6.0),
            Vec3::new(12.0, 26.0 * height_multiplier, 0.0),
            Vec3::new(-12.0, 32.0 * height_multiplier, -6.0),
        ]);
    }
    
    // Level 4+ - Add extreme high-altitude hazards
    if level >= 4 {
        all_hazards.extend(vec![
            Vec3::new(-22.0, 1.0, 12.0),
            Vec3::new(26.0, 1.0, -8.0),
            // EXTREME high-altitude hazards
            Vec3::new(16.0, 38.0 * height_multiplier, -4.0),
            Vec3::new(-20.0, 56.0 * height_multiplier, 8.0),
            Vec3::new(24.0, 62.0 * height_multiplier, -2.0),
            Vec3::new(0.0, 68.0 * height_multiplier, -13.0),
        ]);
    }
    
    // Level 5+ - Add nightmare hazards everywhere
    if level >= 5 {
        all_hazards.extend(vec![
            Vec3::new(30.0, 1.0, 15.0),
            Vec3::new(-30.0, 1.0, -15.0),
            // NIGHTMARE high-altitude hazards
            Vec3::new(28.0, 80.0 * height_multiplier, -8.0),
            Vec3::new(-28.0, 86.0 * height_multiplier, 10.0),
            Vec3::new(32.0, 92.0 * height_multiplier, -3.0),
            Vec3::new(0.0, 98.0 * height_multiplier, 16.0),
            Vec3::new(-32.0, 104.0 * height_multiplier, -6.0),
        ]);
    }

    // Spawn all hazards with enhanced danger indicators
    for (i, pos) in all_hazards.iter().take(hazard_count as usize).enumerate() {
        let is_elevated = pos.y > 5.0;
        let adjusted_pos = if is_elevated {
            Vec3::new(pos.x, pos.y * height_multiplier, pos.z)
        } else {
            *pos
        };
        
        // Danger level coloring
        let (hazard_color, emissive_color, hazard_size) = match (level, is_elevated) {
            (1..=2, false) => (Color::rgb(1.0, 0.2, 0.2), Color::rgb(0.3, 0.0, 0.0), 2.0), // Red ground hazards
            (1..=2, true) => (Color::rgb(1.0, 0.1, 0.1), Color::rgb(0.4, 0.0, 0.0), 2.2), // Brighter elevated
            (3..=4, false) => (Color::rgb(1.0, 0.0, 0.0), Color::rgb(0.4, 0.0, 0.0), 2.3), // Bright red
            (3..=4, true) => (Color::rgb(1.0, 0.0, 0.0), Color::rgb(0.5, 0.0, 0.0), 2.5), // Very bright elevated
            (_, false) => (Color::rgb(0.8, 0.0, 0.8), Color::rgb(0.3, 0.0, 0.3), 2.5), // Purple nightmare
            (_, true) => (Color::rgb(1.0, 0.0, 1.0), Color::rgb(0.4, 0.0, 0.4), 2.8), // Magenta nightmare elevated
        };
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(hazard_size, 1.5, hazard_size))),
                material: materials.add(StandardMaterial {
                    base_color: hazard_color,
                    emissive: emissive_color, // Glowing danger effect
                    ..default()
                }),
                transform: Transform::from_translation(adjusted_pos),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(hazard_size / 2.0, 0.75, hazard_size / 2.0),
            Sensor,
            Hazard,
            LevelEntity,
        ));
        
        // Debug info for elevated hazards
        if is_elevated && adjusted_pos.y > 50.0 {
            println!("⚠️  EXTREME HAZARD at height {:.1}", adjusted_pos.y);
        }
    }
    
    println!("💀 Level {} spawned with {} hazards ({} elevated)", 
             level, hazard_count, all_hazards.iter().filter(|h| h.y > 5.0).count());
}
// ===== ENHANCED PLAYER MOVEMENT SYSTEMS =====
fn player_movement(
    mut player_query: Query<(&mut Velocity, &mut Player), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut velocity, mut player) in &mut player_query {
        let mut movement = Vec3::ZERO;
        let player_speed = 8.0;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            movement.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            movement.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            movement.z += 1.0;
        }

        if movement.length() > 0.0 {
            movement = movement.normalize() * player_speed;
            velocity.linvel.x = movement.x;
            velocity.linvel.z = movement.z;
            player.animation_timer += 0.1;
        } else {
            velocity.linvel.x = 0.0;
            velocity.linvel.z = 0.0;
            player.animation_timer += 0.02;
        }
    }
}

fn player_jump(
    mut player_query: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut velocity in &mut player_query {
        if keyboard_input.just_pressed(KeyCode::Space) && velocity.linvel.y.abs() < 0.1 {
            velocity.linvel.y = 12.0;
        }
    }
}

// ===== ENHANCED CAMERA SYSTEM FOR EXTREME HEIGHTS =====
fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if let (Ok(mut camera_transform), Ok(player_transform)) =
        (camera_query.get_single_mut(), player_query.get_single())
    {
        // Dynamic camera positioning based on player height and level
        let base_height_offset = 25.0;
        let base_distance_offset = 40.0;
        
        // Scale camera distance based on player height for extreme levels
        let height_factor = (player_transform.translation.y / 30.0).max(0.0);
        let level_factor = (game_state.level as f32 - 1.0) * 0.5;
        
        let height_offset = base_height_offset + (height_factor * 15.0) + (level_factor * 10.0);
        let distance_offset = base_distance_offset + (height_factor * 20.0) + (level_factor * 15.0);
        
        // Smooth camera positioning
        let target_pos = player_transform.translation + Vec3::new(0.0, height_offset, distance_offset);
        camera_transform.translation = camera_transform
            .translation
            .lerp(target_pos, time.delta_seconds() * 2.0);

        // Look at player with slight upward offset for better visibility
        let look_target = player_transform.translation + Vec3::Y * (5.0 + height_factor * 2.0);
        camera_transform.look_at(look_target, Vec3::Y);
        
        // Debug camera info for extreme heights
        if player_transform.translation.y > 80.0 {
            if (time.elapsed_seconds() % 2.0) < 0.1 { // Print every 2 seconds
                println!("📹 Camera adjusted for extreme height: {:.1} (offset: {:.1})", 
                         player_transform.translation.y, height_offset);
            }
        }
    }
}

// ===== ENHANCED PLAYER ANIMATION SYSTEM =====
fn animate_player(
    player_query: Query<&Player>,
    mut left_arm_query: Query<&mut Transform, (With<LeftArm>, Without<RightArm>, Without<LeftLeg>, Without<RightLeg>)>,
    mut right_arm_query: Query<&mut Transform, (With<RightArm>, Without<LeftArm>, Without<LeftLeg>, Without<RightLeg>)>,
    mut left_leg_query: Query<&mut Transform, (With<LeftLeg>, Without<LeftArm>, Without<RightArm>, Without<RightLeg>)>,
    mut right_leg_query: Query<&mut Transform, (With<RightLeg>, Without<LeftArm>, Without<RightArm>, Without<LeftLeg>)>,
) {
    if let Ok(player) = player_query.get_single() {
        let swing_angle = (player.animation_timer * 6.0).sin() * 0.6; // More dramatic animation
        let bounce_factor = (player.animation_timer * 12.0).sin().abs() * 0.1;
        
        if let Ok(mut transform) = left_arm_query.get_single_mut() {
            transform.rotation = Quat::from_rotation_x(swing_angle) * Quat::from_rotation_z(0.3);
            transform.translation = Vec3::new(-0.6, 0.2 + bounce_factor, 0.0);
        }
        
        if let Ok(mut transform) = right_arm_query.get_single_mut() {
            transform.rotation = Quat::from_rotation_x(-swing_angle) * Quat::from_rotation_z(-0.3);
            transform.translation = Vec3::new(0.6, 0.2 + bounce_factor, 0.0);
        }
        
        if let Ok(mut transform) = left_leg_query.get_single_mut() {
            transform.rotation = Quat::from_rotation_x(-swing_angle * 0.8);
            transform.translation = Vec3::new(-0.2, -0.5 + bounce_factor, 0.0);
        }
        
        if let Ok(mut transform) = right_leg_query.get_single_mut() {
            transform.rotation = Quat::from_rotation_x(swing_angle * 0.8);
            transform.translation = Vec3::new(0.2, -0.5 + bounce_factor, 0.0);
        }
    }
}

// ===== ENHANCED GAME LOGIC SYSTEMS =====
fn collect_items(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    player_query: Query<&Transform, With<Player>>,
    collectible_query: Query<(Entity, &Transform, &Collectible), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (collectible_entity, collectible_transform, collectible) in &collectible_query {
            let distance = player_transform.translation.distance(collectible_transform.translation);
            
            if distance < 2.5 { // Slightly larger collection radius for extreme heights
                let old_score = game_state.score;
                game_state.score += collectible.value;
                
                // Enhanced bonus system for extreme difficulty
                if game_state.score % 100 == 0 && game_state.score > 0 {
                    game_state.lives += 1;
                    println!("🌟 MAJOR BONUS! 100 points reached! Lives: {}", game_state.lives);
                } else if game_state.score % 50 == 0 && game_state.score > 0 && old_score % 100 != 0 {
                    game_state.lives += 1;
                    println!("⭐ Bonus life at 50 points! Lives: {}", game_state.lives);
                }
                
                // Special messages for high-value collectibles
                if collectible.value >= 30 {
                    println!("💎 EXTREME collectible worth {} points! Height bonus!", collectible.value);
                }
                
                commands.entity(collectible_entity).despawn();
                break;
            }
        }
    }
}

fn check_level_complete(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<&mut Transform, With<Player>>,
    collectible_query: Query<Entity, With<Collectible>>,
    level_entities: Query<Entity, With<LevelEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if collectible_query.is_empty() {
        let completed_level = game_state.level;
        game_state.level += 1;
        
        // Enhanced level completion messages
        match completed_level {
            1 => println!("🎯 LEVEL 1 CONQUERED! Welcome to the nightmare..."),
            2 => println!("🔥 LEVEL 2 MASTERED! The challenge intensifies!"),
            3 => println!("💀 LEVEL 3 DEMOLISHED! Extreme heights await!"),
            4 => println!("👑 LEVEL 4 ANNIHILATED! You're entering legend territory!"),
            _ => println!("🏆 LEVEL {} OBLITERATED! PLATFORMING DEITY STATUS!", completed_level),
        }
        
        // Clean up current level entities
        for entity in &level_entities {
            commands.entity(entity).despawn();
        }
        
        // Reset player position with fanfare
        for mut transform in &mut player_query {
            transform.translation = Vec3::new(0.0, 2.0, 0.0);
            println!("🚀 Preparing for EXTREME Level {}...", game_state.level);
        }
        
        // Spawn new level content
        spawn_level_content(&mut commands, &mut meshes, &mut materials, game_state.level);
        
        // Enhanced level completion bonuses
        let completion_bonus = match completed_level {
            1..=2 => 1,
            3..=4 => 2,
            _ => 3,
        };
        
        game_state.lives += completion_bonus;
        game_state.score += completed_level * 25; // Completion score bonus
        
        println!("🎁 Level completion bonus: +{} lives, +{} points!", 
                 completion_bonus, completed_level * 25);
        println!("📊 Total: {} points, {} lives", game_state.score, game_state.lives);
    }
}
// ===== ENHANCED HAZARD AND RESPAWN SYSTEMS =====
fn check_hazards(
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&Transform, &mut Player)>,
    hazard_query: Query<&Transform, (With<Hazard>, Without<Player>)>,
    time: Res<Time>,
) {
    for (player_transform, mut player) in &mut player_query {
        if player.invulnerable_timer > 0.0 {
            player.invulnerable_timer -= time.delta_seconds();
        }
        
        if player.invulnerable_timer <= 0.0 {
            for hazard_transform in &hazard_query {
                let distance = player_transform.translation.distance(hazard_transform.translation);
                
                // Increased hazard detection radius for extreme difficulty
                let hazard_radius = if hazard_transform.translation.y > 50.0 { 2.0 } else { 1.8 };
                
                if distance < hazard_radius {
                    if game_state.lives > 0 {
                        game_state.lives -= 1;
                        player.invulnerable_timer = 2.5; // Longer invulnerability for extreme levels
                        
                        // Enhanced hazard hit messages
                        if hazard_transform.translation.y > 80.0 {
                            println!("💀 EXTREME ALTITUDE HAZARD HIT! Lives: {}", game_state.lives);
                        } else if hazard_transform.translation.y > 30.0 {
                            println!("⚠️ Elevated hazard hit! Lives: {}", game_state.lives);
                        } else {
                            println!("🔥 Ground hazard hit! Lives: {}", game_state.lives);
                        }
                        
                        // Game over warning
                        if game_state.lives == 1 {
                            println!("🚨 LAST LIFE! One more mistake and it's over!");
                        } else if game_state.lives == 0 {
                            println!("💀 GAME OVER! Press R to try the extreme challenge again!");
                        }
                    }
                    break;
                }
            }
        }
    }
}

fn respawn_player(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut game_state: ResMut<GameState>,
) {
    for (mut transform, mut velocity) in &mut player_query {
        // Much deeper fall threshold for extreme levels
        if transform.translation.y < -30.0 {
            let fall_height = -transform.translation.y;
            transform.translation = Vec3::new(0.0, 2.0, 0.0);
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
            
            if game_state.lives > 0 {
                game_state.lives -= 1;
                
                // Enhanced fall messages based on height fallen
                match fall_height as u32 {
                    30..=60 => println!("💥 Fell into the abyss! Lives: {}", game_state.lives),
                    61..=100 => println!("🌪️ EXTREME fall from great height! Lives: {}", game_state.lives),
                    101.. => println!("☄️ CATASTROPHIC fall from impossible heights! Lives: {}", game_state.lives),
                    _ => println!("💀 Fell off the world! Lives: {}", game_state.lives),
                }
                
                if game_state.lives == 0 {
                    println!("💀 ELIMINATED! The extreme challenge claims another victim!");
                }
            }
        }
    }
}

// ===== ENHANCED MOVING PLATFORM SYSTEM =====
fn move_platforms(
    mut platform_query: Query<(&mut Transform, &mut MovingPlatform)>,
    time: Res<Time>,
) {
    for (mut transform, mut platform) in &mut platform_query {
        // Enhanced movement with potential vertical movement
        let is_vertical = (platform.end_pos.y - platform.start_pos.y).abs() > 5.0;
        
        let movement = if is_vertical {
            // Vertical moving platform
            Vec3::new(0.0, platform.direction, 0.0) * platform.speed * time.delta_seconds()
        } else {
            // Horizontal moving platform
            Vec3::new(platform.direction, 0.0, 0.0) * platform.speed * time.delta_seconds()
        };
        
        transform.translation += movement;

        // Direction reversal logic
        if is_vertical {
            if platform.direction > 0.0 && transform.translation.y >= platform.end_pos.y {
                platform.direction = -1.0;
            } else if platform.direction < 0.0 && transform.translation.y <= platform.start_pos.y {
                platform.direction = 1.0;
            }
        } else {
            if platform.direction > 0.0 && transform.translation.x >= platform.end_pos.x {
                platform.direction = -1.0;
            } else if platform.direction < 0.0 && transform.translation.x <= platform.start_pos.x {
                platform.direction = 1.0;
            }
        }
    }
}

// ===== ENHANCED UI SYSTEM =====
fn update_ui(
    game_state: Res<GameState>,
    mut ui_query: Query<&mut Text, With<GameUI>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for mut text in &mut ui_query {
        text.sections[1].value = game_state.level.to_string();
        text.sections[3].value = game_state.score.to_string();
        text.sections[5].value = game_state.lives.to_string();
        
        // Dynamic UI color changes based on lives and level
        text.sections[5].style.color = match game_state.lives {
            0 => Color::DARK_GRAY,
            1 => Color::RED,
            2 => Color::ORANGE_RED,
            3..=5 => Color::YELLOW,
            _ => Color::GREEN,
        };
        
        // Level indicator color based on difficulty
        text.sections[1].style.color = match game_state.level {
            1..=2 => Color::CYAN,
            3..=4 => Color::ORANGE,
            _ => Color::RED,
        };
        
        // Add height indicator if player is at extreme height
        if let Ok(player_transform) = player_query.get_single() {
            if player_transform.translation.y > 50.0 {
                text.sections[6].value = format!(
                    "\n🏔️ ALTITUDE: {:.0}m | ⚠️ NIGHTMARE DIFFICULTY - Press R to Reset ⚠️", 
                    player_transform.translation.y
                );
                text.sections[6].style.color = Color::GOLD;
            } else {
                text.sections[6].value = "\n⚠️ NIGHTMARE DIFFICULTY - Press R to Reset ⚠️".to_string();
                text.sections[6].style.color = Color::ORANGE_RED;
            }
        }
    }
}

// ===== ENHANCED RESET SYSTEM =====
fn reset_game(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    level_entities: Query<Entity, With<LevelEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        println!("🔄 RESETTING EXTREME CHALLENGE...");
        println!("📊 Previous session stats:");
        println!("   🏆 Reached Level: {}", game_state.level);
        println!("   💰 Final Score: {}", game_state.score);
        println!("   ❤️ Lives Remaining: {}", game_state.lives);
        
        // Enhanced reset with statistics
        let previous_level = game_state.level;
        let previous_score = game_state.score;
        
        // Reset game state
        game_state.score = 0;
        game_state.lives = 3;
        game_state.level = 1;
        game_state.collectibles_in_level = 8;
        
        // Reset player position and velocity
        for (mut transform, mut velocity) in &mut player_query {
            transform.translation = Vec3::new(0.0, 2.0, 0.0);
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
        }
        
        // Remove all existing level entities
        for entity in &level_entities {
            commands.entity(entity).despawn();
        }
        
        // Respawn level 1 with extreme difficulty
        spawn_level_content(&mut commands, &mut meshes, &mut materials, 1);
        
        // Motivational reset messages
        match (previous_level, previous_score) {
            (1, 0..=50) => println!("🌱 Fresh start! The extreme challenge awaits!"),
            (2..=3, _) => println!("💪 Good attempt! Ready to conquer the heights again?"),
            (4..=5, _) => println!("🎯 Impressive progress! Time to claim the summit!"),
            (_, 200..) => println!("🏆 LEGENDARY performance! One more try for perfection!"),
            _ => println!("🔥 Back to the grind! Show these platforms who's boss!"),
        }
        
        println!("✨ EXTREME CHALLENGE RESET COMPLETE! ✨");
        println!("🎮 Controls: WASD/Arrows to move, Space to jump");
        println!("💎 Collect all items to advance to next level");
        println!("⚠️ Avoid hazards and don't fall into the abyss!");
        println!("🏔️ Each level gets exponentially more difficult!");
    }
}

// ===== GAME OVER DETECTION SYSTEM =====
fn check_game_over(
    game_state: Res<GameState>,
    mut ui_query: Query<&mut Text, With<GameUI>>,
) {
    if game_state.lives == 0 {
        for mut text in &mut ui_query {
            // Flash the UI when game over
            text.sections[0].value = "💀 GAME OVER! 💀 Level: ".to_string();
            text.sections[0].style.color = Color::RED;
            
            // Add game over message to bottom
            if text.sections.len() > 6 {
                text.sections[6].value = format!(
                    "\n💀 ELIMINATED at Level {} with {} points! Press R to try again! 💀", 
                    game_state.level, game_state.score
                );
                text.sections[6].style.color = Color::RED;
            }
        }
    }
}