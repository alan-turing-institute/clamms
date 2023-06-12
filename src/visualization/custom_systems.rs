use crate::model::board::*;
use crate::model::env_item::EnvItem;
use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::prelude::Component;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;

impl Component for Patch {
    type Storage = TableStorage;
}

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
            EnvItem::Tree => "tree".to_string(),
            EnvItem::Land => "land".to_string(),
            EnvItem::Sweet => "sweet".to_string(),
        };
    }
    fn fetch_loc(state: &Board, obj: &Patch) -> Option<Int2D> {
        if let Some(loc) = state.field.get_location(*obj) {
            // shift environment object grid up by 1, to align it with
            // the agent grid
            Some(Int2D {
                x: loc.x,
                y: loc.y + 1,
            })
        } else {
            None
        }
    }
    fn fetch_rotation(_state: &Board, _obj: &Patch) -> f32 {
        0.0
    }
    fn scale(_obj: &Patch) -> (f32, f32) {
        (0.016, 0.016)
    }
}
