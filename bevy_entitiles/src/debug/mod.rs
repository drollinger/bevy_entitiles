use bevy::{
    ecs::entity::Entity,
    math::{UVec2, Vec2},
};

use crate::{
    math::aabb::AabbBox2d,
    render::{extract::ExtractedTilemap, texture::TilemapTexture, uniform::TileAnimation},
    tilemap::{map::Tilemap, tile::TileType},
    MAX_ANIM_COUNT,
};

pub struct PubTilemap {
    pub id: Entity,
    pub tile_type: TileType,
    pub size: UVec2,
    pub tile_render_scale: Vec2,
    pub tile_slot_size: Vec2,
    pub pivot: Vec2,
    pub render_chunk_size: u32,
    pub texture: Option<TilemapTexture>,
    pub tiles: Vec<Option<Entity>>,
    pub aabb: AabbBox2d,
    pub translation: Vec2,
    pub z_order: i32,
    pub anim_seqs: [TileAnimation; MAX_ANIM_COUNT],
}

impl PubTilemap {
    pub fn from_tilemap(value: &Tilemap) -> Self {
        Self {
            id: value.id,
            tile_type: value.tile_type,
            size: value.size,
            tile_render_scale: value.tile_render_scale,
            tile_slot_size: value.tile_slot_size,
            pivot: value.pivot,
            render_chunk_size: value.render_chunk_size,
            texture: value.texture.clone(),
            tiles: value.tiles.clone(),
            aabb: value.aabb,
            translation: value.translation,
            z_order: value.z_order,
            anim_seqs: value.anim_seqs,
        }
    }

    pub fn from_extracted_tilemap(value: ExtractedTilemap) -> Self {
        Self {
            id: value.id,
            tile_type: value.tile_type,
            size: value.size,
            tile_render_scale: value.tile_render_scale,
            tile_slot_size: value.tile_slot_size,
            pivot: value.pivot,
            render_chunk_size: value.render_chunk_size,
            texture: value.texture,
            tiles: vec![],
            aabb: value.aabb,
            translation: value.translation,
            z_order: value.z_order,
            anim_seqs: value.anim_seqs,
        }
    }

    pub fn into_extracted_tilemap(self) -> ExtractedTilemap {
        ExtractedTilemap {
            id: self.id,
            tile_type: self.tile_type,
            size: self.size,
            tile_render_scale: self.tile_render_scale,
            tile_slot_size: self.tile_slot_size,
            pivot: self.pivot,
            render_chunk_size: self.render_chunk_size,
            texture: self.texture,
            translation: self.translation,
            aabb: self.aabb,
            z_order: self.z_order,
            anim_seqs: self.anim_seqs,
            time: 0.,
        }
    }
}