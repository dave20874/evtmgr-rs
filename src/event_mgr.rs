// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events


// TODO-DW : Test generating an event from within an event handler.
// TODO-DW : Now that I've kind of figured out inner mutability with Mutex, review
//           all the &'a references I created and eliminate the unnecessary ones.
// TODO-DW : Where do I really need Arc<Mutex<>> vs just Mutex<>
//           What is using reference counting?  The mgr itself?
// TODO-DW : If mgr is Arc<Mutex<>> does it's event_queue need to be, too?


use std::{collections::VecDeque, fmt::Debug, sync::{Arc, Mutex}};

// EventMgr
// The singleton manager of events manages the event queue.
pub struct EventMgr<'a>
{
    // TODO-DW : Could this be lighter weight?
    // (No need for Arc here, just Mutex?)
    // Could another data structure remove requirement for Mutex?

    event_queue: VecDeque<&'a dyn EventChannelIf>,
}

pub trait EventChannelIf{
    fn service_event(&self);
}

impl<'a> EventMgr<'a>
{
    // private function to create the singleton event manager in lazy_static, above.
    pub fn new() -> EventMgr<'a> {
        EventMgr {
            event_queue: VecDeque::new(),
        }
    }

    pub fn queue(&mut self, callback: &'a dyn EventChannelIf)
    {
        // let mut event_queue = self.event_queue.lock().unwrap();

        self.event_queue.push_back(callback);
    }

    pub fn poll(&mut self)
    {
        // let mut event_queue = self.event_queue.lock().unwrap();

        while !self.event_queue.is_empty() {
            let channel = self.event_queue.pop_front().unwrap();
            channel.service_event();
        }
        println!("Polling done");
    }
}



// EventChannel
// An EventChannel provides operations on specific types of Events.


pub struct EventChannel<'a, T, F>
where T: Debug, F: Fn(&T)
{
    mgr: Arc<Mutex<EventMgr<'a>>>,
    handler: Option<F>,                          // TODO-DW : Mutliple handlers
    event_queue: Arc<Mutex<VecDeque<T>>>,
}

impl<'a, T, F> EventChannel<'a, T, F>
    where T: Debug, F: Fn(&T)
{
    pub fn new(mgr: Arc<Mutex<EventMgr<'a>>>) -> EventChannel<'a, T, F>
    {
        EventChannel {
            mgr, 
            handler: None, 
            event_queue: Arc::new(Mutex::new(VecDeque::new()))
        }
    }

    pub fn subscribe(&mut self, handler: F) 
    {
        self.handler = Some(handler);
        println!("Subscribe set handler.");
    }

    pub fn publish(&'a self, e: T) {
        let mut event_queue = self.event_queue.lock().unwrap();

        // TODO-DW : Consider order of Mutex acquisition (there are two.)  Are we avoiding deadlock?

        // Add event to the queue for this channel
        event_queue.push_back(e);

        // tell manager to call us back.
        let mut mgr = self.mgr.lock().unwrap();
        mgr.queue(self);
    }

}

impl<'a, T, F> EventChannelIf for EventChannel<'a, T, F>
where T: Debug, F: Fn(&T)
{
    fn service_event(&self) 
    {
        let mut event_queue = self.event_queue.lock().unwrap();
        let data = event_queue.pop_front().unwrap();
        match &self.handler {
            Some(f) => { f(&data); }
            None => { println!("No handler."); }
        }
    }
}
