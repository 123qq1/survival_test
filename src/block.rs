use bevy::prelude::*;

#[derive(Component)]
pub struct BlockId {
    pub block_id: usize,
    pub index_id: i32,
    pub height: f32,
}

impl BlockId{
    pub fn from_values(block_id:usize,index_id:i32,height:f32) -> BlockId{
        BlockId{block_id,index_id,height}
    }
}