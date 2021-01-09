use std::net::SocketAddr;

pub type NetworkTaskResource = Vec<NetworkTask>;

pub enum NetworkTask {
    AttachEntity {
        address: SocketAddr,
        external_id: u16,
    },
}
