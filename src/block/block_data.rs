use std::collections::HashMap;
use crate::block::BlockType;

pub struct BlockData{
    pub block_type: BlockType,
    // TODO: Replace String with something that makes more sense
    pub data: HashMap<String, String> 
}
impl BlockData {
    pub fn new(block_type: BlockType) -> BlockData{
        BlockData { block_type, data: HashMap::new() }
    }

}