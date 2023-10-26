use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_ecs::{prelude::*, core::Local};
use essay_plot::prelude::Angle;
use essay_tensor::Tensor;
use mind_ecs::Tick;
use crate::action::Turn;
use crate::body_locomotion::{Action, ActionFactory};
use crate::mid_explore::MidExplore;
use crate::tectum::{TectumPlugin, TectumLocomotionStn};
use crate::{
    body::{Body, BodyPlugin}
};

pub struct MesState {
    _effort: f32,
    left: ActionFactory,
    left60: ActionFactory,
    right: ActionFactory,
    right60: ActionFactory,
    forward: ActionFactory,

    explore: MidExplore,
}

impl MesState {
    const _CPG_TIME : f32 = 1.;

    fn left_seek(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().action(&self.left60);
    }

    fn left_avoid(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().action(&self.left60);
    }

    fn right_seek(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().action(&self.right60);
    }

    fn right_avoid(
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

    fn explore_mut(&mut self) -> &mut MidExplore {
        &mut self.explore
    }
}

impl FromStore for MesState {
    fn init(store: &mut Store) -> Self {
        MesState {
            _effort: 1.,
            left: ActionFactory::new(1., Angle::Deg(30.)),
            left60: ActionFactory::new(1., Angle::Deg(60.)),
            right: ActionFactory::new(1., Angle::Deg(-30.)),
            right60: ActionFactory::new(1., Angle::Deg(-60.)),
            forward: ActionFactory::new(1., Angle::Deg(0.)),

            explore: MidExplore::init(store),
        }
    }
}

fn update_touch(
    body: &Body,
    tectum: &mut TectumLocomotionStn,
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
    mut tectum: ResMut<TectumLocomotionStn>,
    mut state: Local<MesState>, 
) {
    let tectum = tectum.get_mut();
    update_touch(body.get(), tectum);

    tectum.seek().default();

    tectum.update();

    //if ! body.locomotion().is_idle() {
    //    return;
    //}

    if let Some(turn) = tectum.away().action() {
        match turn {
            Turn::Left => { state.left_avoid(body.get_mut()); }
            Turn::Right => { state.right_avoid(body.get_mut()); }
        }

        tectum.away().action_copy(turn)
    } else if let Some(turn) = tectum.away_odor().action() {
        match turn {
            Turn::Left => { state.left_avoid(body.get_mut()); }
            Turn::Right => { state.right_avoid(body.get_mut()); }
        }
    
        tectum.away().action_copy(turn)
    } else if let Some(turn) = tectum.seek().action() {
        match turn {
            Turn::Left => { state.left_seek(body.get_mut()); }
            Turn::Right => { state.right_seek(body.get_mut()); }
        }

        tectum.seek().action_copy(turn)
    } else if tectum.seek().indirect() {
        state.explore_mut().update(body.get_mut());
    }
}

pub struct MidLocomotorPlugin;

impl Plugin for MidLocomotorPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "MesLocomotorPlugin requires BodyPlugin");
        assert!(app.contains_plugin::<TectumPlugin>(), "MesLocomotorPlugin requires TectumPlugin");

        app.system(Tick, update_locomotor);
    }
}
