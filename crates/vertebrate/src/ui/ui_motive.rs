use essay_ecs::prelude::*;
use essay_plot::{
    prelude::*, 
    artist::{paths::Unit, PathStyle, ColorMaps, ColorMap}
};

use ui_graphics::{ui_layout::{BoxId, UiLayout, UiLayoutEvent}, UiCanvas, ui_canvas::UiRender};
use crate::{body::Body, taxis::taxis_pons::TaxisPons};
use crate::ui::ui_world::UiWorldPlugin;
use crate::util::Angle;

//#[derive(Component)]
pub struct UiMotive {
    id: BoxId,
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<Unit>,

    items: Vec<UiMotiveItem>,

    emoji: Option<FontTypeId>,
}

impl UiMotive {
    pub const N_DIR : usize = 12;

    pub fn new(id: BoxId) -> Self {
        let affine = Affine2d::eye();

        Self {
            id,
            pos: Bounds::zero(),
            clip: Clip::None,
            bounds: Bounds::from([1., 1.]),

            items: Vec::new(),

            emoji: None,
        }
    }

    fn push(&mut self, item: UiMotiveItem) {
        self.items.push(item);

        let width = self.items.len() as f32;
        let height = 1.0f32;

        self.bounds = Bounds::from([
            width.max(self.bounds.width()),
            height.max(self.bounds.height()),
        ]);
    }

    pub fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        self.pos = Bounds::from([
            pos.xmin() + 0.05 * pos.width(),
            pos.ymin() + 0.05 * pos.height(),
            pos.xmax() - 0.05 * pos.width(),
            pos.ymax() - 0.05 * pos.height()
        ]);

        self.clip = Clip::from(&self.pos);
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }
}

pub fn ui_motive_resize(
    mut ui_motive: ResMut<UiMotive>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let id = ui_motive.id;
        ui_motive.set_pos(ui_layout.get_box(id));
    }
}

pub fn ui_motive_draw(
    mut ui_motive: ResMut<UiMotive>,
    mut ui_canvas: ResMut<UiCanvas>
) {
    if let Some(mut ui) = ui_canvas.renderer(Clip::None) {
        if ui_motive.emoji.is_none() {
            let emoji_path = "/Users/ferg/wsp/essay-mind/assets/font/NotoEmoji-Bold.ttf";
        
            ui_motive.emoji = Some(ui.font(emoji_path));
        }

        //let state = MotiveEmoji::new();
        //let crab = "\u{1f980}";
        // graph.text((0.5, 0.5), "\u{1f980}\u{1f990}").family(family).color("red");

        let color_map: ColorMap = ColorMaps::WhiteBlue.into();

        let mut style = PathStyle::new();

        let mut text_style = TextStyle::new();
        text_style.valign(VertAlign::Center);
        text_style.halign(HorizAlign::Center);
        text_style.size(14.);
        text_style.font(ui_motive.emoji.unwrap());

        for (i, item) in ui_motive.items.iter().enumerate() {
            let pos = Point(0.5 + i as f32, 0.5);
            let pos = ui_motive.to_canvas().transform_point(pos);
            
            style.color(color_map.map(0.5));

            item.emoji.draw(&mut ui, pos, &style, &mut text_style);
        }

        //let pos = Point(ui_motive.pos.xmid(), ui_motive.pos.ymid());

        // ui.draw_text(ui_homunculus.emoji_pos, crab, &style);
        //ui.draw_text((100.5, 100.5), "M", &style);

        //println!("Emoji: {:?} {}", ui_homunculus.emoji, crab);
    }
}

pub trait UiMotiveDraw {
    fn draw(
        &self, 
        ui: &mut UiRender, 
        pos: Point, 
        style: &PathStyle,
        text_style: &mut TextStyle
    );

    fn box_clone(&self) -> Box<dyn UiMotiveDraw>;
}

#[derive(Clone, Debug)]
pub enum MotiveEmoji {
    Bandage,
    Bell,
    Candy,
    Cheese,
    Cupcake,
    Crab,
    Detective,
    DirectHit,
    Eyes,

    FaceAstonished,
    FaceConfounded,
    FaceDisappointed,
    FaceFreezing,
    FaceFrowning,
    FaceGrimacing,
    FaceGrinning,
    FaceOpenMouth,
    FaceOverheated,
    FaceMonocle,
    FaceNauseated,
    FaceNeutral,
    FaceSleeping,
    FaceSleepy,
    FaceSlightlySmiling,
    FaceSunglasses,
    FaceThinking,
    FaceVomiting,
    FaceWithCowboyHat,
    FaceWithThermometer,
    FaceWorried,
    FaceYawning,

