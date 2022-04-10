use arwa::console;
use arwa::worker::dedicated::DedicatedWorker;

fn main() {
    for i in 0..10u32 {
        DedicatedWorker::spawn(move |cx| {
            console::log!("Hello from worker %i!", i);
        });
    }
}
