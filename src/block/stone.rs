use crate::block::BlockLogic;
pub struct Stone{}
impl BlockLogic for Stone{
    fn asset_path(&self) -> Option<String> {
        return Some("block/stone.gltf".to_string());
    }
}