
mod clients;
mod tcp_servers;
mod udp_servers;
mod ProxyServer;
mod web_sockets;



pub fn test_all()
{
    clients::test_all();
    // tcp_servers::test_all();
    // udp_servers::test_all();
    // web_sockets::test_all();
    // ProxyServer::test_all();
}