pub mod iterator;

use wrapper::ffi;
use wrapper::types::mem::Buffer;
use wrapper::types::kernel::Kernel;
use wrapper::types::program;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::sync::RwLock;
use std::mem;

type Shape = (usize, usize, usize);

lazy_static! {
    static ref KERNELS: RwLock<HashMap<String, Kernel>> = RwLock::new(HashMap::new());
}

pub struct Array<T> {
    shape: Shape,
    buffer: Buffer,
    _marker: PhantomData<*const T>,
}

pub struct ArrayOperation<T> {
    kernel: Kernel,
    _marker: PhantomData<*const T>,
}

impl<T> FromIterator<T> for ArrayOperation<T> {
    fn from_iter(iter: T) -> ArrayOperation<T>
        where T: IntoIterator
    {
        let mut code = String::new();
        write!(code, "__kernel void operation(float * input, float * output) \{\n");
        write!(code, "output[get_global_id(0)] = ");
        iter.write(&mut code);
        write!(code, ";\n\}");

        let lock = KERNELS.read().unwrap();
        if Some(ref kernel) = lock.get(&code) {
            return ArrayOperation {
                kernel: kernel.clone(),
            };
        }

        mem::drop(lock);

        let program = program::Builder::create_with_sources(Some(&code), )
    }
}
