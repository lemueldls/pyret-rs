#[macro_export]
macro_rules! ty {
    ($ident:ident, $predicate:expr) => {
        pub struct $ident(pub ::std::rc::Rc<$crate::value::PyretValue>);

        impl $ident {
            #[must_use]
            pub fn predicate() -> $crate::value::TypePredicate {
                static PREDICATE: ::std::sync::LazyLock<$crate::value::TypePredicate> =
                    ::std::sync::LazyLock::new(|| ::std::sync::Arc::new($predicate));

                ::std::sync::Arc::clone(&*PREDICATE)
            }
        }

        impl ::std::ops::Deref for $ident {
            type Target = ::std::rc::Rc<$crate::value::PyretValue>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
