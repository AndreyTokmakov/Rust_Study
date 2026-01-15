#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod client;
mod tcp_servers;
mod udp_servers;
mod dns;
mod ProxyServer;
mod web_sockets;
mod listen_port;
mod tokio_asynch_tcp_client;
mod tokio_asynch_tcp_server;
mod client_server_examples;

pub fn main()
{
    // client::test_all();
    // dns::test_all();
    // listen_port::test_all();
    // tcp_servers::test_all();
    // udp_servers::test_all();
    // web_sockets::test_all();
    // ProxyServer::test_all();

    client_server_examples::test_all();

    // tokio_asynch_tcp_client::test_all();
    // tokio_asynch_tcp_server::test_all();
}