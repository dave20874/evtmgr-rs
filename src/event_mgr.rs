// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events

use std::{collections::VecDeque, fmt::Debug, sync::Mutex};
use lazy_static::lazy_static;

// TODO-DW : Eliminate 'static lifetime on channels managed by EventMgr.

// EventMgr
// The singleton manager of events manages the event queue.
pub struct EventMgr
{
    event_queue: Mutex<VecDeque<&'static dyn EventChannelIf>>,
}

pub trait EventChannelIf: Sync+Send {
    fn service_event(&self);
}

pub trait EventHandler<T>: Sync+Send {
    fn on_event(&self, event: &T);
}

impl EventMgr
{
    // private function to create the singleton event manager in lazy_static, above.
    fn new() -> EventMgr {
        EventMgr {
            event_queue: Mutex::new(VecDeque::new()),
        }
    }

    fn queue(&self, callback: &'static dyn EventChannelIf)
    {
        let mut event_queue = self.event_queue.lock().unwrap();
        event_queue.push_back(callback);
    }

    pub fn poll()
    {
        let mut poll_again = true;
        while poll_again {
            let chan_opt: Option<&dyn EventChannelIf> = {
                let mut event_queue = EVENT_MGR.event_queue.lock().unwrap();
                if event_queue.is_empty() {
                    None
                }
                else {
                    let channel = event_queue.pop_front().unwrap();
                    Some(channel)
                }
            };
            match chan_opt {
                Some(channel) => {
                    channel.service_event();
                }
                None => {
                    poll_again = false;
                }
            }
        }
        println!("Polling done");
    }
}

lazy_static! {
    static ref EVENT_MGR: EventMgr = EventMgr::new();
}



// EventChannel
// An EventChannel provides operations on specific types of Events.
// Its static members represent the class of events, E.

// TODO: Figure out how to declare F as &dyn with a lifetime.

pub struct EventChannel<T>
where T: Debug+Sync+Send+'static
{
    handlers: Mutex<Vec<&'static dyn EventHandler<T>>>,
    event_queue: Mutex<VecDeque<T>>,
}

impl<T> EventChannel<T>
where T: Debug+Sync+Send+'static
{
    pub fn new() -> EventChannel<T>
    {
        EventChannel { handlers: Mutex::new(Vec::new()), event_queue: Mutex::new(VecDeque::new()) }
    }

    pub fn subscribe(&self, handler: &'static dyn EventHandler<T>) 
    {
        let mut handler_list = self.handlers.lock().unwrap();
        handler_list.push(handler);
        println!("Subscribe added handler.");
    }

    pub fn publish(&self, e: T) {
        println!("TODO: Publish an event.");

        let mut event_queue = self.event_queue.lock().unwrap();

        // Add event to the queue for this channel
        event_queue.push_back(e);

        // tell manager to call us back.
        EVENT_MGR.queue(self); // Do Callbacks need to be Arc?
    }

}

impl<T> EventChannelIf for EventChannel<T>
where T: Debug+Sync+Send
{
    fn service_event(&self) 
    {
        let mut event_queue = self.event_queue.lock().unwrap();
        let data = event_queue.pop_front().unwrap();

        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.on_event(&data);
        }
    }
}
