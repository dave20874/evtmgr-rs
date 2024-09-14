use std::{thread::sleep, time::Duration};

use event_mgr::{EventChannel, EventHandler};

// --- u32 EventData and Listener ---------------------------------------
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
        // println!("MyListener, {}, received {}", self.name, data.data1);
    }
}

// --- Quaternion EventData and Listener ---------------------------------------
struct QData {
    w: f64,
    x: f64,
    y: f64,
    z: f64
}

// Define my listener
struct QListener {
    name: String,
}

impl QListener {
    fn new(name: &str) -> QListener {
        QListener { name: name.to_string() }
    }
}

// Define my Event Handler
impl EventHandler<QData> for QListener {
    fn handle(&self, data: &QData) {
        // println!("QListener, {}, received w:{}, x:{}, y:{}, z:{}", self.name, data.w, data.x, data.y, data.z);
    }
}

// --- Main function ------------------------------------------------------------
// Creates the channels, listeners, subscriptions and posts events.
// The handlers handle the events in the event handler thread.

fn main() {
    // create some  channels
    let chan = EventChannel::<EventData>::new();
    let chan2 = EventChannel::<QData>::new();

    // register some listeners for the channel
    let listener1 = Box::new(MyListener::new("L1"));
    let listener2 = Box::new(MyListener::new("L2"));

    chan.subscribe(listener1);
    chan.subscribe(listener2);
    chan2.subscribe(Box::new(QListener::new("L3")));

    // Start event manager thread
    let t = event_mgr::run_thread();

    // post some events
    println!("Posting events.");

    let pause = Duration::new(0, 10000);
    for n in 0..1000000 {
        chan.post(EventData { data1: n });
        chan2.post(QData{ w:1.0, x:0.0, y:0.0, z:0.0 });
        // sleep(pause)
    }

    println!("Joining with mgr thread.");
    t.join().unwrap();
    println!("All done.");

}
