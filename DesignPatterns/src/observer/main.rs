
mod Observer;
mod Observer_Func;
mod Tokio_Broadcast_Observer;
mod Stock_Price_Observer;

pub fn test_all()
{
    // Observer::temperature_sensor::demo();
    Observer_Func::observer_func::demo();
    // Tokio_Broadcast_Observer::demo();
    // Stock_Price_Observer::demo();
}