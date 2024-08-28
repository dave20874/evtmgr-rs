mod event_mgr;

use event_mgr::{EventMgr, EventChannel, EventHandler};
use lazy_static::lazy_static;

#[derive(Debug)]
struct EventData {
    data1: u32,
}

lazy_static! {
    static ref CH1: EventChannel<EventData> = EventChannel::new();
}

struct System {
    dummy: u32,
}

impl System
{
    pub fn new() -> Self
    {
        let mut ch1 = EventChannel::<EventData>::new();

        let mut sys = System {
            dummy: 0,
        };

        // Register a listener
        CH1.subscribe(&sys);

        sys
    }

    pub fn send(&self) 
    {
        // Publish a message
        // let e = Box::new(EventData {data1: 69});
        CH1.publish(EventData {data1: 69});
    }

    pub fn poll(&self)
    {
        println!("Polling.");
        EventMgr::poll();
        println!("Polling done.");
    }
}

impl EventHandler<EventData> for System
{
    fn on_event(&self, data: &EventData) 
    {
        println!("in System on_event()");
    }
}

fn main() 
{
    let sys = System::new();

    sys.send();
    sys.send();
    sys.send();
    sys.poll();

    println!("Did it work?");
}
