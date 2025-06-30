mod attention;
mod lateral_line;
mod orient;
mod looming_zebrafish_mtl;
mod looming;
mod tectum_map;
mod thigmotaxis;

pub use looming::TectumLoomingPlugin;

pub use tectum_map::{TectumMap, TectumPlugin};
pub use orient::{TectumOrientPlugin, OrientTectum};
pub use lateral_line::TectumLateralLinePlugin;