use std::{time::Instant, sync::Arc};

use async_trait::async_trait;

use crate::config::InfoscreenConfig;

#[async_trait]
pub trait ApiQuery : Send + Sync {
    fn get_name(&self) -> String;
    async fn get_response(&self, config: Arc<InfoscreenConfig>) -> (String, Instant);
}