    Footprints,
    ForkAndKnife,
    Lemon,
    MagnifyingGlassLeft,
    MagnifyingGlassRight,
    OctagonalSign,
    Onion,
    Pedestrian,
    Salt,
    Sleeping,
    Telescope,
}

impl MotiveEmoji {
    fn new() -> Self {
        Self::Footprints
    }

    fn code(&self) -> &str {
        match self {
            MotiveEmoji::Bandage => "\u{1fa79}",
            MotiveEmoji::Bell => "\u{1f514}",
            MotiveEmoji::Candy => "\u{1f36c}",
            MotiveEmoji::Cheese => "\u{1f9c0}",
            MotiveEmoji::Crab => "\u{1f980}",
            MotiveEmoji::Cupcake => "\u{1f9c1}",
            MotiveEmoji::Detective => "\u{1f575}",
            MotiveEmoji::DirectHit => "\u{1f3af}",
            MotiveEmoji::Eyes => "\u{1f440}",

            MotiveEmoji::FaceAstonished => "\u{1f632}",
            MotiveEmoji::FaceConfounded => "\u{1f616}",
            MotiveEmoji::FaceDisappointed => "\u{1f61e}",
            MotiveEmoji::FaceFreezing => "\u{1f976}",
            MotiveEmoji::FaceFrowning => "\u{2639}",
            MotiveEmoji::FaceGrimacing => "\u{1f62c}",
            MotiveEmoji::FaceGrinning => "\u{1f600}",
            MotiveEmoji::FaceMonocle => "\u{1f9d0}",
            MotiveEmoji::FaceNauseated => "\u{1f922}",
            MotiveEmoji::FaceNeutral => "\u{1f610}",
            MotiveEmoji::FaceOverheated => "\u{1f975}",
            MotiveEmoji::FaceOpenMouth => "\u{1f62e}",
            MotiveEmoji::FaceSleepy => "\u{1f62a}",
            MotiveEmoji::FaceSleeping => "\u{1f634}",
            MotiveEmoji::FaceSlightlySmiling => "\u{1f642}",
            MotiveEmoji::FaceSunglasses => "\u{1f60e}",
            MotiveEmoji::FaceThinking => "\u{1f914}",
            MotiveEmoji::FaceVomiting => "\u{1f92e}",
            MotiveEmoji::FaceWithCowboyHat => "\u{1f920}",
            MotiveEmoji::FaceWithThermometer => "\u{1f912}",
            MotiveEmoji::FaceWorried => "\u{1f61f}",
            MotiveEmoji::FaceYawning => "\u{1f971}",

            MotiveEmoji::Footprints => "\u{1f463}",
            MotiveEmoji::ForkAndKnife => "\u{1f374}",
            MotiveEmoji::Lemon => "\u{1f34b}",
            MotiveEmoji::MagnifyingGlassLeft => "\u{1f50d}",
            MotiveEmoji::MagnifyingGlassRight => "\u{1f50e}",
            MotiveEmoji::OctagonalSign => "\u{1f6d1}",
            MotiveEmoji::Onion => "\u{1f9c5}",
            MotiveEmoji::Pedestrian => "\u{1f6b6}",
            MotiveEmoji::Salt => "\u{1f9c2}",
            MotiveEmoji::Sleeping => "\u{1f4a4}",
            MotiveEmoji::Telescope => "\u{1f52d}",
        }
    }
}

impl UiMotiveDraw for MotiveEmoji {
    fn draw(
        &self, 
        ui: &mut UiRender, 
        pos: Point, 
        style: &PathStyle,
        text_style: &mut TextStyle
    ) {
        ui.draw_text(pos, self.code(), style, text_style);
    }

    fn box_clone(&self) -> Box<dyn UiMotiveDraw> {
        Box::new(self.clone())
    }
}

struct UiMotiveItem {
    emoji: MotiveEmoji,
}

impl UiMotiveItem {
    fn new(emoji: MotiveEmoji) -> Self {
        Self {
            emoji,
        }
    }
}

impl Clone for UiMotiveItem {
    fn clone(&self) -> Self {
        Self { 
            emoji: self.emoji.clone()
        }
    }
}

//
// UiMotivePlugin
//

pub struct UiMotivePlugin {
    bounds: Bounds::<UiLayout>,
    items: Vec<UiMotiveItem>,

}

impl UiMotivePlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            items: Vec::new(),
        }
    }

    pub fn item(mut self, emoji: MotiveEmoji) -> Self {
        self.items.push(UiMotiveItem::new(emoji));

        self
    }
}

impl Plugin for UiMotivePlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());

            let mut ui_motive = UiMotive::new(box_id);

            for item in &self.items {
                ui_motive.push(item.clone());
            }

            app.insert_resource(ui_motive);

            app.system(PreUpdate, ui_motive_resize);
            app.system(Update, ui_motive_draw);
        }
    }
}
