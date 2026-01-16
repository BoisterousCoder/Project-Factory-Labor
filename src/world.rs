pub const GEN_DEPTH:u8 = 8;
const START_RADIUS_CHUNKS:u8 = 3;
pub const CHUNK_SIZE:u8 = 16;


use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::chunk::Chunk;
use crate::block::block_data::BlockData;
use crate::block::BlockType;

pub static WORLD: LazyLock<DashMap<(i64,i64), Chunk>> = LazyLock::new(|| {
    gen_world()
}); 

fn gen_world() -> DashMap<(i64,i64), Chunk>{
    let chunks:DashMap<(i64,i64), Chunk> = DashMap::new();
    for x in 0..(START_RADIUS_CHUNKS*2) as i64{
        for y in 0..(START_RADIUS_CHUNKS*2) as i64{
            let mut chunk = gen_chunk(x,y);
            chunks.insert((x-START_RADIUS_CHUNKS as i64,y-START_RADIUS_CHUNKS as i64), chunk);
        }
    }
    chunks
}

fn gen_chunk(x:i64, y:i64) -> Chunk{
    let mut chunk = Chunk::new();
    for x in 0..CHUNK_SIZE{
        for y in 0..CHUNK_SIZE{
            for z in 0..GEN_DEPTH{
                if z == GEN_DEPTH-1{
                    chunk.blocks.insert((x,y,z), BlockData::new(BlockType::Grass));
                }else if z > GEN_DEPTH-2{
                    chunk.blocks.insert((x,y,z), BlockData::new(BlockType::Dirt));
                }else{
                    chunk.blocks.insert((x,y,z), BlockData::new(BlockType::Stone));
                }
            }
        }
    }
    chunk
}