use crate::{world::World, util::{DecayValue, Point}};

pub struct BodyEat {
    is_sensor_food: bool,

    is_sweet: DecayValue,
    _is_umami: f32,
    _is_bitter: f32,
    _is_sour: f32,

    blood_sugar: DecayValue,

    is_eating: DecayValue,
}

impl BodyEat {
    pub fn new() -> Self {
        Self {
            is_sensor_food: false,

            is_sweet: DecayValue::new(10),
            _is_umami: 0.,
            _is_bitter: 0.,
            _is_sour: 0.,

            blood_sugar: DecayValue::new(200).fill_time(20),

            is_eating: DecayValue::new(2),
        }
    }

    pub fn is_sensor_food(&self) -> bool {
        self.is_sensor_food
    }

    pub fn set_sensor_food(&mut self, is_food: bool) {
        self.is_sensor_food = is_food;
    }

    #[inline]
    pub fn sweet(&self) -> f32 {
        self.is_sweet.value()
    }

    #[inline]
    pub fn glucose(&self) -> f32 {
        self.blood_sugar.value()
    }

    #[inline]
    pub fn is_eating(&self) -> bool {
        self.is_eating.value() > 0.25
    }

    pub fn eat(&mut self) {
        self.is_eating.add(1.);
    }

    ///
    /// Update the animal's eating and digestion
    /// 
    pub fn update(&mut self, world: &World, pos: Point) {
        self.is_sweet.update();

        self.blood_sugar.update();

        self.is_eating.update();

        let is_food = world.is_food(pos);
        self.set_sensor_food(is_food);

        if self.is_eating() && self.is_sensor_food() {
            self.blood_sugar.add(1.);
            self.is_sweet.add(1.);
        }
    }
}
