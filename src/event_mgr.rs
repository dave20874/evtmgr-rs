// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events

use std::{collections::VecDeque, marker::PhantomData, sync::{Arc, Mutex}};

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
    channel: Arc<Mutex<EventChannel<T>>>,
}

impl<T> EventRecord<T>
{
    pub fn new(data: Box<T>, channel: &Arc<Mutex<EventChannel<T>>>) -> Box<EventRecord<T>>
    {
        Box::new(EventRecord {data: data, channel: channel.clone()} )
    }
}

impl<T> EventDispatchIf for EventRecord<T>
where T : Send
{
    fn dispatch(&self)
    {
        let channel = self.channel.lock().unwrap();
        channel.dispatch(&self.data);
    }
}

// EventChannel
// An EventChannel provides the registration mechanism for listeners to receive events
pub struct EventChannel<T>
{
    handlers: Mutex<Vec<Box<dyn EventHandler<T>>>>,
    phantom: PhantomData<T>,
}


impl<T> EventChannel<T>
where T: Send
{
    pub fn create() -> Arc<Mutex<EventChannel<T>>> {
        // For now, just create one and return the reference so we can compile
        Arc::new(Mutex::new(EventChannel::<T> {handlers: Mutex::new(Vec::new()), phantom: PhantomData } ))
    }

    pub fn subscribe(&self, l: Box<dyn EventHandler<T>>) {
        let mut handlers = self.handlers.lock().unwrap();

        handlers.push(l);
    }

    fn dispatch(&self, e: &T) {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.handle(e);
        }
    }
}

pub struct WrappedEventChannel<'a, T>
{
    am_ec: Arc<Mutex<EventChannel<T>>>,
    mgr: &'a EventMgr,
}

impl<'a, T> WrappedEventChannel<'a, T>
where T: Send+'static
{
    pub fn new(mgr: &'a EventMgr) -> WrappedEventChannel<'a, T>
    {
        WrappedEventChannel {
            am_ec: EventChannel::<T>::create(),
            mgr: mgr,
        }
    }

    pub fn post(&'a self, event_data: T) {
        self.mgr.post(
            EventRecord::new(Box::new(event_data), &self.am_ec.clone())
        );
    }

    pub fn subscribe(&'a self, l: Box<dyn EventHandler<T>>) {
        let chan = self.am_ec.lock().unwrap();

        chan.subscribe(l);
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

