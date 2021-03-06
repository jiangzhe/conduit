use std::net::SocketAddr;
use std::sync::Arc;

use control::pb::common::Protocol;

use ctx;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ctx {
    Client(Arc<Client>),
    Server(Arc<Server>),
}

/// Identifies a connection from another process to a proxy listener.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Server {
    pub proxy: Arc<ctx::Proxy>,
    pub remote: SocketAddr,
    pub local: SocketAddr,
    pub orig_dst: Option<SocketAddr>,
    pub protocol: Protocol,
}

/// Identifies a connection from the proxy to another process.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Client {
    pub proxy: Arc<ctx::Proxy>,
    pub remote: SocketAddr,
    pub protocol: Protocol,
}

impl Ctx {
    pub fn proxy(&self) -> &Arc<ctx::Proxy> {
        match *self {
            Ctx::Client(ref ctx) => &ctx.proxy,
            Ctx::Server(ref ctx) => &ctx.proxy,
        }
    }
}

impl Server {
    pub fn new(
        proxy: &Arc<ctx::Proxy>,
        local: &SocketAddr,
        remote: &SocketAddr,
        orig_dst: &Option<SocketAddr>,
        protocol: Protocol,
    ) -> Arc<Server> {
        let s = Server {
            proxy: Arc::clone(proxy),
            local: *local,
            remote: *remote,
            orig_dst: *orig_dst,
            protocol: protocol,
        };

        Arc::new(s)
    }
}

impl Client {
    pub fn new(
        proxy: &Arc<ctx::Proxy>,
        remote: &SocketAddr,
        protocol: Protocol,
    ) -> Arc<Client> {
        let c = Client {
            proxy: Arc::clone(proxy),
            remote: *remote,
            protocol: protocol,
        };

        Arc::new(c)
    }
}

impl From<Arc<Client>> for Ctx {
    fn from(c: Arc<Client>) -> Self {
        Ctx::Client(c)
    }
}

impl From<Arc<Server>> for Ctx {
    fn from(s: Arc<Server>) -> Self {
        Ctx::Server(s)
    }
}
