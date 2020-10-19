use std::net::SocketAddr;

pub type NetworkTaskResource = Vec<NetworkTask>;

pub enum NetworkTask {
    AttachPublicId(SocketAddr, u16),
}
