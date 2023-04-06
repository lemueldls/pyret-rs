#[macro_export]
macro_rules! ty {
    ($ident:ident, $predicate:expr) => {
        pub struct $ident(pub $crate::value::PyretValue);

        impl $ident {
            #[must_use]
            pub fn predicate() -> $crate::value::TypePredicate {
                static PREDICATE: ::std::sync::LazyLock<$crate::value::TypePredicate> =
                    ::std::sync::LazyLock::new(|| ::std::sync::Arc::new($predicate));

                ::std::sync::Arc::clone(&*PREDICATE)
            }

            pub fn register(
                context: $crate::value::context::Context,
            ) -> $crate::PyretResult<$crate::value::TypePredicate> {
                context.register_builtin_type(stringify!($ident), ::std::sync::Arc::new($predicate))
            }
        }

        impl ::std::ops::Deref for $ident {
            type Target = $crate::value::PyretValue;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
