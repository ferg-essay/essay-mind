use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_plot::prelude::Angle;
use essay_tensor::Tensor;
use crate::body::{Action, Body};

pub struct MidExplore {
    _effort: f32,
}

impl MidExplore {
    const _CPG_TIME : f32 = 1.;

    pub fn update(
        &mut self,
        body: &mut Body
    ) {
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
            // tectum.toward().action_copy(Turn::Left)
        } else if step <= random && random <= 2. * step {
            let action = Action::new(len, 1., Angle::Deg(-angle));
            body.locomotion_mut().action(action);
            // tectum.toward().action_copy(Turn::Right)
        } else {
            let len = random_pareto(low, high, alpha);

            let action = Action::new(len, 1., Angle::Unit(0.));

                // state.forward(body.get_mut());

            body.locomotion_mut().action(action);
        }
    }
}

impl FromStore for MidExplore {
    fn init(_store: &mut Store) -> Self {
        MidExplore {
            _effort: 1.,
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
