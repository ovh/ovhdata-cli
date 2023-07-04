mod custom_formatting_layer;
mod event;

use std::future::Future;
use tokio::task::JoinHandle;
use tracing::subscriber::{set_global_default, SetGlobalDefaultError};
use tracing::Subscriber;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use custom_formatting_layer::CustomFormattingLayer;
use event::DataLogContext;

/// Register a subscriber as global default to process span data.
/// It should only be called once!
pub fn init_subscriber<W>(make_writer: W, json: bool, env_filter: EnvFilter) -> Result<(), SetGlobalDefaultError>
where
    W: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    match json {
        true => _init_subscriber(_get_json_subscriber(make_writer, env_filter)),
        false => _init_subscriber(_get_default_subscriber(make_writer, env_filter)),
    }
}

fn _init_subscriber(subscriber: impl Subscriber + Sync + Send) -> Result<(), SetGlobalDefaultError> {
    // LogTracer convert logger events from other libraries into tracing events
    LogTracer::init().expect("Failed to set logger");
    // Set the global tracing subscriber
    set_global_default(subscriber)
}

fn _get_default_subscriber<W>(make_writer: W, env_filter: EnvFilter) -> impl Subscriber + Sync + Send
where
    W: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_writer(make_writer)
        .compact()
        .finish()
}

fn _get_json_subscriber<W>(make_writer: W, env_filter: EnvFilter) -> impl Subscriber + Sync + Send
where
    W: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    Registry::default()
        // Check log configuration from RUST_LOG env variable
        .with(env_filter)
        // Record span attributes and infos so that they can be retrieved
        .with(JsonStorageLayer)
        .with(CustomFormattingLayer::new(make_writer))
}

pub fn tokio_spawn<T, C>(maybe_log_context: C, future: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
    C: Into<Option<DataLogContext>>,
{
    if let Some(log_context) = maybe_log_context.into() {
        tokio::spawn(run_future_with_log(log_context, future))
    } else {
        tokio::spawn(future)
    }
}

#[tracing::instrument(
    skip(future, ctx),
    name = "tokio_spawn",
    fields(
        project_id = %ctx.project_uuid,
        data_sync_id = %ctx.datasync_uuid,
        volume_id = %ctx.volume_uuid,
        worker_id = %ctx.worker_id
    )
)]
async fn run_future_with_log<T>(ctx: DataLogContext, future: T) -> T::Output
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    future.await
}
