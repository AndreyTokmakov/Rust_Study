
mod clients;
mod tcp_servers;
mod udp_servers;
mod ProxyServer;
mod web_sockets;
mod listen_port;



pub fn test_all()
{
    // clients::test_all();
    // listen_port::test_all();
    // tcp_servers::test_all();
    // udp_servers::test_all();
    web_sockets::test_all();
    // ProxyServer::test_all();
}