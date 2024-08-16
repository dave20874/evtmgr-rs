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
    ch2: EventChannel<'a, String, &'a dyn Fn(&String)>,
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

        let mut ch2: EventChannel<String, &dyn Fn(&String)> =
        EventChannel::<String, &dyn Fn(&String)>::new(mgr.clone());

        // Register a listener
        ch1.subscribe(&|d| { println!("In the callback: {}", d.data1); } );
        ch2.subscribe(&|s: &String| { println!("Chan 2: {}", s)});

        System {
            mgr,
            ch1,
            ch2,
        }
    }

    pub fn run(&'a self)
    {
        for _n in 0..5 {
            self.ch1.publish(EventData {data1: 99});
        }
        self.ch2.publish("Hello event manager.".to_string());

        let mut mgr = self.mgr.lock().unwrap();
        mgr.shutdown();
    
        while mgr.is_running() {
            println!("Polling.");
            mgr.poll();
            println!("Polling done.");
        }
    }
}

fn main() 
{
    let sys = System::new();

    sys.run();

    println!("Did it work?");
}
