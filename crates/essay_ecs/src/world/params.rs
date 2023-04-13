use std::{ops::{Deref, DerefMut}};

use crate::prelude::Param;

use super::prelude::World;


#[derive(Debug)]
pub struct Res<'a, T> {
    value: &'a T,
}

impl<'a, T:'static> Res<'a, T> {
    pub fn get(&self) -> &T {
        self.value
    }
}

impl<'a, T:'static> Param for Res<'a, T> {
    type Arg<'b> = Res<'b, T>;

    fn get_arg<'b>(world: &'b World) -> Res<'b, T> {
        Res {
            value: world.get_resource::<T>().unwrap(),
        }
    }
}

impl<'a, T:'static> Deref for Res<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

pub struct ResMut<'a, T> {
    value: &'a mut T,
}

impl<'a, T:'static> ResMut<'a, T> {
    pub fn get(&self) -> &T {
        self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<T:'static> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T:'static> DerefMut for ResMut<'a, T> {
    // type Target = T;

    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T:'static> Param for ResMut<'_, T> {
    type Arg<'a> = ResMut<'a, T>;

    fn get_arg<'a>(world: &'a World) -> ResMut<'a, T> {
        ResMut {
            value: world.get_resource_mut().unwrap()
        }
    }
}

