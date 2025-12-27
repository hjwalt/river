#[derive(Debug)]
pub struct StartSuccess {}

#[derive(Debug)]
pub struct StartFailure {}

#[derive(Debug)]
pub struct StopSuccess {}

#[derive(Debug)]
pub struct StopFailure {}

pub trait Service {
    fn start(&self)
    -> impl std::future::Future<Output = Result<StartSuccess, StartFailure>> + Send;

    fn stop(&self) -> impl std::future::Future<Output = Result<StopSuccess, StopFailure>> + Send;
}
