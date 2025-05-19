mod thigmotaxis;
mod ppt;
mod orient;
mod looming_zebrafish_mtl;
mod looming;
mod tectum_map;
mod attention;

pub use looming::TectumLoomingPlugin;

pub use tectum_map::{TectumMap, TectumPlugin};
pub use orient::{TectumOrientPlugin, OrientTectum};