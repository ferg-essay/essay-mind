mod hind_eat;
mod hind_move;
pub mod lateral_line;
mod random_walk;
mod startle;

pub use hind_eat::{
    HindEatPlugin, HindEat
};

pub use hind_move::{
    HindMove,
    HindMovePlugin,
    MoveKind,
};
