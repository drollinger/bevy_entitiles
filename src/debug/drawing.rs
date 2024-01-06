use bevy::{ecs::system::Query, gizmos::gizmos::Gizmos, math::Vec2, render::color::Color};

use crate::{
    math::aabb::Aabb2d,
    tilemap::map::{TilePivot, TilemapSlotSize, TilemapStorage, TilemapTransform, TilemapType},
};

#[cfg(feature = "algorithm")]
use crate::algorithm::pathfinding::Path;

#[cfg(feature = "debug")]
pub fn draw_chunk_aabb(
    mut gizmos: Gizmos,
    tilemaps: Query<(
        &TilemapType,
        &TilePivot,
        &TilemapSlotSize,
        &TilemapTransform,
        &TilemapStorage,
    )>,
) {
    for (ty, tile_pivot, tile_slot_size, transform, storage) in tilemaps.iter() {
        storage.storage.chunks.keys().for_each(|chunk| {
            let aabb = Aabb2d::from_tilemap(
                *chunk,
                storage.storage.chunk_size,
                *ty,
                tile_pivot.0,
                tile_slot_size.0,
                *transform,
            );
            gizmos.rect_2d(
                aabb.center(),
                0.,
                Vec2::new(aabb.width(), aabb.height()),
                Color::GREEN,
            );
        });
    }
}

#[cfg(feature = "algorithm")]
pub fn draw_path(
    mut gizmos: Gizmos,
    path_query: Query<&Path>,
    tilemaps: Query<(
        &TilemapType,
        &TilemapTransform,
        &TilePivot,
        &TilemapSlotSize,
    )>,
) {
    for path in path_query.iter() {
        let (ty, transform, pivot, slot_size) = tilemaps.get(path.get_target_tilemap()).unwrap();

        for node in path.iter() {
            gizmos.circle_2d(
                crate::tilemap::coordinates::index_to_world(*node, ty, transform, pivot, slot_size),
                10.,
                Color::YELLOW_GREEN,
            );
        }
    }
}

pub fn draw_axis(mut gizmos: Gizmos) {
    gizmos.line_2d(Vec2::NEG_X * 1e10, Vec2::X * 1e10, Color::RED);
    gizmos.line_2d(Vec2::NEG_Y * 1e10, Vec2::Y * 1e10, Color::GREEN);
}
