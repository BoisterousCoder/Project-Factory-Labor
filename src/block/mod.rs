pub mod block_data;
pub mod dirt;
pub mod grass;
pub mod stone;
use self::block_data::BlockData;

pub trait BlockLogic{
    fn asset_path(&self) -> Option<String>;
    fn update(&mut self, block_data: &mut BlockData){
        println!("Default update for block logic does nothing!");
    }
}
pub fn get_block_logic(block: &BlockData) -> Box<dyn BlockLogic>{
    match block.block_type {
        BlockType::Grass => Box::new(grass::Grass{}) as Box<dyn BlockLogic>,
        BlockType::Dirt => Box::new(dirt::Dirt{}) as Box<dyn BlockLogic>,
        BlockType::Stone => Box::new(stone::Stone{}) as Box<dyn BlockLogic>,
    }
}
pub enum BlockType{
    Grass,
    Dirt,
    Stone
}