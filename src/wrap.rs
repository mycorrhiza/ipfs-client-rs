#[macro_export]
macro_rules! wrapped_future {
    ($item:ty, $wrapper:ident($wrappee:ty)) => {
        #[allow(missing_debug_implementations)]
        pub struct $wrapper(pub(crate) $wrappee);

        impl ::futures::Future for $wrapper {
            type Item = $item;
            type Error = ::errors::Error;

            fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error> {
                self.0.poll()
            }
        }

        impl<T: Into<$wrappee>> From<T> for $wrapper {
            fn from(f: T) -> $wrapper {
                $wrapper(f.into())
            }
        }
    };

    ($item:ty, $wrapper:ident<$($t:ident:$tt:tt),+>($wrappee:ty)) => {
        #[allow(missing_debug_implementations)]
        pub struct $wrapper<$($t:$tt),+>(pub(crate) $wrappee);

        impl<$($t: $tt),+> ::futures::Future for $wrapper<$($t),+> {
            type Item = $item;
            type Error = ::errors::Error;

            fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error> {
                self.0.poll()
            }
        }
    };
}
