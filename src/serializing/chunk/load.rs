use std::{collections::VecDeque, path::Path};

use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, ParallelCommands, Query, Res, ResMut, Resource},
    },
    hierarchy::BuildChildren,
    math::IVec2,
    reflect::Reflect,
    utils::{EntityHashMap, HashMap},
};

use crate::{
    math::extension::ChunkIndex,
    serializing::{load_object, map::TilemapLayer},
    tilemap::{
        buffers::TileBuilderBuffer,
        map::{TilemapName, TilemapStorage},
        tile::Tile,
    },
};

#[cfg(feature = "algorithm")]
use super::PATH_TILE_CHUNKS_FOLDER;
use super::TILE_CHUNKS_FOLDER;
#[cfg(feature = "algorithm")]
use crate::tilemap::{algorithm::path::PathTilemap, buffers::PathTilesBuffer};

#[derive(Component)]
pub struct ScheduledLoadChunks;

#[derive(Resource, Default, Reflect)]
pub struct ChunkLoadConfig {
    pub path: String,
    pub chunks_per_frame: usize,
}

#[derive(Resource, Default)]
pub struct ChunkLoadCache(pub(crate) EntityHashMap<Entity, HashMap<TilemapLayer, VecDeque<IVec2>>>);

impl ChunkLoadCache {
    #[inline]
    pub fn schedule(
        &mut self,
        commands: &mut Commands,
        tilemap: Entity,
        layer: TilemapLayer,
        chunk_index: IVec2,
    ) {
        self.0
            .entry(tilemap)
            .or_default()
            .entry(layer)
            .or_default()
            .push_front(chunk_index);
        commands.entity(tilemap).insert(ScheduledLoadChunks);
    }

    #[inline]
    pub fn schedule_many(
        &mut self,
        commands: &mut Commands,
        tilemap: Entity,
        layer: TilemapLayer,
        chunk_indices: impl Iterator<Item = IVec2>,
    ) {
        let queue = self.0.entry(tilemap).or_default().entry(layer).or_default();
        queue.reserve(chunk_indices.size_hint().0);
        chunk_indices.for_each(|chunk_index| queue.push_front(chunk_index));
        commands.entity(tilemap).insert(ScheduledLoadChunks);
    }

    #[inline]
    pub fn pop_chunk(&mut self, tilemap: Entity, layer: TilemapLayer) -> Option<IVec2> {
        self.0
            .get_mut(&tilemap)
            .map(|layers| {
                layers
                    .get_mut(&layer)
                    .map(|chunks| chunks.pop_back())
                    .flatten()
            })
            .flatten()
    }
}

pub fn load_color_layer(
    commands: ParallelCommands,
    mut tilemaps_query: Query<
        (Entity, &TilemapName, &mut TilemapStorage),
        With<ScheduledLoadChunks>,
    >,
    config: Res<ChunkLoadConfig>,
    mut cache: ResMut<ChunkLoadCache>,
) {
    tilemaps_query.for_each_mut(|(entity, name, mut storage)| {
        let chunk_size = storage.storage.chunk_size as i32;
        (0..config.chunks_per_frame).into_iter().for_each(|_| {
            let Some(chunk_index) = cache.pop_chunk(entity, TilemapLayer::Color) else {
                cache
                    .0
                    .get_mut(&entity)
                    .unwrap()
                    .remove(&TilemapLayer::Color);
                return;
            };

            let Ok(chunk) = load_object::<TileBuilderBuffer>(
                &Path::new(&config.path)
                    .join(&name.0)
                    .join(TILE_CHUNKS_FOLDER),
                format!("{}.ron", chunk_index.chunk_file_name()).as_str(),
            ) else {
                return;
            };

            commands.command_scope(|mut c| {
                let mut tiles = Vec::with_capacity((chunk_size * chunk_size) as usize);
                let mut entities = vec![None; (chunk_size * chunk_size) as usize];
                let chunk_origin = chunk_index * chunk_size;
                chunk.tiles.into_iter().for_each(|(in_chunk_index, tile)| {
                    let e = c.spawn_empty().id();
                    let in_chunk_index_vec =
                        (in_chunk_index.x + in_chunk_index.y * chunk_size) as usize;

                    tiles.push((
                        e,
                        Tile {
                            tilemap_id: entity,
                            chunk_index,
                            in_chunk_index: in_chunk_index_vec,
                            index: chunk_origin + in_chunk_index,
                            texture: tile.texture,
                            color: tile.color,
                        },
                    ));
                    entities[in_chunk_index_vec] = Some(e);
                });

                c.entity(entity)
                    .push_children(&entities.iter().filter_map(|e| *e).collect::<Vec<_>>());
                storage.storage.set_chunk(chunk_index, entities);
                c.insert_or_spawn_batch(tiles);
            });
        });
    });
}

#[cfg(feature = "algorithm")]
pub fn load_path_layer(
    tilemaps_query: Query<(Entity, &TilemapName, &PathTilemap), With<ScheduledLoadChunks>>,
    config: Res<ChunkLoadConfig>,
    mut cache: ResMut<ChunkLoadCache>,
) {
    tilemaps_query.for_each(|(entity, name, path_tilemap)| {
        let chunk_size = path_tilemap.storage.chunk_size as i32;
        (0..config.chunks_per_frame).into_iter().for_each(|_| {
            let Some(chunk_index) = cache.pop_chunk(entity, TilemapLayer::Path) else {
                cache
                    .0
                    .get_mut(&entity)
                    .unwrap()
                    .remove(&TilemapLayer::Path);
                return;
            };

            let Ok(chunk) = load_object::<PathTilesBuffer>(
                &Path::new(&config.path)
                    .join(&name.0)
                    .join(PATH_TILE_CHUNKS_FOLDER),
                format!("{}.ron", chunk_index.chunk_file_name()).as_str(),
            ) else {
                return;
            };

            let mut c = vec![None; (chunk_size * chunk_size) as usize];
            chunk.tiles.into_iter().for_each(|(in_chunk_index, tile)| {
                c[(in_chunk_index.y * chunk_size + in_chunk_index.x) as usize] = Some(tile);
            });
            path_tilemap.storage.get_chunk(chunk_index).replace(&c);
        });
    });
}