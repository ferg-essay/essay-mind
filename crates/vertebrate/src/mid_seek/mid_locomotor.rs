use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_ecs::{prelude::*, core::Local};
use mind_ecs::Tick;
use crate::action::Turn;
//use crate::locomotor::mid_explore::MidExplore;
use crate::tectum::{TectumPlugin, TectumLocomotionStn};
use crate::body::{Body, BodyPlugin};
use crate::util::Angle;

pub struct MesState {
    //left60: ActionFactory,
    //right60: ActionFactory,
    //_forward: ActionFactory,

    // explore: MidExplore,
}

impl MesState {
    const _CPG_TIME : f32 = 1.;

    fn left_seek(
        &mut self,
        body: &mut Body
    ) {
        todo!();
        // body.locomotion_mut().action(&self.left60);
    }

    fn left_avoid(
        &mut self,
        body: &mut Body
    ) {
        todo!();
        // body.locomotion_mut().avoid(&self.left60);
    }

    fn right_seek(
        &mut self,
        body: &mut Body
    ) {
        todo!();
        //body.locomotion_mut().action(&self.right60);
    }

    fn right_avoid(
        &mut self,
        body: &mut Body
    ) {
        todo!();
        // body.locomotion_mut().avoid(&self.right60);
    }

    fn _forward(
        &mut self,
        body: &mut Body
    ) {
        todo!();
        // body.locomotion_mut().action(&self._forward);
    }

    // fn explore_mut(&mut self) -> &mut MidExplore {
    //    &mut self.explore
    // }
}

impl FromStore for MesState {
    fn init(_store: &mut Store) -> Self {
        MesState {
            //left60: ActionFactory::new(1., Angle::Deg(-60.)),
            //right60: ActionFactory::new(1., Angle::Deg(60.)),
            //_forward: ActionFactory::new(1., Angle::Deg(0.)),

            // explore: MidExplore::init(store),
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
    // mut explore: ResMut<MidExplore>,
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
        todo!();
        // explore.update(body.get_mut());
    }
}

#[derive(Clone, Copy, Debug, Event)]
pub struct LocomotorEvent {

}

pub struct MidLocomotorPlugin;

impl Plugin for MidLocomotorPlugin {
    fn build(&self, app: &mut App) {
        todo!();
        assert!(app.contains_plugin::<BodyPlugin>(), "MesLocomotorPlugin requires BodyPlugin");
        assert!(app.contains_plugin::<TectumPlugin>(), "MesLocomotorPlugin requires TectumPlugin");

        //app.init_resource::<MidExplore>();

        // app.system(Tick, update_locomotor);
    }
}
