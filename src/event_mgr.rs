// A Event Management System for Rust

// TODO : Document how new event types are defined.
// TODO : Document how event sources register event channels.
// TODO : Document how event handlers register callbacks for events

use std::{fmt::Debug, marker::PhantomData, sync::Arc};


trait EventChannelIf {
// TODO
}

// TODO-DW : Eliminate 'static lifetime on channels managed by EventMgr.

// EventMgr
// The singleton manager of events manages the event queue.
pub struct EventMgr
{
    events_processed: usize,
    named_channels: Vec<&'static dyn EventChannelIf>,
}

impl EventMgr
{
    // private function to create the singleton event manager in lazy_static, above.
    pub fn new() -> EventMgr {
        EventMgr {
            events_processed: 0,
            named_channels: Vec::new(),
        }
    }

    fn register_channel(&mut self, channel: &'static dyn EventChannelIf)
    {
        self.named_channels.push(channel);
    }
}



// EventChannel
// An EventChannel provides operations on specific types of Events.
// Its static members represent the class of events, E.
pub struct EventChannel<'a, T>
where T: Debug
{
    _channel_id: usize,
    handler: &'a dyn Fn(T),
    phantom: PhantomData<T>,
}

impl<'a, T> EventChannel<'a, T>
    where T: Debug
{
    static mut pub fn new() -> EventChannel<'a, T>
    {
        EventChannel {_channel_id: 0, handler: &(|data| {println!("Ignoring data: {:?}", data)}), phantom: PhantomData }
    }

    pub fn subscribe(&mut self, handler: &'a dyn Fn(T)) {
        self.handler = handler;
        println!("Subscribe set handler.");
    }

    pub fn publish(&self, _e: T) {
        println!("TODO: Publish an event.");
    }
}
