use essay_ecs::prelude::*;

pub struct UiLayout {
    horiz: Vec<f32>,
}

impl UiLayout {
    fn new() -> Self {
        Self {
            horiz: Vec::new(),
        }
    }
}

fn layout_update(mut layout: ResMut<UiLayout>) {
    
}

pub struct UiLayoutPlugin;

impl Plugin for UiLayout {
    fn build(&self, app: &mut essay_ecs::prelude::App) {
        app.insert_resource(UiLayout::new());

        app.system(First, layout_update);
    }
}