use std::future::Future;

use tokio_util::sync::CancellationToken;

/// Create a cancellable task.
///
/// # Arguments
///
/// * `token`: The token that will be cancelled when the task is cancelled.
/// * `task`: The task to spawn.
#[macro_export]
macro_rules! cancellable_task {
    ( $token:expr, $task:expr ) => {{
        let token = $token.clone();

        async move {
            tokio::select! {
                _ = token.cancelled() => {}
                _ = $task => {}
            }
        }
    }};
}

/// Spawn a task that will be cancelled when the token is cancelled.
///
/// # Arguments
///
/// * `token`: The token that will be cancelled when the task is cancelled.
/// * `task`: The task to spawn.
pub fn spawn_cancellable_task<T>(token: &CancellationToken, task: T) -> tokio::task::JoinHandle<()>
where
    T: Future + Send + 'static,
{
    tokio::spawn(cancellable_task!(token, task))
}
