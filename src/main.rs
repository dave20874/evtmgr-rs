mod event_mgr;

use std::sync::Arc;

use event_mgr::{EventMgr, EventChannel};

#[derive(Debug)]
struct EventData {
    data1: u32,
}

struct System<'a> {
    mgr: Arc<EventMgr<'a>>,
    ch1: EventChannel<'a, EventData, &'a dyn Fn(&EventData)>,
}

impl<'a> System<'a>
{
    pub fn new() -> Self
    {
        let mgr = Arc::new(EventMgr::new());
        let mut ch1 = EventChannel::<EventData, &dyn Fn(&EventData)>::new(&mgr);

        let mut sys = System {
            mgr: mgr.clone(),
            ch1: ch1,
        };

        // Register a listener
        sys.ch1.subscribe(&|d| { println!("In the callback: {}", d.data1); } );

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
        self.mgr.poll();
        println!("Polling done.");
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
