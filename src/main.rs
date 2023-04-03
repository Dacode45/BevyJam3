use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::*;

mod resources;
use crate::resources::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins) // <- Adds picking, interaction, and highlighting
        .add_startup_system(setup)
        .add_state::<GameState>()
        .add_system(draw_hand.in_schedule(OnEnter(GameState::Game)))
        .add_system(cards_look_at_camera)
        .add_system(card_hover)
        .add_system(card_follow_mouse)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // plane

    for x in 0..8 {
        for y in 0..8 {
            let color = ((x + y) % 2) as f32;
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(color, color, color).into()),
                    transform: Transform::from_xyz(x as f32 - 4.0, 0.0, y as f32 - 4.0),
                    ..default()
                },
                PickableBundle::default(),
            ));
        }
    }

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 12.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PickingCameraBundle::default(),
    ));

    game_state.set(GameState::Game)
}

fn draw_hand(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..5 {
        // cube
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.8, 2.0, 0.1))),
                material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
                transform: Transform::from_xyz(-2.0 + x as f32, 0.0, 4.5),
                ..default()
            },
            PickableBundle::default(),
            Card::default(),
            PlayerOwned,
        ));
    }

    for x in 0..5 {
        // cube
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.8, 2.0, 0.1))),
                material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
                transform: Transform::from_xyz(-2.0 + x as f32, 0.0, -4.5),
                ..default()
            },
            Card::default(),
            PlayerOwned,
        ));
    }
}

fn cards_look_at_camera(
    mut cards: Query<(&Card, &PlayerOwned, &mut Transform)>,
    camera: Query<(&Camera, &Transform, Without<Card>)>,
) {
    let (_, camera_transform, _) = camera.get_single().unwrap();

    for (_, _, mut card_transform) in &mut cards {
        card_transform.look_at(camera_transform.translation, Vec3::Y)
    }
}

fn card_hover(mut cards: Query<(&Card, &PlayerOwned, &Hover, &mut Transform)>) {
    for (_, _, selection, mut trans) in &mut cards {
        if selection.hovered() {
            trans.translation.y = 2.0
        } else {
            trans.translation.y = 0.0
        }
    }
}

#[derive(Component)]
struct Dangle;

fn follow_mouse(
    windows: Query<&Window>,

    mut dangle: Query<(&Dangle, &mut Transform)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, g_trans) = cameras.single();

    let mouse_pos = window.cursor_position().unwrap();
    let world_pos = camera.viewport_to_world(g_trans, mouse_pos).unwrap();
    for (_, mut trans) in &mut dangle {
        trans.translation =
            world_pos.get_point(world_pos.intersect_plane(Vec3::ZERO, Vec3::Y).unwrap())
    }
}

fn card_follow_mouse(
    windows: Query<&Window>,
    mouse_button_input: Res<Input<MouseButton>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut cards: Query<(&Card, &PlayerOwned, &mut Selection, &mut Transform)>,
) {
    let window = windows.single();
    let (camera, g_trans) = cameras.single();

    let world_pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(g_trans, cursor))
        .and_then(|ray| {
            ray.intersect_plane(Vec3::new(0.0, 2.0, 0.0), Vec3::Y)
                .map(|d| ray.get_point(d))
        });

    for (_, _, mut selection, mut trans) in &mut cards {
        if let Some(world_pos) = world_pos {
            if selection.selected() {
                trans.translation = world_pos;
            }
            if mouse_button_input.just_released(MouseButton::Left) {
                selection.set_selected(false);
            }
        }
    }
}
