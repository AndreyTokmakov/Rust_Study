
mod udp_tests;
mod tcp_servers;
mod experiments;
mod web_sockets_tungstenite;
mod web_socket_binance;


pub fn test_all()
{
    // udp_tests::test_all();
    // tcp_servers::test_all();
    // experiments::test_all();
    
    // web_sockets_tungstenite::test_all();
    web_socket_binance::test_all();
}