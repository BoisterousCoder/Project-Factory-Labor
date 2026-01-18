use crate::block::block_data::BlockData;
use crate::block::get_block_logic;
use crate::world::CHUNK_SIZE;

use std::collections::HashMap;
use std::collections::LinkedList;
use bevy::prelude::*;

const MAX_HEIGHT:u8 = 32;

pub struct Chunk{
    pub blocks:HashMap<(u8,u8,u8), BlockData>,
    pub has_spawned:bool
}
impl Chunk {
    pub fn new() -> Chunk{
        let mut blocks:HashMap<(u8,u8,u8), BlockData> = HashMap::with_capacity(CHUNK_SIZE as usize*CHUNK_SIZE as usize*MAX_HEIGHT as usize);
        Chunk{blocks, has_spawned:false}
    }
    pub fn get_scene_bundle(&mut self, asset_server: &Res<AssetServer>, offset_x:i32, offset_y:i32) -> LinkedList<(SceneRoot, Transform)>{
        self.has_spawned = true;
        self.get_block_render_data(offset_x, offset_y).iter().map(|(x,y,z,asset_path)| {
            let scene_handle = asset_server.load(asset_path.to_string() + "#Scene0");
            //Okay I built the block generation to have the z be vertical, but bevy has y as up so I can just pretend z is up now
            (SceneRoot(scene_handle), Transform::from_xyz(*x as f32, *z as f32, *y as f32))
        }).collect()
    }
    pub fn get_block_render_data(&self, offset_x:i32, offset_y:i32) -> LinkedList<(i32, i32, u8, String)>{
            self.blocks.iter().filter_map(|((block_x, block_y, block_z), block)| {
            let block_logic = get_block_logic(&block.block_type);
            
            //Hide if there is no asset to show
            let mut asset_path;
            if let Some(_asset_path) = block_logic.asset_path(){
                asset_path = _asset_path;
            }else {
                return None;
            }

            //Hide if the block is hidden by having a full block on all sides
            if self.is_block_surrounded(*block_x, *block_y, *block_z) {
                return None;
            }

            Some((
                block_x.to_owned() as i32+offset_x*CHUNK_SIZE as i32,  
                block_y.to_owned() as i32+offset_y*CHUNK_SIZE as i32, 
                block_z.to_owned(),
                asset_path
            ))
        }).collect()
    }
    fn is_block_surrounded(&self, x:u8, y:u8, z:u8) -> bool{
        fn exists_and_is_full(block: Option<&BlockData>) -> bool{
            if let Some(existing_block) = block {
                return existing_block.is_full_block;
            }else{
                return false;
            }
        }

        //Stop out of bounds for u8
        if (x < 1 || x >= u8::MAX) || 
            (y < 1 || y >= u8::MAX) || 
            (z < 1 || z >= u8::MAX) {
            return false;
        }
        
        let block_above = self.blocks.get(&(x, y+1, z));
        let block_below = self.blocks.get(&(x, y-1, z));
        let block_left = self.blocks.get(&(x+1, y, z));
        let block_right = self.blocks.get(&(x-1, y, z));
        let block_forward = self.blocks.get(&(x, y, z+1));
        let block_back = self.blocks.get(&(x, y, z-1));

        if exists_and_is_full(block_above) &&
            exists_and_is_full(block_below) &&
            exists_and_is_full(block_left) &&
            exists_and_is_full(block_right) &&
            exists_and_is_full(block_forward) &&
            exists_and_is_full(block_back)
        {
            return true;
        }
        return false;
    }
}
