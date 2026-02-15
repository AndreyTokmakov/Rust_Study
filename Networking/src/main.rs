#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

#[path = "tokio/tokio_asynch_tcp_client.rs"] pub mod tokio_asynch_tcp_client;
#[path = "tokio/tokio_asynch_tcp_server.rs"] pub mod tokio_asynch_tcp_server;

#[path = "mio/Servers.rs"] pub mod mio_servers;

#[path = "std/client.rs"] pub mod client;
#[path = "std/tcp_servers.rs"] pub mod tcp_servers;
#[path = "std/udp_servers.rs"] pub mod udp_servers;
#[path = "std/dns.rs"] pub mod dns;
#[path = "std/ProxyServer.rs"] pub mod ProxyServer;
#[path = "std/web_sockets.rs"] pub mod web_sockets;
#[path = "std/listen_port.rs"] pub mod listen_port;
#[path = "std/client_server_examples.rs"] pub mod client_server_examples;



pub fn main()
{
    // client::test_all();
    // dns::test_all();
    // listen_port::test_all();
    // tcp_servers::test_all();
    // udp_servers::test_all();
    // web_sockets::test_all();
    // ProxyServer::test_all();

    // client_server_examples::test_all();

    // tokio_asynch_tcp_client::test_all();
    // tokio_asynch_tcp_server::test_all();

    mio_servers::test_all();
}