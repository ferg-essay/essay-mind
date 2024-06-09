use std::cell::RefCell;

use essay_ecs::prelude::*;
use essay_plot::{
    prelude::*, 
    artist::{paths::Unit, PathStyle, ColorMaps, ColorMap}
};

use ui_graphics::{ui_layout::{BoxId, UiLayout, UiLayoutEvent}, UiCanvas, ui_canvas::UiRender};
use crate::ui::ui_world::UiWorldPlugin;

//#[derive(Component)]
pub struct UiMotive {
    id: BoxId,

    size: f32,
    items: Vec<UiMotiveItem>,

    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<Unit>,

    emoji: Option<FontTypeId>,
}

impl UiMotive {
    pub const N_DIR : usize = 12;

    pub fn new(id: BoxId) -> Self {
        // let affine = Affine2d::eye();

        Self {
            id,

            size: 16.,
            items: Vec::new(),

            pos: Bounds::zero(),
            clip: Clip::None,
            bounds: Bounds::from([1., 1.]),

            emoji: None,
        }
    }

    fn push(&mut self, item: UiMotiveItem) -> usize {
        let id = self.items.len();

        let pos = item.pos;

        self.items.push(item);

        let width = self.bounds.width();
        let height = self.bounds.height();

        self.bounds = Bounds::from([
            width.max(pos.x() + 0.5),
            height.max(pos.y() + 0.5),
        ]);

        id
    }

    fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        self.pos = Bounds::from([
            pos.xmin() + 0.05 * pos.width(),
            pos.ymin() + 0.05 * pos.height(),
            pos.xmax() - 0.05 * pos.width(),
            pos.ymax() - 0.05 * pos.height()
        ]);

        self.clip = Clip::from(&self.pos);
    }

    fn to_canvas(&self) -> Affine2d {
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
        text_style.size(ui_motive.size);
        text_style.font(ui_motive.emoji.unwrap());

        let height = ui_motive.bounds.height();

        for item in ui_motive.items.iter() {
            let pos = Point(item.pos.0, height - item.pos.1);
            let pos = ui_motive.to_canvas().transform_point(pos);

            let value = item.value.clamp(0.05, 1.);

            if let Some(color_map) = &item.colormap {
                style.color(color_map.map(value));
            } else {
                style.color(color_map.map(value));
            }

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

impl UiMotiveDraw for Emoji {
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
    pos: Point,

    emoji: Emoji,
    value: f32,
    colormap: Option<ColorMap>,
}

impl UiMotiveItem {
    fn new(item: &Box<dyn PluginItem>) -> Self {
        Self {
            pos: item.pos(),
            emoji: item.emoji(),
            value: 0.,
            colormap: item.colormap(),
        }
    }
}

//
// UiMotivePlugin
//

pub struct UiMotivePlugin {
    bounds: Bounds::<UiLayout>,
    size: f32,
    items: Vec<Box<dyn PluginItem>>,

    x: usize,
    y: usize,
}

impl UiMotivePlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            size: 12.,
            items: Vec::new(),
            x: 0,
            y: 0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;

        self
    }

    pub fn row(mut self) -> Self {
        self.y += 1;
        self.x = 0;

        self
    }

    pub fn item<T>(
        mut self, 
        emoji: Emoji,
        fun: impl Fn(&T) -> f32 + Send + Sync + 'static
    ) -> Self
    where T: Default + Send + Sync + 'static
    {
        // let i = self.items.len();

        self.items.push(Box::new(Item {
            pos: Point(self.x as f32 + 0.5, self.y as f32 + 0.5),
            emoji,
            colormap: None,
            fun: RefCell::new(Some(Box::new(fun)))
        }));

        self.x += 1;

        self
    }

    pub fn colormap(
        mut self,
        colormap: impl Into<ColorMap>,
    ) -> Self {
        assert!(self.items.len() > 0);

        let tail = self.items.len() - 1;
        self.items[tail].set_colormap(colormap.into());

        self

    }
}

trait PluginItem {
    fn pos(&self) -> Point;
    fn emoji(&self) -> Emoji;
    fn set_colormap(&mut self, colormap: ColorMap);
    fn colormap(&self) -> Option<ColorMap>;
    fn system(&self, id: usize, app: &mut App);
}

struct Item<T: Send + Sync + 'static> {
    pos: Point,
    emoji: Emoji,
    colormap: Option<ColorMap>,
    fun: RefCell<Option<Box<dyn Fn(&T) -> f32 + Send + Sync + 'static>>>,
}

impl<T: Send + Sync + 'static> Item<T> {
    
}

impl<T: Default + Send + Sync + 'static> PluginItem for Item<T> {
    fn pos(&self) -> Point {
        self.pos.clone()
    }

    fn emoji(&self) -> Emoji {
        self.emoji.clone()
    }

    fn set_colormap(&mut self, colormap: ColorMap) {
        self.colormap = Some(colormap);
    }

    fn colormap(&self) -> Option<ColorMap> {
        self.colormap.clone()
    }

    fn system(&self, id: usize, app: &mut App) {
        app.init_resource::<T>();

        let fun = self.fun.take().unwrap();

        app.system(PostUpdate, 
            move |mut motives: ResMut<UiMotive>, item: Res<T>| {
                motives.items[id].value = fun(item.get());
            }
        );
    }
} 

