use essay_ecs::{core::ResMut, prelude::{App, Plugin}};
use mind_ecs::PreTick;

use crate::util::{
    Angle, DecayValue, Heading
};

pub struct TectumMap {
    pos_map: Vec<DecayValue>,
    neg_map: Vec<DecayValue>,
}

impl TectumMap {
    const THRESHOLD : f32 = 1.0e-1;
    pub const N : usize = 12;

    fn update(&mut self) {
        for value in &mut self.pos_map {
            value.update();
        }

        for value in &mut self.neg_map {
            value.update();
        }
    }

    pub fn neg(&mut self, dir: Heading, value: f32) {
        let da = 0.2 / Self::N as f32;
        let d1 = Heading::unit(dir.to_unit() + da);
        let d2 = Heading::unit(dir.to_unit() - da);

        //let i = (Self::N as f32 * dir.to_unit()).floor() as usize;
        //self.neg_map[i].set_max(value);

        let i = (Self::N as f32 * d1.to_unit()).floor() as usize;
        self.neg_map[i].set_max(value);

        let i = (Self::N as f32 * d2.to_unit()).floor() as usize;
        self.neg_map[i].set_max(value);
    }

    pub fn pos(&mut self, dir: Heading, value: f32) {
        let i = (Self::N as f32 * dir.to_unit()).floor() as usize;

        self.pos_map[i].set_max(value);
    }

    pub fn values(&self) -> Vec<f32> {
        let mut vec = Vec::<f32>::new();

        for (pos, neg) in self.pos_map.iter().zip(&self.neg_map) {
            let pos_value = pos.value();
            let neg_value = neg.value();

            vec.push(if neg_value < Self::THRESHOLD {
                0.5 + 0.5 * pos_value
            } else {
                0.5 - 0.5 * neg_value
            });
        }

        vec
    }
}

impl Default for TectumMap {
    fn default() -> Self {
        let mut pos_map = Vec::new();
        let mut neg_map = Vec::new();

        for _ in 0..Self::N {
            pos_map.push(DecayValue::new(0.5));
            neg_map.push(DecayValue::new(0.5));
        }

        Self { 
            pos_map,
            neg_map,
        }
    }
}

fn update_tectum(mut tectum_map: ResMut<TectumMap>) {
    tectum_map.update();
}

pub struct TectumPlugin {
}

impl TectumPlugin {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn striatum(self) -> Self {
        Self {
            .. self
        }
    }

    pub fn ni(self) -> Self {
        Self {
            .. self
        }
    }
}

impl Plugin for TectumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TectumMap>();
        app.system(PreTick, update_tectum);
    }
}
