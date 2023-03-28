///
/// Ptr/PtrMut structure from bevy_ptr
///
use std::{marker::PhantomData, ptr::NonNull, fmt::Pointer, mem::{ManuallyDrop, self}};

#[derive(Copy, Clone)]
pub struct Ptr<'a>(NonNull<u8>, PhantomData<&'a u8>);

#[derive(Copy, Clone)]
pub struct PtrMut<'a>(NonNull<u8>, PhantomData<&'a mut u8>);

#[derive(Copy, Clone)]
pub struct PtrOwn<'a>(NonNull<u8>, PhantomData<&'a mut u8>);

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

impl<'a> PtrOwn<'a> {
    #[inline]
    pub fn new(data: NonNull<u8>) -> Self {
        Self(data, PhantomData)
    }

    pub fn spawn<T, F, R>(value: T, fun: F) -> R
        where F: FnOnce(PtrOwn<'a>) -> R
    {
        let mut value = ManuallyDrop::new(value);
        
        fun(Self::new(NonNull::from(&mut *value).cast()))
    }

    pub unsafe fn make_into<T>(value: T, storage: &mut NonNull<u8>) -> Self {
        let len = mem::size_of::<T>();

        let offset = 0;
        
        let mut value = ManuallyDrop::new(value);
        let source: NonNull<u8> = NonNull::from(&mut *value).cast();

        std::ptr::copy_nonoverlapping::<u8>(
            source.as_ptr(), 
            storage.as_ptr(),
            len
        );
        // println!("src {:?} target {:?}", ptr.as_ptr(), storage);
    
        PtrOwn::new(*storage)
    }

    #[inline]
    pub unsafe fn deref<T>(self) -> &'a T {
        &*self.as_ptr().cast::<T>() // .debug_ensure_aligned()
    }

    #[inline]
    pub unsafe fn deref_mut<T>(self) -> &'a mut T {
        &mut *self.as_ptr().cast::<T>() // .debug_ensure_aligned()
    }

    /*

    #[inline]
    pub unsafe fn deref_mut<T>(self) -> &'a mut T {
        &mut *self.as_ptr().cast::<T>() // .debug_ensure_aligned()
    }
    */

    #[inline]
    pub fn as_ptr(self) -> *mut u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_mut(&mut self) -> PtrMut<'_> {
        PtrMut::new(self.0)
    }

    #[inline]
    pub fn as_ref(&self) -> Ptr<'_> {
        Ptr::new(self.0)
    }
}

impl<'a> From<PtrOwn<'a>> for NonNull<u8> {
    fn from(value: PtrOwn<'a>) -> Self {
        value.0
    }
}

impl Pointer for PtrOwn<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.0, f)
    }
}

pub struct PtrCell<'t, T> {
    data: Vec<u8>,
    ptr: PtrOwn<'t>,
    marker: PhantomData<T>,
}

// TODO: alignment, drop, columns, non-vec backing
impl<'t, T> PtrCell<'t, T> {
    pub fn new(value: T) -> Self {
        let len = mem::size_of::<T>();

        let mut data = Vec::<u8>::new();
        data.resize(len, 0); // TODO: ignoring alignment

        let mut storage = unsafe { NonNull::new_unchecked(data.as_mut_ptr()) };

        let ptr = unsafe { PtrOwn::make_into(value, &mut storage) };

        Self {
            data: data,
            ptr: ptr,
            marker: PhantomData,
        }
    }

    pub fn deref(&self) -> &T {
        unsafe { self.ptr.deref() }
    }

    pub fn deref_mut(&self) -> &mut T {
        unsafe { self.ptr.deref_mut() }
    }
}

#[cfg(test)]
mod tests {
    use std::{mem, ptr::NonNull};

    use super::{PtrMut, PtrOwn};

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
    fn test_ptr_own_to_and_from() {
        let test = Test32(1);
        let size = mem::size_of::<Test32>();
        let mut vec = Vec::<u8>::new();
        vec.resize(size, 0);

        let mut data = unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) };

        let ptr =  unsafe { PtrOwn::make_into(test, &mut data) };

        let test2 = unsafe { ptr.as_ref().deref::<Test32>() };

        assert_eq!(format!("{:?}", test2), "Test32(1)");
    }

    #[test]
    fn test_ptr_mut_update() {
        let test = TestValue { value: "test-a".to_string() };
        let ptr: PtrMut =  PtrMut::from(&test);

        update_value(ptr);

        let test2 = unsafe { ptr.as_ref().deref::<TestValue>() };

        assert_eq!(format!("{:?}", test2), "TestValue { value: \"new-a\" }");
    }

/*
    #[test]
    fn ptr_own_u32() {
        let test = Test32(1);
        let ptr =  PtrOwn::spawn(test);
        let test2 = unsafe { ptr.as_ref().deref::<Test32>() };

        assert_eq!(format!("{:?}", test2), "Test32(1)");
    }

*/
    fn update_value(ptr: PtrMut) {
        let test2 = unsafe { ptr.deref_mut::<TestValue>() };
        test2.value = "new-a".to_string();
    }

    #[derive(Debug)]
    struct Test(String);

    #[derive(Debug)]
    struct Test32(u32);

    #[derive(Debug)]
    struct TestValue {
        value: String
    }
}