mod event_mgr;

use std::sync::{Arc, Mutex};

use event_mgr::{EventChannel, EventMgr};

#[derive(Debug, Clone, Copy)]
struct EventData {
    data1: u32,
}

// TODO: Add a counter to system of events processed.  (This caused it to get mutable and problems cascaded.)
struct System<'a> {
    mgr: Arc<Mutex<EventMgr<'a>>>,
    ch1: EventChannel<'a, EventData, &'a dyn Fn(&EventData)>,
}

// TODO-DW : Add threads: one to service the even queue and a couple to generate events.
// TODO-DW : Create an event channel to terminate the program.

impl<'a> System<'a>
{
    // TODO-DW : Solve this problem where mgr is referenced by channel and by system.
    // Do I have to bring back Arc<>?
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
        self.ch1.publish(EventData {data1: 99});
    }

    pub fn poll(&self)
    {
        println!("Polling.");
        let mut mgr = self.mgr.lock().unwrap();
        mgr.poll();
        println!("Polling done.");
    }
}

fn main() 
{
    let sys = System::new();

    for _n in 0..3 {
        sys.send();
    }

    sys.poll();

    println!("Did it work?");
}
