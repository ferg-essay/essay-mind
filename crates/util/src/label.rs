use std::{any::Any, hash::{Hasher, Hash}};


pub trait DynKey : 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_dyn_eq(&self) -> &dyn DynKey;
    fn dyn_eq(&self, other: &dyn DynKey) -> bool;
    fn dyn_hash(&self, state: &mut dyn Hasher);
}

impl<T> DynKey for T
where
    T: Any + Eq + Hash
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_dyn_eq(&self) -> &dyn DynKey {
        self
    }

    fn dyn_eq(&self, other: &dyn DynKey) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            return self == other;
        }
        false
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        T::hash(self, &mut state);
        self.type_id().hash(&mut state);
    }
}

