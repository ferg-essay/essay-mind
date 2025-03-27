use std::cell::RefCell;

use essay_ecs::prelude::*;
use essay_graphics::{layout::{View, ViewArc}, ui::UiState};
use essay_plot::{
    api::renderer::Drawable, 
    prelude::*
};
use ui_graphics::ViewPlugin;

pub struct UiTable {
    values: Vec<TableItem>,
    state: UiState,
}

impl UiTable {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            state: UiState::new(),
        }
    }

    fn add(&mut self, label: &str, color: Color) -> BarId {
        let id = BarId(self.values.len());

        self.values.push(TableItem::new(label, color));

        id
    }
}

impl Drawable for UiTable {
    fn draw(&mut self, ui: &mut dyn renderer::Renderer) -> renderer::Result<()> {
        // self.set_pos(ui.pos());
        self.state.draw(ui, |ui| {
            for item in &self.values {
                ui.horizontal(|ui| {
                    ui.label(&item.label);
                    ui.label(&format!(": {}", item.value));
                });
            }
        })
    }
}

impl Coord for UiTable {}


struct TableItem {
    label: String,
    _color: Color,
    value: f32,
}

impl TableItem {
    fn new(label: &str, color: Color) -> Self {
        Self {
            label: String::from(label),
            _color: color,
            value: 0.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarId(usize);

impl BarId {
    fn i(&self) -> usize {
        self.0
    }
}

pub struct UiTablePlugin {
    colors: Vec<Color>,

    items: Vec<(String, Box<dyn Item>)>,

    view: Option<View<UiTable>>,
}

impl UiTablePlugin {
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            items: Vec::new(),
            view: None,
        }
    }

    pub fn item<T: Send + Sync + 'static>(
        mut self,
        label: &str,
        fun: impl Fn(&T) -> f32 + Send + Sync + 'static
    ) -> Self {
        self.items.push((String::from(label), Box::new(ItemImpl::new(fun))));

        self
    }

    pub fn colors(mut self, colors: impl Into<Colors>) -> Self {
        self.colors = colors.into().into();

        self
    }
}

impl ViewPlugin for UiTablePlugin {
    fn view(&mut self, _app: &mut App) -> Option<&ViewArc> {
        let ui_bar = UiTable::new();

        self.view = Some(View::from(ui_bar));

        self.view.as_ref().map(|v| v.arc())
    }
}

impl Plugin for UiTablePlugin {
    fn build(&self, app: &mut App) {
        if let Some(view) = &self.view {
            app.insert_resource(view.clone());

            let colors = if self.colors.len() > 0 {
                self.colors.clone()
            } else {
                vec!(
                    Color::from("sky"),
                    Color::from("red"),
                    Color::from("beige"),
                    Color::from("purple"),
                    Color::from("olive"),
                )
            };

            let mut view = view.clone();

            for (i, (label, item)) in self.items.iter().enumerate() {
                let color = colors[i % colors.len()];

                let id = view.write(|v| v.add(label, color));

                item.add(id, app);
            }
        }
    }
}

pub trait Item {
    fn add(&self, id: BarId, app: &mut App);
}

struct ItemImpl<T> {
    update: RefCell<Option<UpdateBox<T>>>,
}

impl<T> ItemImpl<T> {
    fn new(fun: impl Fn(&T) -> f32 + Send + Sync + 'static) -> Self {
        Self {
            update: RefCell::new(Some(Box::new(fun))),
        }
    }
}

impl<T: Send + Sync + 'static> Item for ItemImpl<T> {
    fn add(&self, id: BarId, app: &mut App) {
        assert!(app.contains_resource::<T>());

        if let Some(fun) = self.update.take() {
            app.system(
                Update,
                move |res: Res<T>, mut ui_bar: ResMut<View<UiTable>>| {
                    ui_bar.write(|v| {
                        v.values[id.i()].value = fun(res.get());
                    });
            });
        }
    }
} 

type UpdateBox<T> = Box<dyn Fn(&T) -> f32 + Sync + Send>;
