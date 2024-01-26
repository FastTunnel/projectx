use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::RwLock;

use async_trait::async_trait;

pub trait IEvent: Send + Sync {
    type Data: 'static;
    fn get_type(&self) -> TypeId {
        TypeId::of::<Self::Data>()
    }
    fn get_data(&self) -> &Self::Data;
}

pub struct Event<T: 'static> {
    inner: T,
}

impl<T: 'static> Event<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: 'static + Sync + Send> IEvent for Event<T> {
    type Data = T;

    fn get_data(&self) -> &Self::Data {
        &self.inner
    }
}

#[async_trait]
pub trait Subscriber {
    type Input: 'static;

    /// Called when the event bus is run.
    async fn on_event(&self, event: &dyn IEvent<Data = Self::Input>);
}

#[derive(Default)]
pub struct EventBus {
    pub subscribers: RwLock<Vec<Box<dyn Any>>>,
}

unsafe impl Sync for EventBus {}
unsafe impl Send for EventBus {}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(Vec::new()),
        }
    }

    pub fn subscribe<T>(&mut self, subscriber: Box<dyn Subscriber<Input = T>>)
    where
        T: 'static,
    {
        self.subscribers.write().unwrap().push(Box::new(subscriber));
    }

    pub fn publish<E>(&self, event: E)
    where
        E: IEvent + 'static,
        E::Data: 'static,
    {
        futures::executor::block_on(async {
            match self.subscribers.read() {
                Ok(guard) => {
                    for subscriber in guard.deref() {
                        if subscriber.type_id() == event.get_type() {
                            let subscriber = subscriber
                                .downcast_ref::<Box<dyn Subscriber<Input = E::Data>>>()
                                .unwrap();
                            subscriber.on_event(&event).await;
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("read lock error: {:?}", e);
                }
            }
        });
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    #[test]
    fn test_event() {
        let event = Event::new(1);
        assert_eq!(event.get_type(), TypeId::of::<i32>());
        assert_eq!(event.get_data(), &1);
    }

    #[test]
    fn test_event_bus() {
        let mut event_bus = EventBus::new();
        struct TestSubscriber;

        #[async_trait]
        impl Subscriber for TestSubscriber {
            type Input = i32;

            async fn on_event(&self, event: &dyn IEvent<Data = Self::Input>) {
                assert_eq!(event.get_data(), &1);
            }
        }

        event_bus.subscribe(Box::new(TestSubscriber));
        event_bus.publish(Event::new(1));
    }
}
