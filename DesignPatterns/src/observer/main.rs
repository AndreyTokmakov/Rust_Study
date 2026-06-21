
mod Observer;
mod Tokio_Broadcast_Observer;
mod Stock_Price_Observer;

pub fn test_all()
{
    // Observer::temperature_sensor::demo();
    // Tokio_Broadcast_Observer::demo();
    Stock_Price_Observer::demo();
}