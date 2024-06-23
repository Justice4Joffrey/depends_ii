use crate::error::ResolveResult;
use crate::{DepRef, Dependency, HashValue, IsDirty, Resolve, SingleRef, Visitor};

pub struct ThangDep<A, B> {
    pub a: Dependency<A>,
    pub b: Dependency<B>,
}
impl<A, B> ThangDep<A, B>
where
    A: Resolve,
    B: Resolve,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    for<'a> <B as Resolve>::Output<'a>: HashValue,
{
    pub fn new(a: A, b: B) -> Self {
        Self {
            a: Dependency::new(a),
            b: Dependency::new(b),
        }
    }
}
pub struct ThangRef<'a> {
    pub this: DepRef<'a, i32>,
    pub that: DepRef<'a, String>,
}
impl IsDirty for ThangRef<'_> {
    fn is_dirty(&self) -> bool {
        self.this.is_dirty() || self.that.is_dirty()
    }
}
impl<A, B> Resolve for ThangDep<A, B>
where
    for<'a> A: Resolve<Output<'a> = i32> + 'a,
    for<'a> B: Resolve<Output<'a> = String> + 'a,
    for<'a> <A as Resolve>::Output<'a>: HashValue,
    for<'a> <B as Resolve>::Output<'a>: HashValue,
{
    type Output<'a> = ThangRef<'a>
        where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
        Ok(ThangRef {
            this: self.a.resolve(visitor)?,
            that: self.b.resolve(visitor)?,
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
