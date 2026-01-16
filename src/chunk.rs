use crate::block::block_data::BlockData;
use crate::block::get_block_logic;
use crate::block::BlockType;
use crate::world::{CHUNK_SIZE, GEN_DEPTH};

use std::collections::HashMap;
use std::collections::LinkedList;
use bevy::prelude::*;

const MAX_HEIGHT:u8 = 64;

pub struct Chunk{
    pub blocks:HashMap<(u8,u8,u8), BlockData>,
    pub has_spawned:bool
}
impl Chunk {
    pub fn new() -> Chunk{
        let mut blocks:HashMap<(u8,u8,u8), BlockData> = HashMap::with_capacity(CHUNK_SIZE as usize*CHUNK_SIZE as usize*MAX_HEIGHT as usize);
        Chunk{blocks, has_spawned:false}
    }
    pub fn get_scene_bundle(&mut self, asset_server: &Res<AssetServer>, offset_x:i64, offset_y:i64) -> LinkedList<(SceneRoot, Transform)>{
        self.has_spawned = true;
        self.get_block_render_data(offset_x, offset_y).iter().map(|(x,y,z,asset_path)| {
            let scene_handle = asset_server.load(asset_path.to_string() + "#Scene0");
            //Okay I built the block generation to have the z be vertical, but bevy has y as up so I can just pretend z is up now
            (SceneRoot(scene_handle), Transform::from_xyz(*x as f32, *z as f32, *y as f32))
        }).collect()
    }
     pub fn get_block_render_data(&self, offset_x:i64, offset_y:i64) -> LinkedList<(i64, i64, u8, String)>{
            self.blocks.iter().filter_map(|((block_x, block_y, block_z), block)| {
            let block_logic = get_block_logic(&block);
            let mut asset_path;
            if let Some(_asset_path) = block_logic.asset_path(){
                asset_path = _asset_path;
            }else {
                return None;
            }

            //debug print to check generation is working
            // println!("{} at {} {} {}", asset_path, 
            //     block_x.to_owned() as i64+offset_x*CHUNK_SIZE as i64, 
            //     block_z, 
            //     block_y.to_owned() as i64+offset_y*CHUNK_SIZE as i64
            // );

            Some((
                block_x.to_owned() as i64+offset_x*CHUNK_SIZE as i64,  
                block_y.to_owned() as i64+offset_y*CHUNK_SIZE as i64, 
                block_z.to_owned(),
                asset_path
            ))
        }).collect()
    }
}