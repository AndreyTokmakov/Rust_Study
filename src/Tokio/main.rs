
mod udp_tests;
mod tcp_servers;
mod experiments;
mod web_sockets_tungstenite;
mod web_socket_binance;
mod oneshot_channel;
mod broadcast_channel;


pub fn test_all()
{
    // udp_tests::test_all();
    // tcp_servers::test_all();
    // experiments::test_all();oneshot channel
    // web_socket_binance::test_all();
    // oneshot_channel::test_all();
    broadcast_channel::test_all();
}