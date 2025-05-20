use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{
    hind_brain::{
        lateral_line::{LateralLine, LateralLine2Plugin, Segment},
        HindMove, HindMovePlugin
    }, 
    subpallium::{StriatumExclusive, StriatumId, StriatumTimeout}, 
    util::{DecayValue, HalfLife, Seconds, Side, Ticks, TimeoutValue, Turn}
};

fn update_orient_tectum(
    mut hind_move: ResMut<HindMove>,
    mut orient: ResMut<OrientTectum>,
    tick: Res<AppTick>,
) {
    //if let Some(turn) = orient.update(tick.as_ref()) {
    //    hind_move.turn(turn);
    //}

    orient.update(hind_move.get_mut(), tick.as_ref());

    /*
    let turn_left = orient.left.turn(lateral_line.as_ref());
    let turn_right = orient.right.turn(lateral_line.as_ref());

    if turn_left.is_some() && turn_right.is_some() {
        // todo
    } else if let Some(turn) = turn_left {
        hind_move.turn(turn);
    } else if let Some(turn) = turn_right {
        hind_move.turn(turn);
    }
    */

    /*
    if thigmotaxis.is_active() {
        //thigmotaxis_ui.set_value(thigmotaxis.active_value());
    }

    thigmotaxis_ui.set_left(if turn_left.is_some() { 1. } else { 0. });
    thigmotaxis_ui.set_right(if turn_right.is_some() { 1. } else { 0. });
    */
}

pub struct OrientTectum {
    turn_max: f32,

    // memory of thigmotaxis side.
    left: OrientSide,
    right: OrientSide,

    memory: TimeoutValue<Side>,
    exclusive: StriatumExclusive,
}

impl OrientTectum {
    pub(super) fn new(plugin: &TectumOrientPlugin) -> Self {
        let mut exclusive = StriatumExclusive::default();
        
        OrientTectum {
            turn_max: plugin.turn.to_unit(),
            left: OrientSide::new(Side::Left, &mut exclusive, plugin), // half_life, plugin.turn),
            right: OrientSide::new(Side::Right, &mut exclusive, plugin),
            memory: TimeoutValue::new(Ticks(plugin.memory_time.ticks() as usize)),
            exclusive,
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
        tick: &AppTick,
    )  {
        self.exclusive.update(tick);

        let left = self.left.value();
        let right = self.right.value();

        let left_turn = self.left.turn();
        let right_turn = self.right.turn();

        self.left.update();
        self.right.update();

        //let common = left.min(right);
        // let left_turn = left - common;
        // let right_turn = right - common;

        if right < left {
            self.memory.set(Side::Left);
            let turn = Turn::Unit(- left_turn * self.turn_max);
            hind_move.turn(turn);
        } else if right > 0. {
            self.memory.set(Side::Right);
            let turn = Turn::Unit(right_turn * self.turn_max);
            hind_move.turn(turn);
        } else if let Some(Side::Left) = self.memory.value() {
            if self.left.obstacle.value() <= 0. {
                let turn = Turn::Unit(- self.turn_max);
                hind_move.turn(turn);
            }
        } else if let Some(Side::Right) = self.memory.value() {
            if self.left.obstacle.value() <= 0. {
                let turn = Turn::Unit(self.turn_max);
                hind_move.turn(turn);
            }
        }

        self.memory.update();
    }
}

impl Default for OrientTectum {
    fn default() -> Self {
        Self {
            left: OrientSide::default(Side::Left),
            right: OrientSide::default(Side::Right),
            turn_max: 0.,
            memory: TimeoutValue::new(Ticks(2)),
            exclusive: StriatumExclusive::default(),
        }
    }
}

struct OrientSide {
    ll_target: f32,
    turn_max: f32,

    value: DecayValue,

    obstacle: DecayValue,

    id: StriatumId,
    timeout: StriatumTimeout,
}

impl OrientSide {
    fn new(
        side: Side,
        exclusive: &mut StriatumExclusive,
        plugin: &TectumOrientPlugin,
    ) -> Self {
        let half_life = plugin.memory_time;
        let mut timeout = StriatumTimeout::new();

        if let Some(ticks) = plugin.timeout {
            timeout = timeout.ltd(ticks);
        }

        if let Some(ticks) = plugin.timeout_recover {
            timeout = timeout.decay(ticks);
        }

        let id = exclusive.alloc_id();

        Self {
            ll_target: 0.6,
            turn_max: plugin.turn.to_unit(),
            obstacle: DecayValue::default(),
            value: DecayValue::default(),

            id,
            timeout,
        }
    }

    fn default(
        side: Side,
    ) -> Self {
        let half_life = HalfLife::default();

        Self {
            ll_target: 0.5,
            turn_max: Turn::Unit(0.25).to_unit(),
            obstacle: DecayValue::default(),
            value: DecayValue::default(),
            timeout: StriatumTimeout::new(),
            id: StriatumId::default(),
        }
    }

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
            app.insert_resource(OrientTectum::new(&self));
            app.system(Tick, update_orient_tectum);
        }
    }
}
