use crate::body::Body;

pub struct Mauthner {
    ss_forward: f32,
    ss_left: f32,
    ss_right: f32,
}

impl Mauthner {
    pub(super) fn new() -> Self {
        Self {
            ss_forward: 0.,
            ss_left: 0.,
            ss_right: 0.,
        }
    }

    pub(super) fn update(&mut self, body: &Body) {
        self.ss_forward = 0.;
        self.ss_left = 0.;
        self.ss_right = 0.;

        if body.is_collide_forward() {
            self.ss_forward = self.ss_forward.max(1.);
        }

        if body.is_collide_left() {
            self.ss_left = self.ss_left.max(1.);
        }

        if body.is_collide_right() {
            self.ss_right = self.ss_right.max(1.);
        }
    }

    pub(super) fn ss_forward(&self) -> f32 {
        self.ss_forward
    }

    pub(super) fn ss_left(&self) -> f32 {
        self.ss_left
    }

    pub(super) fn ss_right(&self) -> f32 {
        self.ss_right
    }
}
