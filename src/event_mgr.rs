// A Event Management System for Rust

use std::{collections::VecDeque, sync::{Arc, Mutex}, thread::{self, sleep, JoinHandle}, time::{Duration, SystemTime}};
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
        {
            let mut mgr = EVENT_MGR.lock().unwrap();
            mgr.post(
                EventRecord::new(Box::new(event_data), self)
            );
        }
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
    event_queue: VecDeque<Box<dyn EventDispatchIf>>,
    events: usize,
    shutting_down: bool,
}

impl EventMgr
{
    // private function to create the singleton event manager in lazy_static, above.
    pub fn new() -> EventMgr {
        EventMgr {
            event_queue: VecDeque::new(),
            events: 0,
            shutting_down: false,
        }
    }

    pub fn post(&mut self, event_record: Box<dyn EventDispatchIf>) {
        if !self.shutting_down {
            self.event_queue.push_back(event_record);
        }
    }

    fn poll(&mut self) {
        let mut run = true;
        while run {
            let handler_opt = {
                self.event_queue.pop_front()
            };

            match handler_opt {
                Some(h) => {
                    // println!("Dispatching.");
                    self.events += 1;
                    h.dispatch();
                }
                None => {
                    run = false;
                }
            }
        }
    }

}

pub fn poll_loop() {
    println!("Started thread.");
    let mut now = SystemTime::now();
    let end_time = now + Duration::new(1, 0);
    let sleep_time = Duration::new(0, 1_000);  // 1us
    let mut count = 0;
    while now < end_time {
        {
            let mut mgr = EVENT_MGR.lock().unwrap();
            mgr.poll();
        }
        count += 1;
        sleep(sleep_time);
        now = SystemTime::now();
    }
    {
        let mgr = EVENT_MGR.lock().unwrap();
        println!("Thread ending after {} sleeps, {} events", count, mgr.events);
    }
}

pub fn run_thread() -> JoinHandle<()> {
    thread::spawn(
        move || {
            poll_loop();
        }
    )
}

lazy_static! {
    static ref EVENT_MGR: Mutex<EventMgr> = Mutex::new(EventMgr::new());
}
