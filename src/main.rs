mod event_mgr;

use event_mgr::{EventChannel, Event};

struct Event1 {
    data1: u32,
}

impl Event for Event1 {
    // TODO
    fn foo(&self) {
        // ok
    }
}

struct Event2 {
    data2: u32,
}

impl Event for Event2 {
    // TODO
    fn foo(&self) {
        // ok
    }
}



fn main() {
    let ch1 = EventChannel::<Event1>::open();

    let e = Event1 {data1: 69};

    ch1.raise(e);

    println!("I think I raised an event.");
}
