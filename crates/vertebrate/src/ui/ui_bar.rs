use std::cell::RefCell;

use essay_ecs::prelude::*;
use essay_graphics::layout::{View, ViewArc};
use essay_plot::{
    api::renderer::Drawable, 
    artist::{paths::{self, Unit}, PathStyle}, 
    prelude::*
};
use renderer::Canvas;
use ui_graphics::ViewPlugin;

#[derive(Component)]
pub struct UiBar {
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<UiBar>,
    values: Vec<BarItem>,
}

impl UiBar {
    pub fn new() -> Self {
        Self {
            pos: Bounds::zero(),
            clip: Clip::None,
            values: Vec::new(),
            bounds: Bounds::from([1., 1.]),
        }
    }

    fn add(&mut self, label: &str, color: Color) -> BarId {
        let id = BarId(self.values.len());

        self.values.push(BarItem::new(label, color));

        self.bounds = Bounds::from([self.values.len() as f32, 1.]);

        id
    }

    fn set_pos(&mut self, set_pos: &Bounds<Canvas>) {
        self.pos = Bounds::new(
            (set_pos.xmin(), set_pos.ymin()), 
            (set_pos.xmin() + set_pos.width() * 0.95,  
            set_pos.ymin() + set_pos.height() * 0.95),  
        );
        self.clip = Clip::from(&self.pos);
    }

    fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }
}

impl Drawable for UiBar {
    fn draw(&mut self, ui: &mut dyn renderer::Renderer) -> renderer::Result<()> {
        self.set_pos(ui.pos());

        let y_min = 0.1;
        let x_margin = 0.2;
        let width = self.values.len().max(1);

        let y_mid = y_min + 0.5 * (1. - y_min);

        let mid = paths::rect::<Unit>(
            [0., y_mid], 
            [1. * width as f32, y_mid + 0.001]
        ).transform(&self.to_canvas());
    
        let mut style = PathStyle::new();
        style.color(0xe0e0e0);
        ui.draw_path(&mid, &style)?;

        let mut text_style = TextStyle::new();
        text_style.valign(VertAlign::Top);
        let text_path_style = PathStyle::new();

        for (i, item) in self.values.iter().enumerate() {
            let value = item.value;
            let y = (1. - y_min) * value + y_min;
            let x = i as f32;

            let line = paths::rect::<Unit>(
                [x + x_margin, y_min], 
                [x + 1.0 - x_margin, y]
            ).transform(&self.to_canvas());

            style.color(item.color);

            ui.draw_path(&line, &style)?;

            ui.draw_text(
                self.to_canvas().transform_point(Point(x + 0.5, y_min)), 
                &item.label,
                0.,
                &text_path_style,
                &text_style
            )?;
        }

        let base = paths::rect::<Unit>(
            [0., y_min], 
            [1. * width as f32, y_min + 0.001]
        ).transform(&self.to_canvas());
    
        let mut style = PathStyle::new();
        style.color("black");
        ui.draw_path(&base, &style)?;

        Ok(())
    }
}

impl Coord for UiBar {}


struct BarItem {
    label: String,
    color: Color,
    value: f32,
}

impl BarItem {
    fn new(label: &str, color: Color) -> Self {
        Self {
            label: String::from(label),
            color,
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

pub struct UiBarPlugin {
    colors: Vec<Color>,

    items: Vec<(String, Box<dyn Item>)>,

    view: Option<View<UiBar>>,
}

impl UiBarPlugin {
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

impl ViewPlugin for UiBarPlugin {
    fn view(&mut self, _app: &mut App) -> Option<&ViewArc> {
        let ui_bar = UiBar::new();

        self.view = Some(View::from(ui_bar));

        self.view.as_ref().map(|v| v.arc())
    }
}

impl Plugin for UiBarPlugin {
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
                move |res: Res<T>, mut ui_bar: ResMut<View<UiBar>>| {
                    ui_bar.write(|v| {
                        v.values[id.i()].value = fun(res.get());
                    });
            });
        }
    }
} 

type UpdateBox<T> = Box<dyn Fn(&T) -> f32 + Sync + Send>;
