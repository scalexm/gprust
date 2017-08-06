use std::marker::PhantomData;
use std::fmt;
use wrapper::types::command_queue;

pub trait Functor<T> {
    type Output;

    fn arity() -> usize;
}

pub trait GpuIterator: Sized {
    type Item;

    fn map<F>(self, f: F) -> Map<Self, F> where F: Functor<Self::Item> {
        Map {
            iter: self,
            f,
            queue: self.command_queue(),
        }
    }

    fn write<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;
    fn into_command_queue(self) -> CommandQueue;
}

pub trait IntoGpuIterator {
    type Item;
    type Iterator;

    fn into_iter(self, queue: CommandQueue) -> Self::Iterator;
}

impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type Iterator = Self;

    fn into_iter(self, _: CommandQueue) -> Self::Iterator {
        self
    }
}

pub trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = A>;
}

struct Map<I, F> {
    iter: I,
    f: F,
}

impl<I: Iterator, F> Iterator for Map<I, F> where F: Functor<I::Item> + fmt::Display {
    type Item = F::Output;

    fn write<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "{}(", self.f)?;
        self.iter.write(w)?;
        write!(w, ")")
    }
}

impl<I, F> fmt::Display for Map<I, F> where I: fmt::Display, F: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", &self.iter, &self.f)
    }
}

#[allow(non_camel_case_types)]
struct cos;

impl Functor<f32> for cos {
    type Output = f32;

    fn arity() -> usize {
        1
    }
}

impl fmt::Display for cos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cos")
    }
}

#[allow(non_camel_case_types)]
struct sin;

impl Functor<f32> for sin {
    type Output = f32;

    fn arity() -> usize {
        1
    }
}

impl fmt::Display for sin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sin")
    }
}

struct IntoIter<T> {
    _marker: PhantomData<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn write<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write!(w, "input[get_global_id(0)]")
    }
}

impl<T> fmt::Display for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "into")
    }
}

#[test]
fn test_test() {
    let a = IntoIter::<f32> { _marker: PhantomData };
    let mut s = String::new();
    a.map(cos).map(sin).map(cos).map(sin).write(&mut s).unwrap();
    println!("{}", s);
}
