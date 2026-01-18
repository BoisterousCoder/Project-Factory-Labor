use std::collections::HashMap;
use crate::block::{BlockType, get_block_logic};

pub struct BlockData{
    pub block_type: BlockType,
    pub is_full_block: bool,
    // TODO: Replace String with something that makes more sense
    pub data: HashMap<String, String> 
}
impl BlockData {
    pub fn new(block_type: BlockType) -> BlockData{
        let block_logic = get_block_logic(&block_type);
        BlockData { 
            block_type, 
            data: HashMap::new(), 
            is_full_block: block_logic.is_full_block() 
        }
    }

}