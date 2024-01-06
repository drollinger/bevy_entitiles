use bevy::{
    app::{App, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    input::{keyboard::KeyCode, Input},
    math::{IVec2, UVec2, Vec2, Vec3Swizzles},
    reflect::Reflect,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::HashMap,
    DefaultPlugins,
};
use bevy_entitiles::{
    algorithm::wfc::{LdtkWfcMode, WfcRules, WfcRunner, WfcSource},
    ldtk::{
        layer::physics::LdtkPhysicsLayer,
        resources::{LdtkLevelManager, LdtkPatterns, LdtkWfcManager},
    },
    math::TileArea,
    tilemap::tile::TilemapType,
    EntiTilesPlugin,
};
use bevy_xpbd_2d::plugins::{debug::PhysicsDebugConfig, PhysicsDebugPlugin, PhysicsPlugins};
use helpers::EntiTilesHelpersPlugin;

mod helpers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EntiTilesPlugin,
            EntiTilesHelpersPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(LdtkLevelManager::new(
            "assets/ldtk/wfc_source.ldtk".to_string(),
            "ldtk/".to_string(),
        ))
        .insert_resource(LdtkPatterns::new(
            (0..=5)
                .into_iter()
                .map(|i| (i, format!("World_Level_{}", i)))
                .collect(),
        ))
        .insert_resource(PhysicsDebugConfig::all())
        .add_systems(Startup, setup)
        .add_systems(Update, (player_control, load_level))
        .register_type::<Player>()
        .run();
}

#[derive(Component, Reflect)]
struct Player {
    pub level: IVec2,
}

#[derive(Component, Reflect)]
struct LevelChange(UVec2);

fn setup(mut commands: Commands, mut manager: ResMut<LdtkLevelManager>) {
    commands.spawn(Camera2dBundle::default());

    manager
        .set_physics_layer(LdtkPhysicsLayer {
            identifier: "PhysicsCollider".to_string(),
            air_value: 0,
            parent: "Patterns".to_string(),
            frictions: Some(HashMap::from([(1, 0.5), (2, 0.8)])),
        })
        .load_all_patterns(&mut commands);

    let rules = WfcRules::from_file("examples/ldtk_wfc_config.ron", TilemapType::Square);
    commands.spawn((
        WfcRunner::new(
            TilemapType::Square,
            rules,
            TileArea::new(IVec2::ZERO, UVec2 { x: 4, y: 4 }),
            None,
        ),
        WfcSource::LdtkMapPattern(LdtkWfcMode::SingleMap),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2 { x: 8., y: 8. }),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        },
        Player { level: IVec2::ZERO },
    ));

    commands.spawn(LevelChange(UVec2::ZERO));
}

fn player_control(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else {
        return;
    };
    if input.pressed(KeyCode::Left) {
        transform.translation.x -= 1.;
    }
    if input.pressed(KeyCode::Right) {
        transform.translation.x += 1.;
    }
    if input.pressed(KeyCode::Up) {
        transform.translation.y += 1.;
    }
    if input.pressed(KeyCode::Down) {
        transform.translation.y -= 1.;
    }

    // 8. * 16. = tile size * pattern size
    let new_idx = (transform.translation.xy() / Vec2::splat(8. * 16.)).as_ivec2();
    if new_idx != player.level {
        player.level = new_idx;
        if new_idx.x >= 0 && new_idx.y >= 0 {
            commands.spawn(LevelChange(new_idx.as_uvec2()));
        }
    }
}

fn load_level(
    mut commands: Commands,
    query: Query<(Entity, &LevelChange)>,
    mut level_manager: ResMut<LdtkLevelManager>,
    wfc_manager: Res<LdtkWfcManager>,
) {
    query.iter().for_each(|(e, l)| {
        if let Some(ident) = wfc_manager.get_ident(l.0) {
            level_manager.switch_to(
                &mut commands,
                ident,
                Some(wfc_manager.get_translation(l.0.as_ivec2())),
            );
        }
        commands.entity(e).despawn_recursive();
    });
}
