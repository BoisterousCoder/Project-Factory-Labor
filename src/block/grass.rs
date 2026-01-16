
use crate::block::BlockLogic;
pub struct Grass{}
impl BlockLogic for Grass{
    fn asset_path(&self) -> Option<String> {
        return Some("block/grass.gltf".to_string());
    }
}