use fetch::FetchJsonFuture;
use data;

macro_rules! data_future {
    ($t:ident) => {
        wrapped_future!($t(FetchJsonFuture<data::$t>));
    };
}

data_future!(Version);
