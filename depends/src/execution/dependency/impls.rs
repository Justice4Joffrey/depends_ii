use crate::error::ResolveResult;
use crate::{DepRef, Dependency, HashValue,Named, IsDirty, Resolve, Visitor};

macro_rules! generate_dependencies {
    ($count:expr, $($param:ident),*) => {
        paste::paste! {
            pub struct [<Dependencies $count>]<$($param),*> {
                $(pub [< $param:lower >]: Dependency<$param>,)*
            }

            impl<$($param),*> [<Dependencies $count>]<$($param),*>
            where
                $($param: Resolve,)*
                $(for<'a> <$param as Resolve>::Output<'a>: HashValue,)*
            {
                pub fn new($([< $param:lower >]: $param),*) -> Self {
                    Self {
                        $([< $param:lower >]: Dependency::new([< $param:lower >]),)*
                    }
                }
            }

            impl<$($param),*> From<($($param),*)> for [<Dependencies $count>]<$($param),*>
            where
                $($param: Resolve,)*
                $(for<'a> <$param as Resolve>::Output<'a>: HashValue,)*
            {
                fn from(($([< $param:lower >],)*): ($($param),*)) -> Self {
                    Self::new($([< $param:lower >]),*)
                }
            }

            pub struct [<DepRef $count>]<'a, $($param),*> {
                $(pub [< $param:lower >]: DepRef<'a, $param>,)*
            }

            impl<$($param),*> IsDirty for [<DepRef $count>]<'_, $($param),*> {
                fn is_dirty(&self) -> bool {
                    $(self.[< $param:lower >].is_dirty() )||*
                }
            }

            impl<$($param),*> Named for [<Dependencies $count>]<$($param),*> {
                fn name() -> &'static str {
                    concat!("Dependencies", stringify!($count))
                }
            }

            impl<$($param),*> Resolve for [<Dependencies $count>]<$($param),*>
            where
                $($param: Resolve,)*
                $(for<'a> <$param as Resolve>::Output<'a>: HashValue,)*
            {
                type Output<'a> = [<DepRef $count>]<'a, $($param::Output<'a>),*>
                where
                    Self: 'a;

                fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
                    visitor.touch_dependency_group(Self::name());
                    Ok([<DepRef $count>] {
                        $([< $param:lower >]: self.[< $param:lower >].resolve(visitor)?,)*
                    })
                }
            }
        }
    };
}

generate_dependencies!(2, A, B);
generate_dependencies!(3, A, B, C);
generate_dependencies!(4, A, B, C, D);
generate_dependencies!(5, A, B, C, D, E);
generate_dependencies!(6, A, B, C, D, E, F);
generate_dependencies!(7, A, B, C, D, E, F, G);
generate_dependencies!(8, A, B, C, D, E, F, G, H);
generate_dependencies!(9, A, B, C, D, E, F, G, H, I);
generate_dependencies!(10, A, B, C, D, E, F, G, H, I, J);
generate_dependencies!(11, A, B, C, D, E, F, G, H, I, J, K);
generate_dependencies!(12, A, B, C, D, E, F, G, H, I, J, K, L);
generate_dependencies!(13, A, B, C, D, E, F, G, H, I, J, K, L, M);
generate_dependencies!(14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
generate_dependencies!(15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
generate_dependencies!(16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
