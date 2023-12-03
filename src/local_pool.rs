use std::future::Future;

use once_cell::race::OnceBox;
use tokio_util::task::LocalPoolHandle;
use tracing::Instrument;

static LOCAL_POOL: OnceBox<LocalPoolHandle> = OnceBox::new();

pub async fn spawn<F, Fut>(create_task: F) -> Fut::Output
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future + 'static,
    Fut::Output: Send + 'static,
{
    let pool = LOCAL_POOL.get_or_init(|| {
        let cpus = num_cpus::get();
        Box::new(LocalPoolHandle::new(cpus * 2))
    });

    let span = tracing::trace_span!("local_pool_spawn");
    pool.spawn_pinned(|| create_task().instrument(span))
        .await
        .unwrap()
}
