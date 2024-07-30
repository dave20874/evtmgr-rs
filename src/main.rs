mod event_mgr;


use event_mgr::{EventMgr, EventChannel};

struct EventData {
    data1: u32,
}

fn main() {
    // Create an event manager
    let mgr = EventMgr::new();

    // Get a reference to a channel
    // (Creates it and registers with event manager if the first time.)
    let for_pub = EventChannel::<EventData>::get(&mgr, "ch1");

    // Get a second reference to the channel
    let for_sub = EventChannel::<EventData>::get(&mgr, "ch1");

    // Register a listener
    for_sub.subscribe();

    // Publish a message
    let e = EventData {data1: 69};
    for_pub.publish(e);

    println!("Did it work?");
}
