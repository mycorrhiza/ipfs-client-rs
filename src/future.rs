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
}