impl Plugin for UiMotivePlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());

            let mut ui_motive = UiMotive::new(box_id);

            ui_motive.size = self.size;

            for item in &self.items {
                let id = ui_motive.push(UiMotiveItem::new(item));

                item.system(id, app);
            }

            app.insert_resource(ui_motive);

            app.system(PreUpdate, ui_motive_resize);
            app.system(Update, ui_motive_draw);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Emoji {
    AnatomicalHeart,
    Bandage,
    Bell,
    Candy,
    Cheese,
    Coffee,
    CookedRice,
    Cupcake,
    Crab,
    Detective,
    DirectHit,
    Droplet,
    Eyeglasses,
    Eyes,

    FaceAstonished,
    FaceConfounded,
    FaceDelicious,
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

    Fire,
    Footprints,
    ForkAndKnife,
    HighVoltage,
    Lemon,
    Lollipop,
    Lungs,
    MagnifyingGlassLeft,
    MagnifyingGlassRight,
    NoEntry,
    OctagonalSign,
    Onion,
    PartyPopper,
    Pedestrian,
    Pig,
    Prohibited,
    Radioactive,
    Ribbon,
    Salt,
    Sleeping,
    StopSign,
    Telescope,
    Warning,
    Whale,

    // buttons
    PlayButton,
    PauseButton,
    StopButton,
}

impl Emoji {
    fn _new() -> Self {
        Self::Footprints
    }

    pub fn code(&self) -> &str {
        match self {
            Emoji::AnatomicalHeart => "\u{1fac0}",
            Emoji::Bandage => "\u{1fa79}",
            Emoji::Bell => "\u{1f514}",
            Emoji::Candy => "\u{1f36c}",
            Emoji::Cheese => "\u{1f9c0}",
            Emoji::Coffee => "\u{2615}",
            Emoji::CookedRice => "\u{1f35a}",
            Emoji::Crab => "\u{1f980}",
            Emoji::Cupcake => "\u{1f9c1}",
            Emoji::Detective => "\u{1f575}",
            Emoji::DirectHit => "\u{1f3af}",
            Emoji::Droplet => "\u{1f4a7}",
            Emoji::Eyeglasses => "\u{1f453}",
            Emoji::Eyes => "\u{1f440}",

            Emoji::FaceAstonished => "\u{1f632}",
            Emoji::FaceConfounded => "\u{1f616}",
            Emoji::FaceDelicious => "\u{1f60b}",
            Emoji::FaceDisappointed => "\u{1f61e}",
            Emoji::FaceFreezing => "\u{1f976}",
            Emoji::FaceFrowning => "\u{2639}",
            Emoji::FaceGrimacing => "\u{1f62c}",
            Emoji::FaceGrinning => "\u{1f600}",
            Emoji::FaceMonocle => "\u{1f9d0}",
            Emoji::FaceNauseated => "\u{1f922}",
            Emoji::FaceNeutral => "\u{1f610}",
            Emoji::FaceOverheated => "\u{1f975}",
            Emoji::FaceOpenMouth => "\u{1f62e}",
            Emoji::FaceSleepy => "\u{1f62a}",
            Emoji::FaceSleeping => "\u{1f634}",
            Emoji::FaceSlightlySmiling => "\u{1f642}",
            Emoji::FaceSunglasses => "\u{1f60e}",
            Emoji::FaceThinking => "\u{1f914}",
            Emoji::FaceVomiting => "\u{1f92e}",
            Emoji::FaceWithCowboyHat => "\u{1f920}",
            Emoji::FaceWithThermometer => "\u{1f912}",
            Emoji::FaceWorried => "\u{1f61f}",
            Emoji::FaceYawning => "\u{1f971}",

            Emoji::Fire => "\u{1f525}",
            Emoji::Footprints => "\u{1f463}",
            Emoji::ForkAndKnife => "\u{1f374}",
            Emoji::HighVoltage => "\u{26a1}",
            Emoji::Lemon => "\u{1f34b}",
            Emoji::Lollipop => "\u{1f36d}",
            Emoji::Lungs => "\u{1fac1}",
            Emoji::MagnifyingGlassLeft => "\u{1f50d}",
            Emoji::MagnifyingGlassRight => "\u{1f50e}",
            Emoji::NoEntry => "\u{26d4}",
            Emoji::OctagonalSign => "\u{1f6d1}",
            Emoji::Onion => "\u{1f9c5}",
            Emoji::PartyPopper => "\u{1f389}",
            Emoji::Pig => "\u{1f416}",
            Emoji::Pedestrian => "\u{1f6b6}",
            Emoji::Prohibited => "\u{1f6ab}",
            Emoji::Radioactive => "\u{2622}",
            Emoji::Ribbon => "\u{1f380}",
            Emoji::Salt => "\u{1f9c2}",
            Emoji::Sleeping => "\u{1f4a4}",
            Emoji::StopSign => "\u{1f6d1}",
            Emoji::Telescope => "\u{1f52d}",
            Emoji::Warning => "\u{26a0}",
            Emoji::Whale => "\u{1f40b}",

            // buttons
            Emoji::PlayButton => "\u{25b6}",
            Emoji::PauseButton => "\u{23f8}",
            Emoji::StopButton => "\u{23f9}",
        }
    }
}
