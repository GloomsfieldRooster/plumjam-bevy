use bevy::render::{
    camera::Camera,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use std::env;

use {bevy::prelude::*, input_plugin::*};

mod input_plugin;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_event::<InputUpdateEvent>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            InputPlugin,
        ))
        .insert_resource(Inputs {
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, player_move)
        .run();
}

#[derive(Component)]
struct Wisteria;

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    asset_server: Res<AssetServer>,
) {
    let ui_texture_size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let mut ui_image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: ui_texture_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // clear the UI image
    ui_image.resize(ui_texture_size);

    let ui_texture_camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                order: 2,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
            TargetCamera(ui_texture_camera),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Testing",
                TextStyle {
                    font_size: 40.0,
                    color: Color::BLACK,
                    ..default()
                },
            ));
        });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.841, 5.0)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("wisteria.png")),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                std::f32::consts::FRAC_PI_2 * 3.0 / 2.0,
            )),
            ..default()
        },
        Wisteria,
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}

fn player_move(
    time: Res<Time>,
    inputs: Res<Inputs>,

    mut input_update_event: EventReader<InputUpdateEvent>,

    mut query: Query<&mut Transform, With<Wisteria>>,
) {
    let mut player_transform = query.single_mut();

    if inputs.direction_up.1 {
        player_transform.translation += Vec3 {
            x: 0.0,
            y: 0.0,
            z: inputs.direction_up.0,
        } * time.delta_seconds();
    }

    if inputs.direction_down.1 {
        player_transform.translation += Vec3 {
            x: 0.0,
            y: 0.0,
            z: inputs.direction_down.0,
        } * time.delta_seconds();
    }

    if inputs.direction_left.1 {
        player_transform.translation -= Vec3 {
            x: inputs.direction_left.0,
            y: 0.0,
            z: 0.0,
        } * time.delta_seconds();
    }

    if inputs.direction_right.1 {
        player_transform.translation += Vec3 {
            x: inputs.direction_right.0,
            y: 0.0,
            z: 0.0,
        } * time.delta_seconds();
    }

    for input in input_update_event.read() {
        match input.input_event {
            InputMappable::DirectionUp => {
                player_transform.translation += Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: inputs.direction_up.0,
                } * time.delta_seconds();
            }
            InputMappable::DirectionDown => {
                player_transform.translation -= Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: inputs.direction_down.0,
                } * time.delta_seconds();
            }
            InputMappable::DirectionLeft => {
                player_transform.translation -= Vec3 {
                    x: inputs.direction_left.0,
                    y: 0.0,
                    z: 0.0,
                } * time.delta_seconds();
            }
            InputMappable::DirectionRight => {
                player_transform.translation += Vec3 {
                    x: inputs.direction_right.0,
                    y: 0.0,
                    z: 0.0,
                } * time.delta_seconds();
            }
            _ => {}
        }
    }
}
