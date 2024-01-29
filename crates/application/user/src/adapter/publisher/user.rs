use app_interface::APP_STATE;
use async_trait::async_trait;

use domain_common::error;
use domain_user::publisher::{IUserEventPublisher, UserEvent};
use mem_event_bus::Event;

pub struct UserEventPublisher {}

impl UserEventPublisher {
    pub fn new() -> Self {
        UserEventPublisher {}
    }
}

#[async_trait]
impl IUserEventPublisher for UserEventPublisher {
    async fn publish(&self, event: UserEvent) -> error::Result<()> {
        APP_STATE.event_bus.publish(Event::new(event));
        Ok(())
    }
}
