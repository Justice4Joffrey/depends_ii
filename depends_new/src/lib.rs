use core::cell::RefCell;
use std::cell::Ref;
use std::ops::Deref;
use std::sync::Arc;

pub trait UpdateTarget<T> {
    fn update(&mut self, value: T);
}

pub trait NumberLike {
    fn value(&self) -> i32;
}
impl NumberLike for Ref<'_, Number>  {
    fn value(&self) -> i32 {
        println!("numberlike {}", self.deref().value);
        self.deref().value
    }
}

impl NumberLike for Number {
    fn value(&self) -> i32 {
        self.value
    }
}

pub struct Sum {
    pub value: i32,
}

pub struct Number {
    pub value: i32,
}
impl UpdateInput for Number {
    type Update = i32;
    fn update(&mut self, value: i32) {
        self.value = value;
    }
}

pub struct DepRef2<'a, A, B> {
    a: A,
    b: B,
    phantom: std::marker::PhantomData<&'a ()>,
}

pub struct Dependencies2<A, B> {
    a: Dependency<A>,
    b: Dependency<B>,
}

impl<A, B> Resolve for Dependencies2<A, B> where
    A: Resolve,
        // for<'a> <T as Resolve>::Output<'a>: HashValue,
    B: Resolve
{
    type Output<'a> = DepRef2<'a, A::Output<'a>, B::Output<'a>> where Self: 'a;

    fn resolve(&self) -> Self::Output<'_> {
        println!("Dependencies2::resolve");
        DepRef2 {
            a: self.a.resolve(),
            b: self.b.resolve(),
            phantom: std::marker::PhantomData,
        }
    }
}

pub struct Dependency<T> {
    value: T,
}

pub trait Resolve {
    type Output<'a> where Self: 'a;
    fn resolve(&self) -> Self::Output<'_>;
}

impl<T> Resolve for Dependency<T> where T: Resolve {
    type Output<'a> = T::Output<'a> where Self: 'a;
    fn resolve(&self) -> Self::Output<'_> {
        self.value.resolve()
    }
}

impl<'a, A, B> UpdateTarget<DepRef2<'a, A, B>> for Sum where A: NumberLike, B: NumberLike {
    fn update(&mut self, value: DepRef2<'a, A, B>) {
        self.value = value.a.value() + value.b.value();
    }
}

pub struct DerivedNode<D, T> {
    dependencies: D,
    value: RefCell<T>,
}

pub struct InputNode<T> {
    value: RefCell<T>,
}

pub trait UpdateInput {
    type Update;
    fn update(&mut self, value: Self::Update);
}

impl<T> InputNode<T> where T: UpdateInput {
    pub fn new(value: T) -> Arc<Self> {
        Arc::new(Self {
            value: RefCell::new(value),
        })
    }

    pub fn update(&self, value: T::Update) {
        self.value.borrow_mut().update(value);
    }
}

impl<T> Resolve for InputNode<T> {
    type Output<'a> = Ref<'a, T> where Self: 'a;

    fn resolve(&self) -> Self::Output<'_> {
        self.value.borrow()
    }
}

impl<D, T> Resolve for DerivedNode<D, T>
    where
            for<'a> D: Resolve + 'a,
            for<'a> T: UpdateTarget<<D as Resolve>::Output<'a>> + 'a,
{
    type Output<'a> = Ref<'a, T> where Self: 'a;

    fn resolve(&self) -> Self::Output<'_> {
        let v = self.dependencies.resolve();
        {
            let mut m = self.value.borrow_mut();
            m.update(v);
        }
        self.value.borrow()
    }
}

impl<D, T> DerivedNode<D, T>
    where
            for<'a> D: Resolve + 'a,
            for<'a> T: UpdateTarget<<D as Resolve>::Output<'a>> + 'a,
{
    /// Create this node with a specified Id. Useful for tests.
    pub fn new_with_id(dependencies: D, value: T, id: usize) -> Arc<Self> {
        // TODO: we should store `update` and make the `update_derived` call
        //  take a &self so that values can be provided for update fns.
        Arc::new(Self {
            dependencies,
            value: RefCell::new(value),
        })
    }
}

impl<T> Resolve for Arc<T> where T: Resolve{
    type Output<'a> = T::Output<'a> where Self: 'a;
    fn resolve(&self) -> Self::Output<'_> {
        self.as_ref().resolve()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = InputNode::new(Number { value: 1 }) ;
        let b = InputNode::new(Number { value: 2 }) ;
        let sum = DerivedNode::new_with_id(
            Dependencies2 {
                a: Dependency { value: a },
                b: Dependency { value: b },
            },
            Sum { value: 0 },
            0,
        );
        println!("sum");
        let ans = sum.resolve();
        println!("sum fone");
        assert_eq!(ans.value, 3);
    }
}
