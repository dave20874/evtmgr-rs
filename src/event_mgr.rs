// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events

use std::{collections::VecDeque, sync::{Arc, Mutex}};

// A handler of events with event_data: T
pub trait EventHandler<T> : Send
where T : Send
{
    fn handle(&self, data: &T);
}

pub trait EventDispatchIf : Send
{
    fn dispatch(&self);
}

pub struct EventRecord<T>
{
    data: Box<T>,
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler<T>>>>>, // remove Arc<Mutex<EventChannelHandlers<T>>>,
}

impl<T> EventRecord<T>
{
    fn new(data: Box<T>, handlers: &Arc<Mutex<Vec<Box<dyn EventHandler<T>>>>>) -> Box<EventRecord<T>>
    {
        Box::new(EventRecord {data: data, handlers: handlers.clone()} )
    }
}

impl<T> EventDispatchIf for EventRecord<T>
where T : Send
{
    fn dispatch(&self)
    {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.handle(&self.data);
        }
    }
}

// EventChannel
// An EventChannel provides the registration mechanism for listeners to receive events
pub struct EventChannel<'a, T>
{
    handlers: Arc<Mutex<Vec<Box<dyn EventHandler<T>>>>>,
    mgr: &'a EventMgr,
}

impl<'a, T> EventChannel<'a, T>
where T: Send+'static
{
    pub fn new(mgr: &'a EventMgr) -> EventChannel<'a, T>
    {
        EventChannel {
            handlers: Arc::new(Mutex::new(Vec::new())),
            mgr: mgr,
        }
    }

    pub fn post(&'a self, event_data: T) {
        self.mgr.post(
            EventRecord::new(Box::new(event_data), &self.handlers)
        );
    }

    pub fn subscribe(&'a self, l: Box<dyn EventHandler<T>>) {
        let mut handlers = self.handlers.lock().unwrap();

        handlers.push(l);
    }
} 

// EventMgr
// The manager of events manages the event queue.
pub struct EventMgr
{
    event_queue: Mutex<VecDeque<Box<dyn EventDispatchIf>>>,
}

impl EventMgr
{
    // private function to create the singleton event manager in lazy_static, above.
    pub fn new() -> EventMgr {
        EventMgr {
            event_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn post(&self, event_record: Box<dyn EventDispatchIf>) {
        let mut queue = self.event_queue.lock().unwrap();

        queue.push_back(event_record);
    }

    pub fn poll(&self) {
        let mut run = true;
        while run {
            let handler_opt = {
                let mut queue = self.event_queue.lock().unwrap();
                queue.pop_front()
            };

            match handler_opt {
                Some(h) => {
                    println!("Dispatching.");
                    h.dispatch();
                }
                None => {
                    run = false;
                }
            }
        }
    }

}

