mod event_mgr;

use event_mgr::{EventMgr, EventChannel, EventHandler};

#[derive(Debug)]
struct EventData {
    data1: u32,
}

struct System<'a> {
    ch1: EventChannel<'a, EventData>,
    handler1: Mutex<&'a dyn &EventData>,  // Fixme.
}

impl<'a> System<'a>
{
    pub fn new() -> Self
    {
        let mut ch1 = EventChannel::<EventData, &dyn Fn(&EventData)>::new();

        let mut sys = System {
            ch1: ch1,
        };

        // Register a listener
        sys.ch1.subscribe(&self );

        sys
    }

    pub fn send(&self) 
    {
        // Publish a message
        // let e = Box::new(EventData {data1: 69});
        self.ch1.publish(&EventData {data1: 69});
    }

    pub fn poll(&self)
    {
        println!("Polling.");
        EventMgr::poll();
        println!("Polling done.");
    }
}

impl<'a> EventHandler<EventData> for System<'a> 
{
    fn on_event(&self, data: &T) 
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
