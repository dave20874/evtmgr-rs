mod event_mgr;

use std::{thread::sleep, time::Duration};

use event_mgr::{EventChannel, EventHandler};

// Define my event type.
struct EventData {
    data1: u32,
}

// Define my listener
struct MyListener {
    name: String,
}

impl MyListener {
    fn new(name: &str) -> MyListener {
        MyListener { name: name.to_string() }
    }
}

// Define my Event Handler
impl EventHandler<EventData> for MyListener {
    fn handle(&self, data: &EventData) {
        println!("MyListener, {}, received {}", self.name, data.data1);
    }
}

fn main() {
    // create some  channels
    let chan = EventChannel::<EventData>::new();
    let chan2 = EventChannel::<EventData>::new();

    // register some listeners for the channel
    let listener1 = Box::new(MyListener::new("L1"));
    let listener2 = Box::new(MyListener::new("L2"));

    chan.subscribe(listener1);
    chan.subscribe(listener2);
    chan2.subscribe(Box::new(MyListener::new("L3")));

    // Start event manager thread
    let t = event_mgr::run_thread();

    // post some events
    println!("Posting events.");

    let pause = Duration::new(0, 10000);
    for n in 0..3 {
        chan.post(EventData { data1: n });
        chan2.post(EventData{ data1: n });
        sleep(pause)
    }

    println!("Joining with mgr thread.");
    t.join().unwrap();
    println!("All done.");

}
