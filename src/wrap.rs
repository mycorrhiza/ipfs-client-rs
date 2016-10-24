#[macro_export]
macro_rules! wrapped_future {
    ($wrapper:ident($wrappee:ty)) => {
        pub struct $wrapper(pub(crate) $wrappee);

        impl ::futures::Future for $wrapper {
            type Item = <$wrappee as ::futures::Future>::Item;
            type Error = ::errors::Error;

            fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error> {
                self.0.poll()
            }
        }
    };

    ($wrapper:ident<$($t:ident:$tt:tt),+>($wrappee:ty)) => {
        pub struct $wrapper<$($t:$tt),+>(pub(crate) $wrappee);

        impl<$($t: $tt),+> ::futures::Future for $wrapper<$($t),+> {
            type Item = <$wrappee as ::futures::Future>::Item;
            type Error = ::errors::Error;

            fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error> {
                self.0.poll()
            }
        }
    };
}
