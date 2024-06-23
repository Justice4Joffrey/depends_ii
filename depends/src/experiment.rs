use crate::error::ResolveResult;
use crate::{DepRef, Dependency, HashValue, IsDirty, Resolve, Visitor};

pub struct Thang<T> {
    pub this: i32,
    pub that: String,
    pub him: T,
}

impl<T> Thang<T>
where
    for<'a> T: Resolve + 'a,
    for<'a> <T as Resolve>::Output<'a>: HashValue,
    // TODO: constraints for C::Output
{
    pub fn init<A, B>(this: A, that: B, him: T) -> ThangDep<A, B, T>
    where
        for<'a> A: Resolve<Output<'a> = i32> + 'a,
        for<'a> B: Resolve<Output<'a> = String> + 'a,
        for<'a> <A as Resolve>::Output<'a>: HashValue,
        for<'a> <B as Resolve>::Output<'a>: HashValue,
    {
        ThangDep::new(
            this,
            that,
            him,
        )
    }
}

pub struct ThangDep<A, B, C> {
    pub a: Dependency<A>,
    pub b: Dependency<B>,
    pub c: Dependency<C>,
}
impl<A, B, C> ThangDep<A, B, C>
where
    for<'a> A: Resolve<Output<'a> = i32> + 'a,
    for<'a> B: Resolve<Output<'a> = String> + 'a,
    for<'a> C: Resolve + 'a,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    for<'a> <B as Resolve>::Output<'a>: HashValue,
    for<'a> <C as Resolve>::Output<'a>: HashValue,
// TODO: constraints for C::Output
{
    pub fn new(a: A, b: B, c: C) -> Self {
        Self {
            a: Dependency::new(a),
            b: Dependency::new(b),
            c: Dependency::new(c),
        }
    }
}
pub struct ThangRef<'a, T> {
    pub this: DepRef<'a, i32>,
    pub that: DepRef<'a, String>,
    pub him: DepRef<'a, T>,
}
impl<T> IsDirty for ThangRef<'_, T> {
    fn is_dirty(&self) -> bool {
        self.this.is_dirty() || self.that.is_dirty() || self.him.is_dirty()
    }
}
impl<A, B, C> Resolve for ThangDep<A, B, C>
where
    for<'a> A: Resolve<Output<'a> = i32> + 'a,
    for<'a> B: Resolve<Output<'a> = String> + 'a,
    for<'a> C: Resolve + 'a,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    for<'a> <B as Resolve>::Output<'a>: HashValue,
    for<'a> <C as Resolve>::Output<'a>: HashValue,
    // TODO: constraints for C::Output
{
    type Output<'a> = ThangRef<'a, <C as Resolve>::Output<'a>>
        where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
        Ok(ThangRef {
            this: self.a.resolve(visitor)?,
            that: self.b.resolve(visitor)?,
            him: self.c.resolve(visitor)?,
        })
    }
}
