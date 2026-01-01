use async_trait::async_trait;

#[derive(Debug)]
pub struct InitSuccess {}

#[derive(Debug)]
pub struct InitFailure {}

#[derive(Debug)]
pub struct StopSuccess {}

#[derive(Debug)]
pub struct StopFailure {}

#[derive(Debug)]
pub struct RunSuccess {}

#[derive(Debug)]
pub struct RunFailure {}

#[async_trait]
pub trait Service {
    async fn run(&self) -> Result<RunSuccess, RunFailure>;
    async fn init(&self) -> Result<InitSuccess, InitFailure>;
    async fn stop(&self) -> Result<StopSuccess, StopFailure>;
}
