use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_ecs::{prelude::*, core::Local};
use essay_plot::prelude::Angle;
use essay_tensor::Tensor;
use mind_ecs::Tick;
use crate::body_locomotion::{Action, ActionFactory};
use crate::tectum::TectumPlugin;
use crate::{
    tectum::{Turn, TectumLocomotion},
    body::{Body, BodyPlugin}
};

pub struct MesState {
    _effort: f32,
    left: ActionFactory,
    left60: ActionFactory,
    right: ActionFactory,
    right60: ActionFactory,
    forward: ActionFactory,
}

impl MesState {
    const _CPG_TIME : f32 = 1.;

    fn left(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().action(&self.left);
    }

    fn left60(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().action(&self.left60);
    }

    fn right(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().action(&self.right);
    }

    fn right60(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().action(&self.right60);
    }

    fn forward(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().action(&self.forward);
    }
}

impl FromStore for MesState {
    fn init(_store: &mut Store) -> Self {
        MesState {
            _effort: 1.,
            left: ActionFactory::new(1., Angle::Deg(30.)),
            left60: ActionFactory::new(1., Angle::Deg(60.)),
            right: ActionFactory::new(1., Angle::Deg(-30.)),
            right60: ActionFactory::new(1., Angle::Deg(-60.)),
            forward: ActionFactory::new(1., Angle::Deg(0.)),
        }
    }
}

fn update_touch(
    body: &Body,
    tectum: &mut TectumLocomotion,
) {
    if body.is_collide_left() {
        tectum.away().turn(Turn::Right, 1.);
    }

    if body.is_collide_right() {
        tectum.away().turn(Turn::Left, 1.);
    }
}

fn update_locomotor(
    mut body: ResMut<Body>, 
    mut tectum: ResMut<TectumLocomotion>,
    mut state: Local<MesState>, 
) {
    let tectum = tectum.get_mut();
    update_touch(body.get(), tectum);

    tectum.update();

    //if ! body.locomotion().is_idle() {
    //    return;
    //}

    if let Some(turn) = tectum.away().action() {
        match turn {
            Turn::Left => { state.left60(body.get_mut()); }
            Turn::Right => { state.right60(body.get_mut()); }
        }

        tectum.away().action_copy(turn)
    } else if let Some(turn) = tectum.away_odor().action() {
        match turn {
            Turn::Left => { state.left60(body.get_mut()); }
            Turn::Right => { state.right60(body.get_mut()); }
        }
    
        tectum.away().action_copy(turn)
    } else if let Some(turn) = tectum.toward().action() {
        match turn {
            Turn::Left => { state.left60(body.get_mut()); }
            Turn::Right => { state.right60(body.get_mut()); }
        }

        tectum.toward().action_copy(turn)
    } else {
        let random = random();
        let step = 0.33;
        let angle = 60. + (random_normal() * 15.).clamp(-30., 30.);

        // bounded pareto as approximation of Levy walk
        let low = 1.;
        let high = 4.;
        let alpha = 2.;
        let len = low;

        // semi-brownian
        if 0. <= random && random <= step {
            let action = Action::new(len, 1., Angle::Deg(angle));
            body.locomotion_mut().action(action);
            tectum.toward().action_copy(Turn::Left)
        } else if step <= random && random <= 2. * step {
            let action = Action::new(len, 1., Angle::Deg(-angle));
            body.locomotion_mut().action(action);
            tectum.toward().action_copy(Turn::Right)
        } else {
            let len = random_pareto(low, high, alpha);

            let action = Action::new(len, 1., Angle::Unit(0.));

                // state.forward(body.get_mut());

            body.locomotion_mut().action(action);
        }
    }
}

fn random_pareto(low: f32, high: f32, alpha: f32) -> f32 {
    let x = random();

    let h_a = high.powf(alpha);
    let l_a = low.powf(alpha);

    (- (x * h_a - x * l_a - h_a) / (h_a * l_a)).powf(- 1. / alpha)
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

fn random_normal() -> f32 {
    Tensor::random_normal([1], ())[0]
}

pub struct MidLocomotorPlugin;

impl Plugin for MidLocomotorPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "MesLocomotorPlugin requires BodyPlugin");
        assert!(app.contains_plugin::<TectumPlugin>(), "MesLocomotorPlugin requires TectumPlugin");

        app.system(Tick, update_locomotor);
    }
}
