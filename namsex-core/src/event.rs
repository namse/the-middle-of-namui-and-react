use once_cell::sync::OnceCell;
use std::any::Any;
use tokio::sync::mpsc::{self, unbounded_channel};
use uuid::Uuid;

static EVENT_SENDER: OnceCell<mpsc::UnboundedSender<Event>> = OnceCell::new();
pub(crate) type EventReceiver = mpsc::UnboundedReceiver<Event>;

pub fn init() -> EventReceiver {
    let (sender, receiver) = unbounded_channel();
    EVENT_SENDER.set(sender).unwrap();
    receiver
}

pub fn send(node_id: Uuid, event: impl Any + Send + Sync) {
    EVENT_SENDER
        .get()
        .unwrap()
        .send(Event {
            inner: Box::new(NamsexEvent::NodeEvent {
                node_id,
                event: Event {
                    inner: Box::new(event),
                },
            }),
        })
        .unwrap();
}

pub(crate) enum NamsexEvent {
    NodeEvent { node_id: Uuid, event: Event },
}

#[derive(Debug)]
pub struct Event {
    inner: Box<dyn Any + Send + Sync>,
}
impl Event {
    pub fn is<T: 'static>(self, callback: impl FnOnce(T)) -> EventIsChain {
        EventIsChain::Continue(self).is(callback)
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref::<T>()
    }
    pub fn downcast<T: 'static>(self) -> Result<T, Self> {
        self.inner
            .downcast::<T>()
            .map(|inner| *inner)
            .map_err(|inner| Self { inner })
    }
}

pub enum EventIsChain {
    Continue(Event),
    Break,
}

impl EventIsChain {
    pub fn is<T: 'static>(self, callback: impl FnOnce(T)) -> EventIsChain {
        match self {
            EventIsChain::Continue(event) => match event.downcast::<T>() {
                Ok(casted_event) => {
                    callback(casted_event);
                    EventIsChain::Break
                }
                Err(this) => EventIsChain::Continue(this),
            },
            EventIsChain::Break => EventIsChain::Break,
        }
    }
}
