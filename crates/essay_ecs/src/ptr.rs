use std::{marker::PhantomData, ptr::NonNull, fmt::Pointer};

#[derive(Copy, Clone)]
pub struct Ptr<'a>(NonNull<u8>, PhantomData<&'a u8>);

#[derive(Copy, Clone)]
pub struct PtrMut<'a>(NonNull<u8>, PhantomData<&'a mut u8>);

impl<'a> Ptr<'a> {
    #[inline]
    pub fn new(data: NonNull<u8>) -> Self {
        Self(data, PhantomData)
    }

    #[inline]
    pub unsafe fn deref<T>(self) -> &'a T {
        &*self.as_ptr().cast::<T>() // .debug_ensure_aligned()
    }

    #[inline]
    pub fn as_ptr(self) -> *mut u8 {
        self.0.as_ptr()
    }
}

impl<'a, T> From<&'a T> for Ptr<'a> {
    fn from(value: &'a T) -> Self {
        Self::new(NonNull::from(value).cast())
    }
}


impl<'a> From<Ptr<'a>> for NonNull<u8> {
    fn from(value: Ptr<'a>) -> Self {
        value.0
    }
}

impl Pointer for Ptr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.0, f)
    }
}

impl<'a> PtrMut<'a> {
    #[inline]
    pub fn new(data: NonNull<u8>) -> Self {
        Self(data, PhantomData)
    }

    #[inline]
    pub unsafe fn deref_mut<T>(self) -> &'a mut T {
        &mut *self.as_ptr().cast::<T>() // .debug_ensure_aligned()
    }

    #[inline]
    pub fn as_ptr(self) -> *mut u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_ref(&self) -> Ptr<'_> {
        Ptr::new(self.0)
    }
}

impl<'a, T> From<&'a T> for PtrMut<'a> {
    fn from(value: &'a T) -> Self {
        Self::new(NonNull::from(value).cast())
    }
}


impl<'a> From<PtrMut<'a>> for NonNull<u8> {
    fn from(value: PtrMut<'a>) -> Self {
        value.0
    }
}

impl Pointer for PtrMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.0, f)
    }
}

mod tests {
    use crate::ptr::PtrMut;

    use super::Ptr;

    #[test]
    fn test_ptr_to_and_from() {
        let test = Test("test-a".to_string());
        let ptr: Ptr =  Ptr::from(&test);
        let test2 = unsafe { ptr.deref::<Test>() };

        assert_eq!(format!("{:?}", test2), "Test(\"test-a\")");
    }

    #[test]
    fn test_ptr_mut_to_and_from() {
        let test = Test("test-a".to_string());
        let ptr: PtrMut =  PtrMut::from(&test);
        let test2 = unsafe { ptr.deref_mut::<Test>() };

        assert_eq!(format!("{:?}", test2), "Test(\"test-a\")");
    }

    #[test]
    fn test_ptr_mut_update() {
        let test = TestValue { value: "test-a".to_string() };
        let ptr: PtrMut =  PtrMut::from(&test);

        update_value(ptr);

        let test2 = unsafe { ptr.as_ref().deref::<TestValue>() };

        assert_eq!(format!("{:?}", test2), "TestValue { value: \"new-a\" }");
    }

    fn update_value(ptr: PtrMut) {
        let test2 = unsafe { ptr.deref_mut::<TestValue>() };
        test2.value = "new-a".to_string();
    }

    #[derive(Debug)]
    struct Test(String);

    #[derive(Debug)]
    struct TestValue {
        value: String
    }
}