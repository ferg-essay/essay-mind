// 
// tegmental motor: posterior tuberculum and Vta
//

use std::{any::type_name, marker::PhantomData};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{core_motive::{Motive, MotiveTrait, Motives}, hab_taxis::chemotaxis::Seek, hind_motor::{HindMove, HindMovePlugin}, util::{DirVector, Seconds}};

pub struct TegSeek<I: TegInput> {
    marker: PhantomData<I>,
}

impl<I: TegInput> TegSeek<I> {
    fn new() -> Self {
        Self {
            marker: PhantomData::default(),
        }
    }
}

pub trait TegInput : Send + 'static {
    fn seek_dir(&self) -> Option<DirVector>;
}

pub trait TegOutput {

}


pub struct TegSeekPlugin<I: TegInput, M: MotiveTrait> {
    marker: PhantomData<(I, M)>,
}

impl<I: TegInput, M: MotiveTrait> TegSeekPlugin<I, M> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData::<(I, M)>::default(),
        }
    }
}

fn update_seek<I: TegInput, M: MotiveTrait>(
    _seek: ResMut<TegSeek<I>>,
    hind_move: ResMut<HindMove>,
    input: Res<I>,
    motive: Res<Motive<M>>,
    mut seek: ResMut<Motive<Seek>>,
) {
    if ! motive.is_active() {
        return;
    }

    if let Some(dir) = input.seek_dir() {
        seek.set_max(1.);
        
        hind_move.forward(0.5);

        let dir = dir.to_unit();

        if dir < 0.5 {
            hind_move.right_brake((4. * dir).min(1.));
        } else {
            hind_move.left_brake((4. * (1. - dir)).min(1.));
        }
    }
}

impl<I: TegInput, M: MotiveTrait> Plugin for TegSeekPlugin<I, M> {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "TegSeek requires HindMovePlugin");
        assert!(app.contains_resource::<I>(), "TegSeek requires resource {}", type_name::<I>());
        
        let seek = TegSeek::<I>::new();
        app.insert_resource(seek);

        app.system(Tick, update_seek::<I, M>);

        Motives::insert::<Seek>(app, Seconds(0.2));
        /*

        app.init_resource::<Taxis>();

        Motives::init::<Sated>(app);
        */
    }
}
