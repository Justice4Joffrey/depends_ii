use crate::error::ResolveResult;
use crate::{DepRef, Dependency, HashValue, IsDirty, Resolve, Visitor};

// todo: all the impls with a macro_rules
pub struct Dependencies2<A, B> {
    a: Dependency<A>,
    b: Dependency<B>,
}

impl<A, B> Dependencies2<A, B>
where
    A: Resolve,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    B: Resolve,
    for<'a> <B as Resolve>::Output<'a>: HashValue,
{
    pub fn new(a: Dependency<A>, b: Dependency<B>) -> Self {
        // TODO: create the Dependency here
        Self { a, b }
    }
}

pub struct DepRef2<'a, A, B> {
    pub a: DepRef<'a, A>,
    pub b: DepRef<'a, B>,
}

impl<A, B> IsDirty for DepRef2<'_, A, B>
where
    A: IsDirty,
    B: IsDirty,
{
    fn is_dirty(&self) -> bool {
        self.a.is_dirty() || self.b.is_dirty()
    }
}

impl<A, B> Resolve for Dependencies2<A, B>
where
    A: Resolve,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    B: Resolve,
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
    B: Resolve,
    for<'a> <B as Resolve>::Output<'a>: HashValue,
    C: Resolve,
    for<'a> <C as Resolve>::Output<'a>: HashValue,
{
    pub fn new(a: Dependency<A>, b: Dependency<B>, c: Dependency<C>) -> Self {
        // TODO: create the Dependency here
        Self { a, b, c }
    }
}

pub struct DepRef3<'a, A, B, C> {
    a: A,
    b: B,
    c: C,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<A, B, C> IsDirty for DepRef3<'_, A, B, C>
where
    A: IsDirty,
    B: IsDirty,
    C: IsDirty,
{
    fn is_dirty(&self) -> bool {
        self.a.is_dirty() || self.b.is_dirty() || self.c.is_dirty()
    }
}

impl<A, B, C> Resolve for crate::execution::dependency::Dependencies3<A, B, C>
where
    A: Resolve,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    B: Resolve,
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
