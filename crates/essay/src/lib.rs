use essay_ecs::prelude::*;
use vertebrate::world::{WorldPlugin, OdorType, FloorType};

pub fn world_block(app: &mut App) {
    app.plugin(
        WorldPlugin::new(20, 10)
        .wall((4, 5), (4, 1))
        .wall((4, 0), (1, 5))
        .food_odor(1, 1, OdorType::FoodA)
        .food_odor(8, 2, OdorType::FoodB)
        .odor(14, 8, OdorType::FoodB)
        .odor(0, 9, OdorType::AvoidA)
    );
}


pub fn world_place_preference(app: &mut App) {
    let w = 12;
    let h = 7;

    let w1 = w / 2;
    let w2 = w1;

    app.plugin(
        WorldPlugin::new(w, h)
        .wall(((w - 1) / 2, 0), (2, 2))
        .wall(((w - 1) / 2, 5), (2, 2))
        .floor((0, 0), (w1, h), FloorType::Light)
        .floor((w2, 0), (w - w2, h), FloorType::Dark)
    );
}

pub fn slug_world(app: &mut App) {
    let mut world = WorldPlugin::new(30, 20);

    world = world.walls([
        (8, 4), (8, 5), (8, 6), (8, 7), (8, 8), (8, 9), (8, 10), (8, 11), (8, 12),
        (18, 6), (19, 6), (20, 6), (25, 6), (26, 6),
        (20, 14), (21, 14), (22, 14), (26, 14), (27, 14)
    ]);

    world = world.food_odor(4, 2, OdorType::FoodA)
        .food_odor(20, 10, OdorType::FoodB);

    app.plugin(world);
}