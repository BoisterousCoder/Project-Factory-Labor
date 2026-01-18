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
    //This is used to deterimine if all sides of a block covered so the block can be hidden for performance improvement
    fn is_full_block(&self) -> bool{ true } 
}
pub fn get_block_logic(block_type: &BlockType) -> Box<dyn BlockLogic>{
    match block_type {
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