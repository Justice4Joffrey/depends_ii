use crate::error::ResolveResult;
use crate::{DepRef, Dependency, HashValue, IsDirty, Resolve, SingleRef, Visitor};

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

// pub struct MyThing<T> {
//     my_int: i32,
//     my_string: String,
//     some_t: T,
// }
//
// impl<C> MyThing<C>
// where
//     C: Resolve,
//     for<'a> <C as Resolve>::Output<'a>: HashValue,
// {
//     pub fn init<A, B>(a: A, b: B, c: C) -> MyThingDep<A, B, C>
//     where
//         for<'a> A: Resolve<Output<'a> = SingleRef<'a, i32>> + 'a,
//         for<'a> B: Resolve<Output<'a> = SingleRef<'a, String>> + 'a,
//         for<'a> <A as Resolve>::Output<'a>: HashValue,
//         for<'a> <B as Resolve>::Output<'a>: HashValue,
//     {
//         MyThingDep {
//             my_int: a,
//             my_string: b,
//             some_t: c,
//         }
//     }
// }
//
// pub struct MyThingRef<'a, T> {
//     my_int: DepRef<'a, i32>,
//     my_string: DepRef<'a, String>,
//     some_t: DepRef<'a, T>,
// }
//
// // Generics
// pub struct MyThingDep<A, B, C> {
//     my_int: A,
//     my_string: B,
//     some_t: C,
// }
//
// // Generics
// // TODO: constriant the generics
// impl<A, B, C> Resolve for MyThingDep<A, B, C>
// where
//     for<'a> A: Resolve<Output<'a> = SingleRef<'a, i32>> + 'a,
//     for<'a> B: Resolve<Output<'a> = SingleRef<'a, String>> + 'a,
//     C: Resolve,
//     for<'a> <A as Resolve>::Output<'a>: HashValue,
//     for<'a> <B as Resolve>::Output<'a>: HashValue,
//     // TODO: you'd have some trait bounds here on the generic
//     for<'a> <C as Resolve>::Output<'a>: HashValue,
// {
//     type Output<'a> = MyThingRef<'a, C::Output<'a>>
//         where Self: 'a;
//
//     fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
//         Ok(MyThingRef {
//             my_int: self.my_int.resolve(visitor)?,
//             my_string: self.my_string.resolve(visitor)?,
//             some_t: self.some_t.resolve(visitor)?,
//         })
//     }
// }
