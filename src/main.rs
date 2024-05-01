use bevy::prelude::*;
use bevy::window::close_on_esc;

const WINDOW_HEIGHT: f32 = 1920.0;
const WINDOW_WIDTH: f32 = 1080.0;

const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SHEET_SCALE_FACTOR: f32 = 3.0;
const TILE_WIDTH: usize = 16;
const TILE_HEIGHT: usize = 16;
const SPRITE_SHEET_WIDTH: usize = 4;
const SPRITE_SHEET_HEIGHT: usize = 4;

const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_HEIGHT, WINDOW_WIDTH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(SPRITE_SHEET_PATH);
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32),
        SPRITE_SHEET_WIDTH,
        SPRITE_SHEET_HEIGHT,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation

    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteSheetBundle {
        transform: Transform::from_scale(Vec3::splat(SPRITE_SHEET_SCALE_FACTOR)),
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        ..default()
    },));
}
