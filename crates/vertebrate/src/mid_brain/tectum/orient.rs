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
    mut sustain: ResMut<Sustain>,
    tick: Res<AppTick>,
) {
    sustain.get_mut().update(orient.get_mut(), striatum.get_mut(), tick.as_ref());

    orient.update(hind_move.get_mut(), striatum.get_mut(), sustain.get_mut(), tick.as_ref());
}

#[derive(Default)]
pub struct OrientTectum {
    turn_max: f32,

    // memory of thigmotaxis side.
    left: OrientSide,
    right: OrientSide,

    active: Option<Side>,
}

impl OrientTectum {
    pub(super) fn new(plugin: &TectumOrientPlugin) -> Self {
        OrientTectum {
            turn_max: plugin.turn.to_unit(),
            left: OrientSide::default(), // half_life, plugin.turn),
            right: OrientSide::default(),
            active: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }

    pub fn active_left(&self) -> bool {
        self.active == Some(Side::Left)
    }

    pub fn active_right(&self) -> bool {
        self.active == Some(Side::Right)
    }

    pub fn active_value(&self) -> f32 {
        if self.active.is_some() { 1. } else { 0. }
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

    fn left_mut(&mut self) -> &mut OrientSide {
        &mut self.left
    }

    fn right_mut(&mut self) -> &mut OrientSide {
        &mut self.right
    }

    fn update(
        &mut self, 
        hind_move: &mut HindMove,
        striatum: &mut Striatum<OrientTectum>,
        sustain: &mut Sustain,
        tick: &AppTick,
    )  {
        let left = self.left.value();
        let right = self.right.value();

        let left_turn = self.left.turn();
        let right_turn = self.right.turn();

        self.left.update();
        self.right.update();

        self.active = None;

        if right < left {
            if striatum.left_mut().active(tick) == StriatumValue2::Active {
                sustain.left.active.set(1.);
                self.active = Some(Side::Left);
                let turn = Turn::Unit(- left_turn * self.turn_max);
                hind_move.turn(turn);
            }
        } else if right > 0. {
            if striatum.right_mut().active(tick) == StriatumValue2::Active {
                sustain.right.active.set(1.);
                self.active = Some(Side::Right);
                let turn = Turn::Unit(right_turn * self.turn_max);
                hind_move.turn(turn);
            }
        }
    }
}

impl MosaicType for OrientTectum {}

#[derive(Default)]
struct OrientSide {
    value: DecayValue,
    obstacle: DecayValue,

    excite: f32,
    inhibit: f32,
}

impl OrientSide {
    fn set_orient_max(&mut self, value: f32) {
        self.value.set_max(value);
    }

    fn value(&self) -> f32 {
        let excite = self.excite.min(0.25);
        let value = self.value.active_value().max(excite);

        (value - self.inhibit).max(0.)
    }

    fn turn(&self) -> f32 {
        (self.value() - self.obstacle.active_value()).max(0.)
    }

    fn update(
        &mut self, 
    ) {
        self.obstacle.update();
        self.value.update();

        self.excite = 0.;
        self.inhibit = 0.;
    }
    
    fn excite(&mut self, value: f32) {
        self.excite = value;
    }
    
    fn inhibit(&mut self, value: f32) {
        self.inhibit = value;
    }
}

#[derive(Default)]
struct Sustain {
    left: SustainSide,
    right: SustainSide,
}

impl Sustain {
    pub(super) fn new(plugin: &TectumOrientPlugin) -> Self {
        Self {
            left: SustainSide::new(plugin),
            right: SustainSide::new(plugin),
        }
    }
    
    fn update(
        &mut self, 
        orient: &mut OrientTectum, 
        striatum: &mut Striatum<OrientTectum>, 
        tick: &AppTick
    ) {
        let left = self.left.active.value();
        let right = self.right.active.value();
        let common = left.min(right);

        self.left.active.set(left - common);
        self.right.active.set(right - common);

        let left = self.left.active.active_value();
        let right = self.right.active.active_value();

        if left > 0. {
            match striatum.left_mut().active(tick) {
                StriatumValue2::None => {}
                StriatumValue2::Active => {
                    // PPT excite ipsilateral attention
                    orient.left_mut().excite(left);
                    // PPT inhibit contralateral attention via S.nr
                    orient.right_mut().inhibit(left);
                }
                StriatumValue2::Timeout => {
                    orient.left_mut().inhibit(1.);
                    self.left.active.set(0.);
                }
            }
        }

        if right > 0. {
            match striatum.right_mut().active(tick) {
                StriatumValue2::None => {}
                StriatumValue2::Active => {
                    // PPT excite ipsilateral attention
                    orient.right_mut().excite(right);
                    // PPT inhibit contralateral attention via S.nr
                    orient.left_mut().inhibit(right);
                }
                StriatumValue2::Timeout => {
                    orient.right_mut().inhibit(1.);
                    self.right.active.set(0.);
                }
            }
        }

        self.left.active.update();
        self.right.active.update();
    }
}

#[derive(Default)]
struct SustainSide {
    active: DecayValue,
}

impl SustainSide {
    fn new(plugin: &TectumOrientPlugin) -> Self {
        Self {
            active: DecayValue::new(plugin.memory_time),
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
            app.insert_resource(Sustain::new(&self));
            // TODO: striatum update
            app.system(Tick, update_orient_tectum);
        }
    }
}
