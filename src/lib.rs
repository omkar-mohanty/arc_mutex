use std::ptr::NonNull;

pub struct ArcMutex<T: Sized> {
    inner: NonNull<ArcInner<T>> 
}

struct ArcInner<T> {
    inner: T
}
