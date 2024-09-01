mod event_mgr;


use event_mgr::{EventChannel, EventHandler, EventMgr, EventRecord};

struct EventData {
    data1: u32,
}

struct MyListener {
    name: String,
}

impl MyListener {
    fn new(name: &str) -> MyListener {
        let l = MyListener { name: name.to_string() };

        l
    }
}

impl EventHandler<EventData> for MyListener {
    fn handle(&self, data: &EventData) {
        println!("MyListener, {}, received {}", self.name, data.data1);
    }
}

fn main() {
    // Create an event manager
    let mgr = EventMgr::new();

    // create a channel
    let chan = EventChannel::<EventData>::new();

    // register some listeners for the channel
    let listener1 = Box::new(MyListener::new("L1"));
    let listener2 = Box::new(MyListener::new("L2"));
    {
        let chan = chan.lock().unwrap();
        chan.subscribe(listener1);
        chan.subscribe(listener2);
    }

    // post some events
    mgr.post(EventRecord::<EventData>::new(EventData { data1: 1 }, chan.clone()));
    mgr.post(EventRecord::<EventData>::new(EventData { data1: 2 }, chan.clone()));

    // let event manager work
    mgr.poll();

    println!("Did it work?");
}
