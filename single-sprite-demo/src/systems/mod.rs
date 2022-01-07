use super::components::Boi;
use bevy::prelude::*;

const BOI_SPEED: f32 = 3.0;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    let bottom = -window.height() / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("boi.png").into()),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 2. + 5., 10.),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(Boi);
}

pub fn move_boi(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, With<Boi>)>,
) {
    for (mut transform, _) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= BOI_SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += BOI_SPEED;
        }
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += BOI_SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= BOI_SPEED;
        }
    }
}
