use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_ecs::{prelude::*, core::Local};
use essay_plot::prelude::Angle;
use essay_tensor::Tensor;
use mind_ecs::Tick;
use crate::world::{World, OdorType};
use crate::body::{Body, BodyPlugin};

pub struct ApproachMlr {
    angle: Angle,
    effort: f32,
}

impl ApproachMlr {
    fn clear(&mut self) {
        self.angle = Angle::Unit(0.);
        self.effort = 0.;
    }

    pub fn turn(&mut self, angle: Angle) {
        self.angle = angle;
        self.effort = 1.;
    }
}

impl FromStore for ApproachMlr {
    fn init(_store: &mut Store) -> Self {
        ApproachMlr {
            angle: Angle::Unit(0.),
            effort: 0.,
        }
    }
}

pub struct RepelMlr {
    angle: Angle,
    effort: f32,
}

impl RepelMlr {
    fn clear(&mut self) {
        self.angle = Angle::Unit(0.);
        self.effort = 0.;
    }

    pub fn turn(&mut self, angle: Angle) {
        self.angle = angle;
        self.effort = 1.;
    }
}

impl FromStore for RepelMlr {
    fn init(_store: &mut Store) -> Self {
        RepelMlr {
            angle: Angle::Unit(0.),
            effort: 0.,
        }
    }
}

pub struct MesState {
    cpg_timer: f32, // 
    turn: Angle,
    _effort: f32,
}

impl MesState {
    const CPG_TIME : f32 = 1.;

    fn is_active(&self) -> bool {
        self.cpg_timer > 0.
    }

    fn approach(
        &mut self,
        angle: Angle,
        _effort: f32
    ) -> bool {
        if self.cpg_timer > 0. {
            return false;
        }

        self.turn = angle;

        self.cpg_timer = Self::CPG_TIME;

        true
    }

    fn repel(
        &mut self,
        angle: Angle,
        _effort: f32
    ) -> bool {
        if self.cpg_timer > 0. {
            return false;
        }

        self.turn = Angle::Unit(1.0 - angle.to_unit());

        self.cpg_timer = Self::CPG_TIME;

        true
    }

    fn update(
        &mut self,
        mut body: ResMut<Body>
    ) {
        if self.cpg_timer > 0. {
            if 0.01 < self.turn.to_unit() && self.turn.to_unit() <= 0.5 {
                body.set_muscle_left(1.);
            } else if 0.5 <= self.turn.to_unit() && self.turn.to_unit() < 0.99 {
                body.set_muscle_right(1.);
            }
        }

        self.cpg_timer = (self.cpg_timer - 1.).max(0.);
    }
}

impl FromStore for MesState {
    fn init(_store: &mut Store) -> Self {
        MesState {
            cpg_timer: 0.,
            turn: Angle::Unit(0.),
            _effort: 1.,
        }
    }
}



fn update_locomotor(
    mut body: ResMut<Body>, 
    world: Res<World>, 
    mut approach: ResMut<ApproachMlr>, 
    mut repel: ResMut<RepelMlr>, 
    mut state: Local<MesState>, 
) {
    let left_touch = body.is_touch_left();
    let right_touch = body.is_touch_right();

    if left_touch && right_touch {
        if random() < 0.5 {
            repel.turn(Angle::Unit(0.25));
        } else {
            repel.turn(Angle::Unit(0.75));
        }
    } else if left_touch {
        repel.turn(Angle::Unit(0.25));
    } else if right_touch {
        repel.turn(Angle::Unit(0.75));
    }
    
    if ! state.is_active() {
        if repel.effort > 0. {
            state.repel(repel.angle, repel.effort);
        } else if approach.effort > 0. {
            state.approach(approach.angle, approach.effort);
        }

        approach.clear();
        repel.clear();
    }

    state.update(body);
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

pub struct MidLocomotorPlugin;

impl Plugin for MidLocomotorPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "MesLocomotorPlugin requires BodyPlugin");

        app.init_resource::<ApproachMlr>();
        app.init_resource::<RepelMlr>();

        // app.system(Tick, rs_update);
        app.system(Tick, update_locomotor);

        // app.system(Tick, food_arrest_update);
    }
}
