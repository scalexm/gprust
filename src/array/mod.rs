pub mod iterator;

use wrapper::ffi;
use wrapper::types::mem::Buffer;
use std::marker::PhantomData;

type Shape = (usize, usize, usize);

pub struct Array<T> {
    shape: Shape,
    buffer: Buffer,
    _marker: PhantomData<*const T>,
}

impl<T> Array<T> {
    /*pub fn from_vec(vec: Vec<T>, shape: Shape) -> Array<T> {

    }*/
}
