use {bevy::prelude::*, input_plugin::*};

mod input_plugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                ImagePlugin::default_nearest()),
            InputPlugin))
        .add_systems(Startup, setup)
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
