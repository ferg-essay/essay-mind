use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{
    hind_brain::{
        lateral_line::LateralLine2Plugin,
        HindMove, HindMovePlugin
    }, 
    subpallium::{MosaicType, Striatum, StriatumValue2}, 
    util::{DecayValue, HalfLife, Seconds, Side, Ticks, TimeoutValue, Turn}
};

fn update_orient_tectum(
    mut hind_move: ResMut<HindMove>,
    mut orient: ResMut<OrientTectum>,
    mut striatum: ResMut<Striatum<OrientTectum>>,
    tick: Res<AppTick>,
) {
    orient.update(hind_move.get_mut(), striatum.get_mut(), tick.as_ref());
}

pub struct OrientTectum {
    turn_max: f32,

    // memory of thigmotaxis side.
    left: OrientSide,
    right: OrientSide,

    memory: TimeoutValue<Side>,
}

impl OrientTectum {
    pub(super) fn new(plugin: &TectumOrientPlugin) -> Self {
        OrientTectum {
            turn_max: plugin.turn.to_unit(),
            left: OrientSide::default(), // half_life, plugin.turn),
            right: OrientSide::default(),
            memory: TimeoutValue::new(Ticks(plugin.memory_time.ticks() as usize)),
        }
    }

    pub fn is_active(&self) -> bool {
        self.memory.is_active()
    }

    pub fn active_left(&self) -> bool {
        self.memory.value().map_or(false, |v| v == Side::Left)
    }

    pub fn active_right(&self) -> bool {
        self.memory.value().map_or(false, |v| v == Side::Right)
    }

    pub fn active_value(&self) -> f32 {
        self.memory.get_timeout() as f32
    }

    pub fn add_orient_left(&mut self, value: f32) {
        self.left.set_orient_max(value);
    }
    
    pub fn add_orient_right(&mut self, value: f32) {
        self.right.set_orient_max(value);
    }

    pub fn set_obstacle_left(&mut self, value: f32) {
        self.left.obstacle.set(value);
    }

    pub fn set_obstacle_right(&mut self, value: f32) {
        self.right.obstacle.set(value);
    }

    fn update(
        &mut self, 
        hind_move: &mut HindMove,
        striatum: &mut Striatum<OrientTectum>,
        tick: &AppTick,
    )  {
        let mut left = self.left.value();
        let mut right = self.right.value();

        let mut left_turn = self.left.turn();
        let mut right_turn = self.right.turn();

        self.left.update();
        self.right.update();

        if &Some(Side::Left) == self.memory.value() {
            if left == 0. {
                left = 1.;
                left_turn = 0.25;
            }

            if striatum.left_mut().active(tick) == StriatumValue2::Timeout {
                left = 0.;
                self.memory.clear();
            }
        }

        if &Some(Side::Right) == self.memory.value() {
            if right == 0. {
                right = 1.;
                right_turn = 0.25;
            }
            
            if striatum.right_mut().active(tick) == StriatumValue2::Timeout {
                right = 0.;
                self.memory.clear();
            }
        }

        if right < left {
            if striatum.left_mut().active(tick) == StriatumValue2::Active {
                self.memory.set(Side::Left);
                let turn = Turn::Unit(- left_turn * self.turn_max);
                hind_move.turn(turn);
            }
        } else if right > 0. {
            if striatum.right_mut().active(tick) == StriatumValue2::Active {
                self.memory.set(Side::Right);
                let turn = Turn::Unit(right_turn * self.turn_max);
                hind_move.turn(turn);
            }
        }

        self.memory.update();
    }
}

impl Default for OrientTectum {
    fn default() -> Self {
        Self {
            left: OrientSide::default(),
            right: OrientSide::default(),
            turn_max: 0.,
            memory: TimeoutValue::new(Ticks(2)),
        }
    }
}

impl MosaicType for OrientTectum {}

struct OrientSide {
    value: DecayValue,

    obstacle: DecayValue,
}

impl OrientSide {
    fn set_orient_max(&mut self, value: f32) {
        self.value.set_max(value);
    }

    fn value(&self) -> f32 {
        self.value.active_value()
    }

    fn turn(&self) -> f32 {
        (self.value.active_value() - self.obstacle.active_value()).max(0.)
    }

    fn update(
        &mut self, 
    ) {
        self.obstacle.update();
        self.value.update();
    }
}

impl Default for OrientSide {
    fn default() -> Self {
        Self { 
            value: Default::default(), 
            obstacle: Default::default() 
        }
    }
}

pub struct TectumOrientPlugin {
    pub(super) is_enable: bool,
    pub(super) memory_time: HalfLife,
    pub(super) inhibited_value: f32,
    pub(super) turn: Turn,
    pub(super) timeout: Option<Ticks>,
    pub(super) timeout_recover: Option<Ticks>,
}

impl TectumOrientPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self, is_enable: bool) -> &mut Self {
        self.is_enable = is_enable;

        self
    }

    pub fn memory_time(&mut self, timeout: impl Into<HalfLife>) -> &mut Self {
        self.memory_time = timeout.into();

        self
    }

    pub fn turn(&mut self, turn: impl Into<Turn>) -> &mut Self {
        self.turn = turn.into();

        self
    }

    // lateral inhibition value when both sides have lateral line values
    pub fn inhibited_value(&mut self, value: f32) -> &mut Self {
        self.inhibited_value = value;

        self
    }

    pub fn timeout(&mut self, ticks: impl Into<Ticks>) -> &mut Self {
        self.timeout = Some(ticks.into());

        self
    }

    pub fn timeout_recover(&mut self, ticks: impl Into<Ticks>) -> &mut Self {
        self.timeout_recover = Some(ticks.into());

        self
    }
}

impl Default for TectumOrientPlugin {
    fn default() -> Self {
        Self { 
            is_enable: true, 
            memory_time: Seconds(1.).into(),
            turn: Turn::Unit(0.1),
            inhibited_value: 1.,
            timeout: None,
            timeout_recover: None,
        }
    }
}

impl Plugin for TectumOrientPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "TectumThigmotaxis requires HindLocomotion");
        assert!(app.contains_plugin::<LateralLine2Plugin>(), "TectumThigmotaxis requires LateralLine");

        if self.is_enable {
            let mut striatum = Striatum::<OrientTectum>::new();
            striatum.timeout(Seconds(30.));
            striatum.recover(Seconds(5.));

            app.insert_resource(OrientTectum::new(&self));
            app.insert_resource(striatum);
            // TODO: striatum update
            app.system(Tick, update_orient_tectum);
        }
    }
}
