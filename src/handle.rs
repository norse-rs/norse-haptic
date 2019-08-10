use std::ptr;

#[repr(transparent)]
pub struct Handle<T>(*mut T);

impl<T> Handle<T> {
    pub const NULL: Handle<T> = Handle(ptr::null_mut());

    pub unsafe fn new(value: T) -> Self {
        let value = Box::new(value);
        Handle(Box::into_raw(value))
    }

    pub unsafe fn destroy(self) {
        if !self.0.is_null() {
            Box::from_raw(self.0);
        }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle(self.0)
    }
}

impl<T> Copy for Handle<T> {}

impl<T> std::ops::Deref for Handle<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let value = unsafe { Box::from_raw(self.0) };
        Box::leak(value)
    }
}

impl<T> std::ops::DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value = unsafe { Box::from_raw(self.0) };
        Box::leak(value)
    }
}
