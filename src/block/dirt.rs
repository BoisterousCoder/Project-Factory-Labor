use crate::block::BlockLogic;
pub struct Dirt{}
impl BlockLogic for Dirt{
    fn asset_path(&self) -> Option<String> {
        return Some("block/dirt.gltf".to_string());
    }
}