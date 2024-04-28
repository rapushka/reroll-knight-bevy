use bevy::prelude::*;
use crate::*;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Bootstrap), (
                bootstrap,
                spawn_camera,
                spawn_light,
            ))

        // .add_systems(OnEnter(AppState::Bootstrap), bootstrap)
        ;
    }
}

pub fn bootstrap(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    build_loading_curtain(&mut commands, &asset_server);
}

#[derive(Component)]
pub struct LoadingCurtain {}

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn spawn_light(
    mut commands: Commands,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub fn build_loading_curtain(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands.spawn((
        Name::new("loading curtain"),
        LoadingCurtain {},
        NodeBundle {
            style: constants::styles::MAIN_MENU,
            background_color: Color::BLACK.into(),
            ..default()
        },
    ))
        .with_children(|parent| {
            create_text(asset_server, "Loading...", parent, constants::FONT_SIZE);
        })
        .id()
}

fn create_text(asset_server: &Res<AssetServer>, text: &str, parent: &mut ChildBuilder, font_size: f32) {
    parent.spawn((
        Name::new(format!("text: {text}")),
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        text,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size,
                            color: Color::WHITE,
                        },
                    )],
                justify: JustifyText::Center,
                ..default()
            },
            ..default()
        },
    ));
}
