use std::marker::PhantomData;

pub struct Task<T> {
    _type: PhantomData<T>,
}
