use arwa::console;
use arwa::window::window;

fn main() {
    console::log!("Simple message");

    // Message with a browser owned complex object
    console::log!(window());

    // Different log levels:
    console::info!("Informational message");
    console::debug!("Debug message - you may have to adjust the console's log level to see this");
    console::warn!("Warning message");
    console::error!("Error message");

    // Multi-part message with mixed types
    console::log!("Multi", "part", "message", 1u8, true);

    // String formatting
    console::log!("Value %i referenced in string", 1u8);

    // String formatting with a complex object
    console::log!("The %o referenced in a string", window());

    // A successful assertion
    console::assert!(1 == 1, "Won't be written to the console");

    // A failing assertion
    let n0 = 1u8;
    let n1 = 2u8;
    console::assert!(n0 == n1, "Unfortunately %i does not equal %i", n0, n1);

    // Let's start a timer!
    console::time("my-timer");

    // Log the timer time
    console::time_log!("my-timer");

    // Log the timer time with a message
    console::time_log!("my-timer", "Logging the time with a message");

    console::time_end("my-timer");
}
