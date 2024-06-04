// 
// tegmental motor: posterior tuberculum and Vta
//

use std::{any::type_name, marker::PhantomData};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{hind_motor::{HindMove, HindMovePlugin}, util::DirVector};

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


pub struct TegSeekPlugin<I: TegInput> {
    marker: PhantomData<I>,
}

impl<I: TegInput> TegSeekPlugin<I> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData::<I>::default(),
        }
    }
}

fn update_seek<I: TegInput>(
    _seek: ResMut<TegSeek<I>>,
    hind_move: ResMut<HindMove>,
    input: Res<I>,
) {
    if let Some(dir) = input.seek_dir() {

        hind_move.forward(0.5);

        let dir = dir.to_unit();

        if dir < 0.5 {
            hind_move.right_brake((4. * dir).min(1.));
        } else {
            hind_move.left_brake((4. * (1. - dir)).min(1.));
        }
    }
}

impl<I: TegInput> Plugin for TegSeekPlugin<I> {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "TegSeek requires HindMovePlugin");
        assert!(app.contains_resource::<I>(), "TegSeek requires resource {}", type_name::<I>());
        
        let seek = TegSeek::<I>::new();
        app.insert_resource(seek);

        app.system(Tick, update_seek::<I>);

        /*

        app.init_resource::<Taxis>();

        Motives::insert::<Seek>(app, Seconds(0.5));
        Motives::init::<Sated>(app);
        */
    }
}
