use crate::components::{FieldOfView, Position};
use crate::resources::Map;
use bevy::prelude::*;
use bracket_lib::pathfinding::field_of_view_set;

pub fn fov(mut views_query: Query<(&Position, &mut FieldOfView)>, map: Res<Map>) {
    views_query
        .iter_mut()
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set((*pos).into(), fov.radius, &*map);
            fov.is_dirty = false;
            println!("fov: {:?}", fov.visible_tiles);
        });
}
