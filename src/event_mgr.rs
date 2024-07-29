// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events

use lazy_static::lazy_static;
use std::{marker::PhantomData, sync::Mutex};


// Event
// represents an instance of an event at the most abstract level
pub trait Event {
    // TODO
    fn foo(&self) {
        // ok
    }
}

// EventMgr
// The singleton manager of events manages the event queue.
pub struct EventMgr 
{
    next_channel_id: usize,
    events_processed: usize,
}

lazy_static! {
    static ref EVENT_MGR: Mutex<EventMgr> = Mutex::new(EventMgr::new());
}

impl EventMgr
{
    // private function to create the singleton event manager in lazy_static, above.
    fn new() -> EventMgr {
        EventMgr {
            next_channel_id: 0, 
            events_processed: 0 
        }
    }

    // Get a reference to the event manager
    pub fn get() -> &'static Mutex<EventMgr>
    {
        EVENT_MGR
    }

    fn get_channel_id(&self) -> usize
    {
        // TODO : Use Mutex
        let retval = self.next_channel_id;
        self.next_channel_id += 1;

        retval
    }
}

// EventChannel
// An EventChannel provides operations on specific types of Events.
// Its static members represent the class of events, E.
pub struct EventChannel<E: Event>
{
    occurrences: usize,
    channel_id: usize,
    phantom: PhantomData<E>,
}


impl<E> EventChannel<E> where E: Event {


    lazy_static! {
        static ref EVENT_CHANNEL: EventChannel<E> = EventChannel<E>::new(EVENT_MGR.get_channel_id());
    }

    fn new(id: usize) -> EventChannel<E> {
        channel_id = id;
        EventChannel { occurrences: 0, channel_id: id, phantom: PhantomData}
    }

    pub fn open() -> &EventChannel<E> {
        match channel_id {
            Some(id) => {
                &EVENT_CHANNEL
            }
            None => {
                // We need to create a channel
                let event_mgr = EventMgr::get();
                channel_id = event_mgr.get_channel_id();
                EVENT_CHANNEL.set_id(channel_id);
                &EVENT_CHANNEL
            }
        }

    }

    fn raise(e: &dyn Event) {
        println!("Raising an event.");
    }
}

