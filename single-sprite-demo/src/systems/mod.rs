use super::components::Boi;
use bevy::prelude::*;

const BOI_SPEED: f32 = 3.0;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn setup_boi(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let boi = commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("boi.png").into()),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Boi)
        .id();

    let tear_spawn_1 = commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            transform: Transform {
                translation: Vec3::new(-14.0, 0.0, 11.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let tear_spawn_2 = commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            transform: Transform {
                translation: Vec3::new(14.0, 0.0, 11.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .entity(boi)
        .push_children(&[tear_spawn_1, tear_spawn_2]);
}

pub fn move_boi(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, With<Boi>)>,
) {
    for (mut transform, _) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= BOI_SPEED;
        } else if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += BOI_SPEED;
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += BOI_SPEED;
        } else if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= BOI_SPEED;
        }
    }
}
