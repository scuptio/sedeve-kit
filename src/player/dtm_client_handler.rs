use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use scupt_net::endpoint_async::EndpointAsync;
use scupt_net::handle_event::HandleEvent;
use scupt_util::error_type::ET;
use scupt_util::res::Res;
use tracing::error;
use crate::player::msg_ctrl::MessageControl;

struct Handler {}

pub struct DTMClientHandler {
    handler: Arc<Handler>,
}

impl DTMClientHandler {
    pub fn new() -> Self {
        Self {
            handler: Arc::new(Handler {})
        }
    }
}

impl Clone for DTMClientHandler {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
        }
    }
}

#[async_trait]
impl HandleEvent<MessageControl> for DTMClientHandler {
    async fn on_accepted(&self, _: Arc<dyn EndpointAsync<MessageControl>>) -> Res<()> {
        panic!("not possible");
    }

    async fn on_connected(
        &self,
        _address: SocketAddr,
        result_endpoint: Res<Arc<dyn EndpointAsync<MessageControl>>>,
    ) -> Res<()> {
        match result_endpoint {
            Ok(_) => {}
            Err(_) => {}
        }
        Ok(())
    }

    async fn on_error(&self, error: ET) {
        error!("error {:?}", error);
    }

    async fn on_stop(&self) {}
}