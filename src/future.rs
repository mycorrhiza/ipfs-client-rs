macro_rules! future {
    ($p:path, $t:ident) => {
        wrapped_future!($p, $t(::fetch::FetchJsonFuture<$p>));
    };
}

future!(::data::PeerInfo, PeerInfo);
future!(::data::Version, Version);

pub mod swarm {
    future!(::data::swarm::Addresses, Addresses);
    future!(::data::swarm::PeerAddresses, PeerAddresses);
    wrapped_future!(Result<Vec<String>, String>, ConnectResult(::futures::Map<::fetch::FetchJsonFuture<::data::swarm::ConnectResultData>, fn(::data::swarm::ConnectResultData) -> Result<Vec<String>, String>>));
}
