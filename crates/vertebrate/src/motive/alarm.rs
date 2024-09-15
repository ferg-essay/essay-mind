use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::BodyEat, hind_brain::HindEatPlugin, util::{Seconds, TimeoutValue}};

///
/// MotiveAlarm includes R.pb CGRP and its S.a/P.bst modulation
/// 

pub struct MotiveAlarm {
    is_alarm: TimeoutValue<bool>
}

impl MotiveAlarm {
    #[inline]
    pub fn is_alarm(&self) -> bool {
        self.is_alarm.value_or(false)
    }

    fn pre_update(&mut self) {
        self.is_alarm.update();
    }
}

impl Default for MotiveAlarm {
    fn default() -> Self {
        Self { 
            is_alarm: TimeoutValue::<bool>::new(Seconds(30.)),
        }
    }
}

fn update_alarm(
    mut alarm: ResMut<MotiveAlarm>,
    body_eat: Res<BodyEat>,
) {
    alarm.pre_update();

    if body_eat.bitter() > 0. {
        alarm.is_alarm.set(true);
    }

    if body_eat.sickness() > 0. {
        alarm.is_alarm.set(true);
    }
}

pub struct MotiveAlarmPlugin;

impl Plugin for MotiveAlarmPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindEatPlugin>(), "MotiveAlarm requires HindEat");

        app.insert_resource(MotiveAlarm::default());

        app.system(Tick, update_alarm);
    }
}
