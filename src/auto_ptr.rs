


use std::{cell::Cell, hint::unreachable_unchecked, mem::transmute, ops::{Deref, DerefMut}, process, ptr::{self, NonNull}};

pub struct AutoPtr<T: ?Sized>(
    Cell<Option<NonNull<T>>>,
);

impl<T: ?Sized> AutoPtr<T> {
    pub fn new(value: T) -> Self
    where 
        T: Sized
    {
        let ptr = Box::into_raw(Box::new(value));
        Self(NonNull::new(ptr).into())
    }

    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self(NonNull::new(ptr).into())
    }
}


impl<T: ?Sized> Clone for AutoPtr<T> {
    fn clone(&self) -> Self {
        let i = self.0.replace(None);
        Self(i.into())
    }
}

impl<T: ?Sized> Drop for AutoPtr<T> {
    fn drop(&mut self) {
        if let Some(ptr) = self.0.get() {
            let _ = unsafe { Box::from_raw(ptr.as_ptr())};
        }
    }
}

impl<T: ?Sized> Deref for AutoPtr<T> {

    type Target = T;
    
    fn deref(&self) -> &Self::Target {

        return unsafe{ transmute(self.0.get()) }
    //     if let Some(r) =  self.0.get() {
    //         return unsafe{ r.as_ref() };
    //     }
    //     unsafe {
    //         let _trap = std::ptr::read_volatile(0x0 as *const u8);
    //         unreachable_unchecked()
    //     }
    }
}

impl<T: ?Sized> DerefMut for AutoPtr<T> {
    
    fn deref_mut(&mut self) -> &mut Self::Target {
        
        return unsafe{ transmute(self.0.get()) }
        // if let Some(mut r) =  self.0.get() {
        //     return unsafe{ r.as_mut() };
        // }
        // unsafe {
        //     let _trap = std::ptr::read_volatile(0x0 as *const u8);
        //     unreachable_unchecked()
        // }
    }
}
