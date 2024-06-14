use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_player)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Still,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("penguin.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Direction::East,
    ));
}

fn move_player(
    mut players: Query<(&mut Transform, &mut Direction), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut direction) in &mut players {
        let mut dir = Vec2::ZERO;
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            dir.y += 1.;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            dir.y -= 1.;
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            dir.x += 1.;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            dir.x -= 1.;
        }

        let move_speed = 30.;
        let move_delta = dir * move_speed * time.delta_seconds();
        transform.translation += move_delta.extend(0.);

        match dir {
            Vec2 { x: 1., .. } => *transform = transform.with_rotation(Quat::from_rotation_y(PI)),
            Vec2 { x: -1., .. } => *transform = transform.with_rotation(Quat::from_rotation_y(0.)),
            _ => {}
        }

        *direction = match dir {
            Vec2 { x: 0., y: 1. } => Direction::North,
            Vec2 { x: 0., y: -1. } => Direction::South,
            Vec2 { x: 1., y: 0. } => Direction::East,
            Vec2 { x: -1., y: 0. } => Direction::West,
            Vec2 { x: 1., y: 1. } => Direction::NorthEast,
            Vec2 { x: -1., y: 1. } => Direction::NorthWest,
            Vec2 { x: 1., y: -1. } => Direction::SouthEast,
            Vec2 { x: -1., y: -1. } => Direction::SouthWest,
            Vec2 { x: 0., y: 0. } => Direction::Still,
            _ => Direction::Still,
        };

        println!("{:?}", direction);
    }
}
