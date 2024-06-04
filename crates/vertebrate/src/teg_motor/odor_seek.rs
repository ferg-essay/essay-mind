// 
// tegmental motor: posterior tuberculum and Vta
//

use std::marker::PhantomData;

use essay_ecs::app::{App, Plugin};

pub struct TegSeek<I: TegInput, O: TegOutput> {
    marker: PhantomData<(I, O)>,
}

pub trait TegInput {

}

pub trait TegOutput {

}


pub struct TegSeekPlugin<I: TegInput, O: TegOutput> {
    marker: PhantomData<(I, O)>,
}

impl<I: TegInput, O: TegOutput> Plugin for TegSeekPlugin<I, O> {
    fn build(&self, app: &mut App) {
        /*
        assert!(app.contains_plugin::<HindMovePlugin>(), "chemotaxis requires HindMovePlugin");
        
        assert!(app.contains_resource::<OlfactoryBulb>(), "chemotaxis requires OlfactoryBulb");

        app.init_resource::<Taxis>();

        Motives::insert::<Seek>(app, Seconds(0.5));
        Motives::init::<Sated>(app);

        let chemotaxis = Chemotaxis::new();

        app.insert_resource(chemotaxis);

        app.system(Tick, update_chemotaxis);
        */
    }
}
