use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::prelude::Component;
use crate::model::board::*;
use crate::model::env_item::EnvItem;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl Component for Patch { type Storage = TableStorage; }

impl RenderObjectGrid2D<Board, Patch> for DenseGrid2D<Patch> {
    fn fetch_sparse_grid(_state: &Board) -> Option<&SparseGrid2D<Patch>> {
        None
    }
    fn fetch_dense_grid(state: &Board) -> Option<&DenseGrid2D<Patch>> {
        Some(&state.field)
    }
    fn fetch_emoji(state: &Board, obj: &Patch) -> String {
        let obj_real = state.field.get(obj).unwrap();
        return match obj_real.env_item {
            EnvItem::food => "evergreen_tree".to_string(),
            EnvItem::land => "house".to_string(),
        }
    }
    fn fetch_loc(state: &Board, obj: &Patch) -> Option<Int2D> {
        state.field.get_location(*obj)
    }
    fn fetch_rotation(_state: &Board, _obj: &Patch) -> f32 {
        0.0
    }
    fn scale(_obj: &Patch) -> (f32, f32) {
        (0.02, 0.02)
    }
}
