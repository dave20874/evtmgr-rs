// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events

use std::{collections::VecDeque, fmt::Debug, sync::{Arc, Mutex}};

// TODO-DW : Eliminate 'static lifetime on channels managed by EventMgr.

// EventMgr
// The singleton manager of events manages the event queue.
pub struct EventMgr<'a>
{
    event_queue: Arc<Mutex<VecDeque<&'a dyn EventChannelIf>>>,
}

pub trait EventChannelIf{
    fn service_event(&self);
}

impl<'a> EventMgr<'a>
{
    // private function to create the singleton event manager in lazy_static, above.
    pub fn new() -> EventMgr<'a> {
        EventMgr {
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn queue(&'a self, callback: &'a dyn EventChannelIf)
    {
        let mut event_queue = self.event_queue.lock().unwrap();

        event_queue.push_back(callback);
    }

    pub fn poll(&self)
    {
        let mut event_queue = self.event_queue.lock().unwrap();

        while !event_queue.is_empty() {
            let channel = event_queue.pop_front().unwrap();
            channel.service_event();
        }
        println!("Polling done");
    }
}



// EventChannel
// An EventChannel provides operations on specific types of Events.
// Its static members represent the class of events, E.

// TODO: Figure out how to declare F as &dyn with a lifetime.

pub struct EventChannel<'a, T, F>
where T: Debug, F: Fn(&T)
{
    mgr: &'a EventMgr<'a>,
    handler: Option<F>,
    event_queue: Arc<Mutex<VecDeque<T>>>,
}

impl<'a, T, F> EventChannel<'a, T, F>
    where T: Debug, F: Fn(&T)
{
    pub fn new(mgr: &'a EventMgr<'a>) -> EventChannel<'a, T, F>
    {
        let ec = EventChannel {mgr: mgr, handler: None, event_queue: Arc::new(Mutex::new(VecDeque::new()))};
        ec
    }

    pub fn subscribe(&mut self, handler: F) 
    {
        self.handler = Some(handler);
        println!("Subscribe set handler.");
    }

    pub fn publish(&'a self, e: T) {
        println!("TODO: Publish an event.");

        let mut event_queue = self.event_queue.lock().unwrap();

        // Add event to the queue for this channel
        event_queue.push_back(e);

        // tell manager to call us back.
        self.mgr.queue(self);
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
