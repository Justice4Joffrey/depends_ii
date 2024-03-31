use core::cell::RefCell;
use std::cell::Ref;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

pub trait UpdateTarget<T, F> {
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

pub struct Number {
    pub value: i32,
}
impl UpdateInput for Number {
    type Update = i32;
    fn update(&mut self, value: i32) {
        self.value = value;
    }
}

pub struct Dependency<T> {
    value: T,
}
pub struct Dependencies2<A, B> {
    a: Dependency<A>,
    b: Dependency<B>,
}

pub struct DepRef<'a, A> {
    a: A,
    phantom: std::marker::PhantomData<&'a ()>,
}

pub struct DepRef2<'a, A, B> {
    a: DepRef<'a,A>,
    b: DepRef<'a, B>,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<A, B> Resolve for Dependencies2<A, B>
where
    A: Resolve,
    B: Resolve ,
{
    type Output<'a> = DepRef2<'a, A::Output<'a>, B::Output<'a>> where Self: 'a;

    fn resolve(&self, ) -> Self::Output<'_> {
        DepRef2 {
            a: DepRef {a: self.a.resolve(), phantom: PhantomData},
            b: DepRef {a: self.b.resolve(), phantom: PhantomData},
            phantom: std::marker::PhantomData,
        }
    }
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

struct SumIt;

struct MultiplyIt;

impl<'a, A, B> UpdateTarget<DepRef2<'a, A, B>, SumIt> for Number where A: NumberLike, B: NumberLike {
    fn update(&mut self, value: DepRef2<'a, A, B>) {
        self.value = value.a.a.value() + value.b.a.value();
    }
}

impl<'a, A, B> UpdateTarget<DepRef2<'a, A, B>, MultiplyIt> for Number where A: NumberLike, B: NumberLike {
    fn update(&mut self, value: DepRef2<'a, A, B>) {
        self.value = value.a.a.value() * value.b.a.value();
    }
}

pub struct DerivedNode<D, T, F> {
    dependencies: D,
    value: RefCell<T>,
    phantom: PhantomData<F>,
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

impl<D, T, F> Resolve for DerivedNode<D, T, F>
    where
            for<'a> D: Resolve + 'a,
            for<'a> T: UpdateTarget<<D as Resolve>::Output<'a>, F> + 'a,
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

impl<D, T, F> DerivedNode<D, T, F>
    where
            for<'a> D: Resolve + 'a,
            for<'a> T: UpdateTarget<<D as Resolve>::Output<'a>, F> + 'a,
{
    /// Create this node with a specified Id. Useful for tests.
    pub fn new_with_id(dependencies: D, value: T, id: usize) -> Arc<Self> {
        // TODO: we should store `update` and make the `update_derived` call
        //  take a &self so that values can be provided for update fns.
        Arc::new(Self {
            dependencies,
            value: RefCell::new(value),
            phantom: Default::default(),
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
        let c = InputNode::new(Number { value: 10 }) ;
        let sum = DerivedNode::<_, _, SumIt>::new_with_id(
            Dependencies2 {
                a: Dependency { value: a },
                b: Dependency { value: b },
            },
            Number{ value: 0 },
            0,
        );
        let multi = DerivedNode::<_, _, MultiplyIt>::new_with_id(
            Dependencies2 {
                a: Dependency { value: sum },
                b: Dependency { value: c },
            },
            Number{ value: 0 },
            0,
        );
        println!("sum");
        let ans = multi.resolve();
        println!("sum fone");
        assert_eq!(ans.value, 30);
    }
}
