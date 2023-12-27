use essay_ecs::{prelude::{Plugin, App}, core::ResMut};
use mind_ecs::Tick;

use crate::{
    locomotor::{
        mid_locomotor::MidLocomotorPlugin, 
        phototaxis::GoalVector
    },
    util::DecayValue, 
};

pub struct HabenulaSeek {
    toward: Vec::<HabenulaItem>,
    away: Vec::<HabenulaItem>,
}

impl HabenulaSeek {
    pub fn new() -> Self {
        Self {
            toward: Vec::new(),
            away: Vec::new(),
        }
    }
}

fn update_habenula_seek(
    mut seek: ResMut<HabenulaSeek>,
) {

}

struct HabenulaItem {
    value: f32,

    average: DecayValue,
    goal_vector: GoalVector,
}
impl HabenulaItem {
    fn update(&mut self, value: f32) {
        self.average.update();
        self.average.add(value);
    }
}

pub struct HabenulaSetter {
    avoid: AvoidType,
    index: usize,
}

impl HabenulaSetter {
    pub fn update(&self, value: f32, mut hb: impl AsMut<HabenulaSeek>) {
        let hb = hb.as_mut();

        match self.avoid {
            AvoidType::TOWARD => { hb.toward[self.index].update(value) },
            AvoidType::AWAY => { hb.away[self.index].update(value) },
        }
    }
}

enum AvoidType {
    TOWARD,
    AWAY,
}

pub struct HabenulaSeekPlugin;

impl Plugin for HabenulaSeekPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<MidLocomotorPlugin>(), "HabenulaSeekPlugin requires MidLocomotorPlugin");

        app.system(Tick, update_habenula_seek);
    }
}
