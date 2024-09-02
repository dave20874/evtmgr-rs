mod event_mgr;


use std::{sync::Arc, thread::{self, sleep}, time::{self, Duration, SystemTime}};

use event_mgr::{EventChannel, EventHandler, EventMgr, EventRecord};

struct EventData {
    data1: u32,
}

struct MyListener {
    name: String,
}

impl MyListener {
    fn new(name: &str) -> MyListener {
        MyListener { name: name.to_string() }
    }
}

impl EventHandler<EventData> for MyListener {
    fn handle(&self, data: &EventData) {
        println!("MyListener, {}, received {}", self.name, data.data1);
    }
}

fn main() {
    // Create an event manager
    let mgr = Arc::new(EventMgr::new());
    let thread_mgr = mgr.clone();

    let t = thread::spawn(move || {
        println!("Started thread.");
        let mut now = SystemTime::now();
        let end_time = now + Duration::new(2, 0);
        let sleep_time = Duration::new(0, 1000000);  // 1ms
        while now < end_time {
            thread_mgr.poll();
            sleep(sleep_time);
            now = SystemTime::now();
        }
        println!("Thread ending.");
    });

    // create a channel
    let chan = EventChannel::<EventData>::create();

    // register some listeners for the channel
    let listener1 = Box::new(MyListener::new("L1"));
    let listener2 = Box::new(MyListener::new("L2"));
    {
        let chan = chan.lock().unwrap();
        println!("Subscribing.");
        chan.subscribe(listener1);
        chan.subscribe(listener2);
    }

    // post some events
    println!("Posting events.");
    mgr.post(EventRecord::<EventData>::new(EventData { data1: 1 }, &chan));
    mgr.post(EventRecord::<EventData>::new(EventData { data1: 2 }, &chan));

    // let event manager work
    // println!("Polling.");
    // mgr.poll();

    // println!("Did it work?");
    println!("Joining with mgr thread.");
    t.join().unwrap();
    println!("All done.");

}
