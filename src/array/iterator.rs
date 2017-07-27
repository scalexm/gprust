use std::marker::PhantomData;
use std::fmt;

trait Functor<T> {
    type Output;
}

trait Iterator: Sized {
    type Item;

    fn transform<F>(self, f: F) -> Transform<Self, F> where F: Functor<Self::Item> {
        Transform { iter: self, f }
    }
}

struct Transform<I, F> {
    iter: I,
    f: F,
}

impl<I: Iterator, F> Iterator for Transform<I, F> where F: Functor<I::Item> {
    type Item = F::Output;
}

impl<I, F> fmt::Display for Transform<I, F> where I: fmt::Display, F: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", &self.iter, &self.f)
    }
}

struct Cos;

impl Functor<f32> for Cos {
    type Output = f32;
}

impl fmt::Display for Cos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cos")
    }
}

struct Sin;

impl Functor<f32> for Sin {
    type Output = f32;
}

impl fmt::Display for Sin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sin")
    }
}

struct IntoIter<T> {
    _marker: PhantomData<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
}

impl<T> fmt::Display for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"\"")
    }
}

#[test]
fn test_test() {
    let a = IntoIter::<f32> { _marker: PhantomData };
    let a = a.transform(Cos);
    let a = a.transform(Sin);
    let a = a.transform(Cos);
    let a = a.transform(Sin);
    println!("{}", a);
}
