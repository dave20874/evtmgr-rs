mod event_mgr;


use std::{fmt::Display, sync::Mutex};

use event_mgr::{EventMgr, EventChannel};

#[derive(Debug)]
struct EventData {
    data1: u32,
}

static mut ch1: Mutex<EventChannel<EventData>> = Mutex::new(EventChannel::<EventData>::new());

fn main() {
    // Create an event manager
    let mgr = EventMgr::new();

    // Register a listener
    ch1.subscribe(|| {println!("In the callback.");});

    // Publish a message
    let e = EventData {data1: 69};
    ch1.publish(e);

    mgr.poll();

    println!("Did it work?");
}
