// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events

use std::{marker::PhantomData, sync::Arc};

// EventMgr
// The singleton manager of events manages the event queue.
pub struct EventMgr 
{
    events_processed: usize,
}

impl EventMgr
{
    // private function to create the singleton event manager in lazy_static, above.
    pub fn new() -> EventMgr {
        EventMgr {
            events_processed: 0 
        }
    }
}

// EventChannel
// An EventChannel provides operations on specific types of Events.
// Its static members represent the class of events, E.
pub struct EventChannel<T>
{
    _channel_id: String,
    phantom: PhantomData<T>,
}


impl<T> EventChannel<T>
{
    pub fn get(_mgr: &EventMgr, id: &str) -> Arc<EventChannel<T>> {
        // Check whether this channel id is already registered with the event manager
        // if not, create the channel and register it
        // return the channel reference reference.

        // TODO : Handle registration with event manager
        println!("TODO: Implement channel registration with manager.");
        // For now, just create one and return the reference so we can compile
        Arc::new(EventChannel::<T> {_channel_id: id.to_string(), phantom: PhantomData } )
    }

    pub fn subscribe(&self) {
        println!("TODO: Implement subscribe.");
    }

    pub fn publish(&self, _e: T) {
        println!("TODO: Publish an event.");
    }
}
