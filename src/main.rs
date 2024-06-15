use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::Rng;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(600., 600.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .insert_resource(Score { value: 0 })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player_move, score_display))
        .add_systems(PostUpdate, item_interact)
        .run();
}

#[derive(Resource)]
struct Score {
    value: u32,
}

#[derive(Component)]
struct Player;

#[derive(Default, Component, Debug)]
enum Direction {
    North,
    South,
    #[default]
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Still,
}

#[derive(Component)]
struct Item;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let font = asset_server.load("fonts/NotJamChunky8.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 15.0,
        color: Color::ANTIQUE_WHITE,
    };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Text2dBundle {
        text: Text::from_sections([
            TextSection::new("Where fish?\n\n", text_style.clone()),
            TextSection::new(format!("Score: {}", score.value), text_style.clone()),
        ]),
        transform: Transform::from_xyz(-200., 200., 0.),
        ..default()
    },));
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("penguin2.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        Direction::East,
    ));
    commands.spawn((
        Item,
        SpriteBundle {
            texture: asset_server.load("fish.png"),
            transform: Transform::from_xyz(100., 100., 0.),
            ..default()
        },
    ));
}

fn score_display(score: Res<Score>, mut text: Query<&mut Text>) {
    let mut text = text.single_mut();
    text.sections[1].value = format!("Score: {}", score.value);
}

fn player_move(
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

        let move_speed = 100.;
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

        // println!("{:?}", direction);
    }
}

fn item_interact(
    mut commands: Commands,
    items: Query<(Entity, &Transform), With<Item>>,
    players: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    let mut rng = rand::thread_rng();
    let player_transform = players.single();

    for (item, transform) in &items {
        // println!(
        //     "player @ {:?}, item @ {:?}",
        //     player_transform.translation, transform.translation
        // );
        if player_transform
            .translation
            .abs_diff_eq(transform.translation, 10.)
        {
            commands.entity(item).despawn();
            let x = (rng.gen::<f32>() - 0.5) * 400.;
            let y = (rng.gen::<f32>() - 0.5) * 400.;
            commands.spawn((
                Item,
                SpriteBundle {
                    texture: asset_server.load("fish.png"),
                    transform: Transform::from_xyz(x, y, 0.),
                    ..default()
                },
            ));
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/bubble.ogg"),
                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(1.)),
            });
            score.value = score.value + 1;
        }
    }
}
