use crate::{DepRef, HashValue, IsDirty, Resolve};

pub struct Dependency2<A, B> {
    a: crate::Dependency<::std::rc::Rc<A>>,
    b: crate::Dependency<::std::rc::Rc<B>>,
}

pub struct Dependency3<A, B, C> {
    a: crate::Dependency<::std::rc::Rc<A>>,
    b: crate::Dependency<::std::rc::Rc<B>>,
    c: crate::Dependency<::std::rc::Rc<C>>,
}

impl<A, B, C> crate::Named for Dependency3<A, B, C> {
    fn name() -> &'static str {
        "Dependency3"
    }
}

// TODO: I think we might have to be more explicit here like below
// struct Dependency3Ref<'a, A, B, C> {
// a: crate::DepRef<'a, ::std::cell::Ref<'a, crate::NodeState<A>>>,
// b: crate::DepRef<'a, ::std::cell::Ref<'a, crate::NodeState<B>>>,
// c: crate::DepRef<'a, ::std::cell::Ref<'a, crate::NodeState<C>>>,
// }
struct Dependency3Ref<'a, A, B, C> {
    pub a: DepRef<'a, A>,
    pub b: DepRef<'a, B>,
    pub c: DepRef<'a, C>,
}

impl<A, B, C> Dependency3<A, B, C>
//TODO
// where
// for<'a> A: crate::Resolve<Output<'a> = ::std::cell::Ref<'a, crate::NodeState<Node1>>> + 'a,
// for<'a> B: crate::Resolve<Output<'a> = ::std::cell::Ref<'a, crate::NodeState<Node2>>> + 'a,
// for<'a> C: crate::Resolve<Output<'a> = ::std::cell::Ref<'a, crate::NodeState<Node3>>> + 'a,
where
    A: Resolve,
    for<'a> <A as Resolve>::Output<'a>: HashValue + 'a,
    B: Resolve,
    for<'a> <B as Resolve>::Output<'a>: HashValue + 'a,
    C: Resolve,
    for<'a> <C as Resolve>::Output<'a>: HashValue + 'a,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(a: ::std::rc::Rc<A>, b: ::std::rc::Rc<B>, c: ::std::rc::Rc<C>) -> Self {
        Self {
            a: crate::Dependency::new(a),
            b: crate::Dependency::new(b),
            c: crate::Dependency::new(c),
        }
    }
}
// TODO
// impl<A, B, C> From<(::std::rc::Rc<A>, ::std::rc::Rc<B>, ::std::rc::Rc<C>)> for Dependency3<A, B, C>
// where
//     for<'a> A: crate::Resolve<Output<'a> = ::std::cell::Ref<'a, crate::NodeState<Node1>>> + 'a,
//     for<'a> B: crate::Resolve<Output<'a> = ::std::cell::Ref<'a, crate::NodeState<Node2>>> + 'a,
//     for<'a> C: crate::Resolve<Output<'a> = ::std::cell::Ref<'a, crate::NodeState<Node3>>> + 'a,
// {
//     fn from((a, b, c): (::std::rc::Rc<A>, ::std::rc::Rc<B>, ::std::rc::Rc<C>)) -> Self {
//         Self {
//             a: crate::Dependency::new(a),
//             b: crate::Dependency::new(b),
//             c: crate::Dependency::new(c),
//         }
//     }
// }
impl<A, B, C> crate::Resolve for Dependency3<A, B, C>
where
    A: Resolve,
    for<'a> <A as Resolve>::Output<'a>: HashValue + 'a,
    B: Resolve,
    for<'a> <B as Resolve>::Output<'a>: HashValue + 'a,
    C: Resolve,
    for<'a> <C as Resolve>::Output<'a>: HashValue + 'a,
    // for<'a> C:
    // ::depends::Resolve<Output<'a> = ::std::cell::Ref<'a, ::depends::NodeState<Node3>>> + 'a,
{
    type Output < 'a > = Dependency3Ref< 'a,
A::Output<'a>, B::Output<'a>, C::Output<'a>> where Self : 'a ;

    fn resolve(
        &self,
        visitor: &mut impl crate::Visitor,
    ) -> crate::error::ResolveResult<Self::Output<'_>> {
        use crate::Named;
        visitor.touch_dependency_group(Self::name());
        Ok(Dependency3Ref {
            a: self.a.resolve(visitor)?,
            b: self.b.resolve(visitor)?,
            c: self.c.resolve(visitor)?,
        })
    }
}
impl<A, B, C> crate::IsDirty for Dependency3Ref<'_, A, B, C>
where
    A: IsDirty,
    B: IsDirty,
    C: IsDirty,
{
    fn is_dirty(&self) -> bool {
        self.a.is_dirty() || self.b.is_dirty() || self.c.is_dirty()
    }
}
