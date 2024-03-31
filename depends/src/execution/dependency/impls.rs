use crate::error::ResolveResult;
use crate::{DepRef, Dependency, HashValue, Resolve, Visitor};

// todo: all the impls with a macro_rules
pub struct Dependencies2<A, B> {
    a: Dependency<A>,
    b: Dependency<B>,
}

impl<A, B> Dependencies2<A, B>
    where
        A: Resolve,
        for<'a> <A as Resolve>::Output<'a>: HashValue,
        B: Resolve + HashValue,
        for<'a> <B as Resolve>::Output<'a>: HashValue,
{
    pub fn new(a: A, b: B) -> Self {
        Self { a: Dependency::new(a), b: Dependency::new(b) }
    }
}

pub struct DepRef2<'a, A, B> {
    a: DepRef<'a, A>,
    b: DepRef<'a, B>,
}

impl<A, B> Resolve for Dependencies2<A, B>
where
    A: Resolve,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    B: Resolve + HashValue,
    for<'a> <B as Resolve>::Output<'a>: HashValue,
{
    type Output<'a> = DepRef2<'a, A::Output<'a>, B::Output<'a>> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
        Ok(DepRef2 {
            a: self.a.resolve(visitor)?,
            b: self.b.resolve(visitor)?,
        })
    }
}

pub struct Dependencies3<A, B, C> {
    a: Dependency<A>,
    b: Dependency<B>,
    c: Dependency<C>,
}

impl<A, B, C> Dependencies3<A, B, C>
    where
        A: Resolve,
        for<'a> <A as Resolve>::Output<'a>: HashValue,
        B: Resolve + HashValue,
        for<'a> <B as Resolve>::Output<'a>: HashValue,
        C: Resolve,
        for<'a> <C as Resolve>::Output<'a>: HashValue,
{
    pub fn new(a: A, b: B, c: C) -> Self {
        Self { a: Dependency::new(a), b: Dependency::new(b), c: Dependency::new(c) }
    }
}

pub struct DepRef3<'a, A, B, C> {
    a: A,
    b: B,
    c: C,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<A, B, C> Resolve for crate::execution::dependency::Dependencies3<A, B, C>
where
    A: Resolve,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    B: Resolve + HashValue,
    for<'a> <B as Resolve>::Output<'a>: HashValue,
    C: Resolve,
    for<'a> <C as Resolve>::Output<'a>: HashValue,
{
    type Output<'a> = crate::execution::dependency::DepRef3<'a, DepRef<'a, A::Output<'a>>, DepRef<'a, B::Output<'a>>, DepRef<'a, C::Output<'a>>> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
        Ok(crate::execution::dependency::DepRef3 {
            a: self.a.resolve(visitor)?,
            b: self.b.resolve(visitor)?,
            c: self.c.resolve(visitor)?,
            phantom: std::marker::PhantomData,
        })
    }
}
