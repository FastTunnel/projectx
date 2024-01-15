use async_trait::async_trait;

use domain::user::publisher::IUserEventPublisher;
use mem_event_bus::Event;

use crate::APP_STATE;

pub struct UserEventPublisher {}

impl UserEventPublisher {
    pub fn new() -> Self {
        UserEventPublisher {}
    }
}

#[async_trait]
impl IUserEventPublisher for UserEventPublisher {
    async fn publish(
        &self,
        event: domain::user::publisher::UserEvent,
    ) -> domain::error::Result<()> {
        APP_STATE.event_bus.publish(Event::new(event));
        Ok(())
    }
}