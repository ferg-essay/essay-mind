#[macro_export]
macro_rules! type_short {
    ($ty:ty) => {
        std::any::type_name::<$ty>()
    }
}