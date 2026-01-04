use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(512, 512),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup_level)
        .run();
}

const PIXEL_RATIO: f32 = 4.0;

#[derive(Component)]
struct Bird {
    pub velocity: f32,
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            image: asset_server.load("bird.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Bird { velocity: 0. },
    ));
}
