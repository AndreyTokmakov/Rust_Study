
mod udp_tests;
mod web_sockets_tungstenite;
mod experiments;

pub fn test_all()
{
    // udp_tests::test_all();
    experiments::test_all();
    // web_sockets_tungstenite::test_all();
}