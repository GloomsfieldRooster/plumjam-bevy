use {bevy::prelude::*, input_plugin::*};
use std::env;

mod input_plugin;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_event::<InputUpdateEvent>()
        .add_plugins((
            DefaultPlugins.set(
                ImagePlugin::default_nearest()),
            InputPlugin))
        .insert_resource(Inputs { ..Default::default() })
        .add_systems(Startup, setup)
        .add_systems(Update, player_move)
        .run();
}

#[derive(Component)]
struct Wisteria;

#[derive(Component)]
struct Camera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    asset_server: Res<AssetServer>
) {
    commands.spawn((
            PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.841, 5.0)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("wisteria.png")),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2 * 3.0 / 2.0)),
            ..default()
        }, 
        Wisteria
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Camera
    ));
}

fn player_move(
    time: Res<Time>,
    inputs: Res<Inputs>,

    mut input_update_event: EventReader<InputUpdateEvent>,

    mut query: Query<&mut Transform, With<Wisteria>>
) {
    let mut player_transform = query.single_mut();

    if inputs.direction_up.1 {
        player_transform.translation += Vec3 { x: 0.0, y: 0.0, z: inputs.direction_up.0 } * time.delta_seconds();
    }

    if inputs.direction_down.1 {
        player_transform.translation += Vec3 { x: 0.0, y: 0.0, z: inputs.direction_down.0 } * time.delta_seconds();
    }

    if inputs.direction_left.1 {
        player_transform.translation -= Vec3 { x: inputs.direction_left.0, y: 0.0, z: 0.0 } * time.delta_seconds();
    }

    if inputs.direction_right.1 {
        player_transform.translation += Vec3 { x: inputs.direction_right.0, y: 0.0, z: 0.0 } * time.delta_seconds();
    }

    for input in input_update_event.read() {
        match input.input_event {
            InputMappable::DirectionUp => {
                player_transform.translation += Vec3 { x: 0.0, y: 0.0, z: inputs.direction_up.0 } * time.delta_seconds();
            }
            InputMappable::DirectionDown => {
                player_transform.translation -= Vec3 { x: 0.0, y: 0.0, z: inputs.direction_down.0 } * time.delta_seconds();
            }
            InputMappable::DirectionLeft => {
                player_transform.translation -= Vec3 { x: inputs.direction_left.0, y: 0.0, z: 0.0 } * time.delta_seconds();
            }
            InputMappable::DirectionRight => {
                player_transform.translation += Vec3 { x: inputs.direction_right.0, y: 0.0, z: 0.0 } * time.delta_seconds();
            }
            _ => { }
        }
    }
}
