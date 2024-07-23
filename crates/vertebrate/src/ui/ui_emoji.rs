use essay_plot::{api::{driver::Renderer, Clip, Point, TextStyle}, artist::PathStyle};


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

pub trait SymbolDraw {
    fn draw(
        &self, 
        ui: &mut dyn Renderer, 
        pos: Point, 
        style: &PathStyle,
        text_style: &mut TextStyle
    );
}

impl SymbolDraw for Emoji {
    fn draw(
        &self, 
        ui: &mut dyn Renderer, 
        pos: Point, 
        style: &PathStyle,
        text_style: &mut TextStyle
    ) {
        ui.draw_text(pos, self.code(), 0., style, text_style, &Clip::None).unwrap();
    }
}
