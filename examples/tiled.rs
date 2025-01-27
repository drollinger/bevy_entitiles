use bevy::{
    app::{App, PluginGroup, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    reflect::Reflect,
    render::{color::Color, texture::ImagePlugin},
    DefaultPlugins,
};
use bevy_entitiles::{
    tiled::{
        app_ext::TiledApp,
        resources::{TiledLoadConfig, TiledTilemapManger},
    },
    EntiTilesPlugin,
};
use bevy_entitiles_derive::{TiledClass, TiledEnum, TiledObject};
use bevy_xpbd_2d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use helpers::EntiTilesHelpersPlugin;

mod helpers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EntiTilesPlugin,
            EntiTilesHelpersPlugin::default(),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, switching)
        .insert_resource(TiledLoadConfig {
            map_path: vec![
                "assets/tiled/tilemaps/hexagonal.tmx".to_string(),
                "assets/tiled/tilemaps/infinite.tmx".to_string(),
                "assets/tiled/tilemaps/orthogonal.tmx".to_string(),
                "assets/tiled/tilemaps/isometric.tmx".to_string(),
            ],
            ignore_unregisterd_objects: true,
        })
        .register_tiled_object::<BlockBundle>("Block")
        .register_tiled_object::<PlainBlockBundle>("PlainBlock")
        .register_tiled_object::<PlayerBundle>("Player")
        .register_tiled_object::<DetectAreaBundle>("DetectArea")
        .register_type::<Block>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

macro_rules! map_switching {
    ($key:ident, $map:expr, $input:expr, $manager:expr, $commands:expr) => {
        if $input.just_pressed(KeyCode::$key) {
            $manager.switch_to(&mut $commands, $map.to_string(), None);
        }
    };
}

fn switching(
    mut commands: Commands,
    mut manager: ResMut<TiledTilemapManger>,
    input: Res<ButtonInput<KeyCode>>,
) {
    map_switching!(Digit1, "hexagonal", input, manager, commands);
    map_switching!(Digit2, "infinite", input, manager, commands);
    map_switching!(Digit3, "orthogonal", input, manager, commands);
    map_switching!(Digit4, "isometric", input, manager, commands);
}

/*
 * Here many macro attributes are the same as LDtk's.
 * So if you want to know what they do, you can go to examples/ldtk.rs.
 */

#[derive(TiledObject, Bundle)]
#[spawn_sprite]
pub struct PlainBlockBundle {
    // You have to use `TiledClass`es for objects.
    // Individual properties are not allowed and will cause panic.
    pub block: PlainBlock,
}

#[derive(TiledClass, Component)]
pub struct PlainBlock;

#[derive(TiledObject, Bundle)]
#[spawn_sprite]
pub struct BlockBundle {
    pub block: Block,
}

#[derive(TiledClass, Component, Reflect)]
pub struct Block {
    #[tiled_name = "Collision"]
    pub collision: bool,
    #[tiled_name = "Hardness"]
    pub hardness: f32,
    #[tiled_name = "Name"]
    pub name: String,
    #[tiled_name = "Tint"]
    pub tint: Color,
    #[tiled_name = "Shape"]
    pub shape: ShapeType,
}

#[derive(TiledEnum, Reflect)]
pub enum ShapeType {
    Square,
    Isometry,
    Hexagon,
    Polygon,
    Eclipse,
}

#[derive(TiledObject, Bundle)]
#[spawn_sprite]
#[global_object]
// Generate the collider according to the shape.
// The won't spawn with rigidbody or friction.
#[shape_as_collider]
pub struct PlayerBundle {
    pub player: Player,
    pub moveable: MoveableObject,
}

#[derive(TiledClass, Component, Default)]
pub struct Player {
    #[tiled_name = "Hp"]
    pub hp: f32,
    #[tiled_default]
    pub level: i32,
}

#[derive(TiledClass, Component)]
pub struct MoveableObject {
    #[tiled_name = "Speed"]
    pub speed: f32,
}

#[derive(TiledObject, Bundle)]
#[shape_as_collider]
pub struct DetectAreaBundle {
    pub detect_area: DetectArea,
}

#[derive(TiledClass, Component)]
pub struct DetectArea;
