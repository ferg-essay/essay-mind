use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_ecs::{prelude::*, core::Local};
use mind_ecs::Tick;
use crate::tectum_action::TectumPlugin;
use crate::{
    tectum_action::{Turn, TectumLocomotion},
    body::{Body, BodyPlugin}
};

pub struct MesState {
    _effort: f32,
}

impl MesState {
    const _CPG_TIME : f32 = 1.;

    fn left(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().set_muscle_left(1.);
    }

    fn right(
        &mut self,
        body: &mut Body
    ) {
        body.locomotion_mut().set_muscle_right(1.);
    }
}

impl FromStore for MesState {
    fn init(_store: &mut Store) -> Self {
        MesState {
            _effort: 1.,
        }
    }
}

fn update_touch(
    body: &Body,
    tectum: &mut TectumLocomotion,
) {
    if body.is_touch_left() {
        tectum.away().turn(Turn::Right, 1.);
    }

    if body.is_touch_right() {
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

    if let Some(turn) = tectum.away().action() {
        match turn {
            Turn::Left => { state.left(body.get_mut()); }
            Turn::Right => { state.right(body.get_mut()); }
        }

        tectum.away().action_copy(turn)
    } else if let Some(turn) = tectum.away_odor().action() {
        match turn {
            Turn::Left => { state.left(body.get_mut()); }
            Turn::Right => { state.right(body.get_mut()); }
        }
    
        tectum.away().action_copy(turn)
    } else if let Some(turn) = tectum.toward().action() {
        match turn {
            Turn::Left => { state.left(body.get_mut()); }
            Turn::Right => { state.right(body.get_mut()); }
        }

        tectum.toward().action_copy(turn)
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
