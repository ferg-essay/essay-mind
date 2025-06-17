use std::marker::PhantomData;

use essay_ecs::{app::App, core::ResMut};
use mind_ecs::PostTick;

use crate::hippocampus::Engram64;

pub struct Mosaic<T: MosaicType> {
    engram: Option<Engram64>,
    next_engram: Option<Engram64>,

    marker: PhantomData<fn(T)>,
}

impl<T: MosaicType> Mosaic<T> {
    pub fn init(app: &mut App) {
        if ! app.contains_resource::<Mosaic<T>>() {
            app.init_resource::<Mosaic<T>>();

            app.system(PostTick, update_mosaic::<T>);
        }
    }
}

impl<T: MosaicType> Default for Mosaic<T> {
    fn default() -> Self {
        Self { 
            engram: Default::default(), 
            next_engram: Default::default(), 
            marker: Default::default() 
        }
    }
}

fn update_mosaic<T: MosaicType>(
    mut mosaic: ResMut<Mosaic<T>>,
) {
    mosaic.engram = mosaic.next_engram.take();
    mosaic.next_engram = None;
}

pub trait MosaicType : 'static {}

