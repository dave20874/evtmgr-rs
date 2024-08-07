mod event_mgr;

use std::sync::{Arc, Mutex};

use event_mgr::{EventMgr, EventChannel};

#[derive(Debug, Clone, Copy)]
struct EventData {
    data1: u32,
}

struct System<'a> {
    mgr: Arc<Mutex<EventMgr<'a>>>,
    ch1: EventChannel<'a, EventData, &'a dyn Fn(&EventData)>,
}

// TODO-DW : Add threads: one to service the even queue and a couple to generate events.
// TODO-DW : Create an event channel to terminate the program.

impl<'a> System<'a>
{
    pub fn new() -> Self
    {
        let mgr = Arc::new(Mutex::new(EventMgr::new()));
        let mut ch1 = 
            EventChannel::<EventData, &dyn Fn(&EventData)>::new(mgr.clone());

        // Register a listener
        ch1.subscribe(&|d| { println!("In the callback: {}", d.data1); } );

        System {
            mgr,
            ch1,
        }
    }

    pub fn send(&'a self) 
    {
        // Publish a message
        // let e = Box::new(EventData {data1: 69});
        self.ch1.publish(EventData {data1: 69});
    }

    pub fn poll(&self)
    {
        println!("Polling.");
        let mgr = self.mgr.lock().unwrap();
        mgr.poll();
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
