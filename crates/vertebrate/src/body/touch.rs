use essay_ecs::prelude::Event;

#[derive(Clone, Copy, Debug, Event)]
pub enum Touch {
    CollideLeft,
    CollideRight,
}