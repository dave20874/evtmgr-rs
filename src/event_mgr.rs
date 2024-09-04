// A Event Management System for Rust

use std::{collections::VecDeque, sync::{Arc, Mutex}};
use lazy_static::lazy_static;

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
    channel: Arc<EventChannel<T>>,
}

impl<T> EventRecord<T>
{
    fn new(data: Box<T>, channel: &Arc<EventChannel<T>>) -> Box<EventRecord<T>>
    {
        Box::new(EventRecord {data, channel: channel.clone()} )
    }
}

impl<T> EventDispatchIf for EventRecord<T>
where T : Send+'static
{
    fn dispatch(&self)
    {
        self.channel.dispatch(&self.data);
    }
}

// EventChannel
// An EventChannel provides the registration mechanism for listeners to receive events
pub struct EventChannel<T>
{
    handlers: Mutex<Vec<Box<dyn EventHandler<T>>>>,
}

impl<T> EventChannel<T>
where T: Send+'static
{
    pub fn new() -> Arc<EventChannel<T>>
    {
        Arc::new(
            EventChannel { handlers: Mutex::new(Vec::new()) }
        )
    }

    pub fn post(self: &Arc<Self>, event_data: T) {
        EVENT_MGR.post(
            EventRecord::new(Box::new(event_data), self));
    }

    pub fn subscribe(self: &Arc<Self>, l: Box<dyn EventHandler<T>>) {
        let mut handlers = self.handlers.lock().unwrap();

        handlers.push(l);
    }

    fn dispatch(self: &Arc<Self>, event_data: &T) {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.handle(event_data);
        }
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

lazy_static! {
    pub static ref EVENT_MGR: EventMgr = EventMgr::new();
}
