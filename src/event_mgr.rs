// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events

use std::{collections::VecDeque, fmt::Debug, sync::{Arc, Mutex}};

// TODO-DW : Eliminate 'static lifetime on channels managed by EventMgr.

// EventMgr
// The singleton manager of events manages the event queue.
pub struct EventMgr
{
    event_queue: VecDeque<&'static dyn EventChannelIf>,
}



pub trait EventChannelIf: Sync {
    fn service_event(&self);
}

pub trait EventHandler<T> {
    fn on_event(&self, event: &T);
}

impl EventMgr
{
    // private function to create the singleton event manager in lazy_static, above.
    fn new() -> EventMgr {
        EventMgr {
            event_queue: VecDeque::new(),
        }
    }

    fn queue(&self, callback: &dyn EventChannelIf)
    {
        self.event_queue.push_back(callback);
    }

    pub fn poll()
    {
        let mut poll_again = true;
        while poll_again {
            let chan_opt: Option<&dyn EventChannelIf> = {
                let mut mgr = EVENT_MGR.lock().unwrap();
                if mgr.event_queue.is_empty() {
                    None
                }
                else {
                    let channel = mgr.event_queue.pop_front().unwrap();
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

static EVENT_MGR: Mutex<EventMgr> = Mutex::new(EventMgr::new());



// EventChannel
// An EventChannel provides operations on specific types of Events.
// Its static members represent the class of events, E.

// TODO: Figure out how to declare F as &dyn with a lifetime.

pub struct EventChannel<'a, T>
where T: Debug
{
    handlers: Vec<Mutex<&'a dyn EventHandler<T>>>,
    event_queue: Arc<Mutex<VecDeque<T>>>,
}

impl<'a, T> EventChannel<'a, T>
where T: Debug
{
    pub fn new() -> EventChannel<'a, T>
    {
        EventChannel { handlers: Vec::new(), event_queue: Arc::new(Mutex::new(VecDeque::new())) }
    }

    pub fn subscribe(&mut self, handler: Mutex<&'a dyn EventHandler<T>>) 
    {
        self.handlers.push(handler);
        println!("Subscribe added handler.");
    }

    pub fn publish(&self, e: T) {
        println!("TODO: Publish an event.");

        let mut event_queue = self.event_queue.lock().unwrap();

        // Add event to the queue for this channel
        event_queue.push_back(e);

        // tell manager to call us back.
        let mgr = EVENT_MGR.unlock.unwrap();
        mgr.unlock().queue(self);
    }

}

impl<'a, T> EventChannelIf for EventChannel<'a, T>
where T: Debug
{
    fn service_event(&self) 
    {
        let mut event_queue = self.event_queue.lock().unwrap();
        let data = event_queue.pop_front().unwrap();
        match &self.handler {
            Some(handler) => { handler.on_event(&data); }
            None => { println!("No handler."); }
        }
    }
}
