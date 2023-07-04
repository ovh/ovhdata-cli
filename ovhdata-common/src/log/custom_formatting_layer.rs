use serde::ser::{SerializeMap, Serializer};
use serde_json::Value;
use std::io::Write;
use time::format_description::well_known::Rfc3339;
use tracing::{Event, Subscriber};
use tracing_bunyan_formatter::JsonStorage;
use tracing_log::AsLog;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::SpanRef;
use tracing_subscriber::Layer;

pub struct CustomFormattingLayer<W>
where
    W: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    make_writer: W,
}

// This custom formatting layer was inspired from :
// https://github.com/LukeMathWalker/tracing-bunyan-formatter/blob/master/src/formatting_layer.rs
impl<W> CustomFormattingLayer<W>
where
    W: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    pub fn new(make_writer: W) -> CustomFormattingLayer<W> {
        CustomFormattingLayer { make_writer }
    }

    pub fn format<S>(&self, event: &Event<'_>, ctx: Context<'_, S>) -> std::io::Result<Vec<u8>>
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        // Events do not necessarily happen in the context of a span, hence lookup_current
        // returns an `Option<SpanRef<_>>` instead of a `SpanRef<_>`.
        let current_span = ctx.lookup_current();

        let mut event_visitor = JsonStorage::default();
        event.record(&mut event_visitor);

        let mut buffer = Vec::new();

        let mut serializer = serde_json::Serializer::new(&mut buffer);
        let mut map_serializer = serializer.serialize_map(None)?;

        // Compute message field
        let maybe_message = event_visitor.values().get("message");
        let message = Self::message(event, &current_span, maybe_message);
        map_serializer.serialize_entry("msg", &message)?;

        // Compute line field
        let maybe_line = match event.metadata().line() {
            None => event_visitor.values().get("log.line").cloned(),
            Some(line) => Some(Value::from(line)),
        };
        map_serializer.serialize_entry("line", &maybe_line)?;

        // Compute file field
        let maybe_file = match event.metadata().file() {
            None => event_visitor.values().get("log.file").cloned(),
            Some(line) => Some(Value::from(line)),
        };
        map_serializer.serialize_entry("file", &maybe_file)?;

        // Compute target field
        let target = match event_visitor.values().get("log.target") {
            None => Value::from(event.metadata().target()),
            Some(target) => target.clone(),
        };
        map_serializer.serialize_entry("target", &target)?;

        // Compute level field
        map_serializer.serialize_entry("level", &event.metadata().level().as_log().to_string().to_lowercase())?;

        // Compute time field
        if let Ok(time) = &time::OffsetDateTime::now_utc().format(&Rfc3339) {
            map_serializer.serialize_entry("time", time)?;
        }

        // Add all the other fields associated with the event, expect the message we already used.
        for (key, value) in event_visitor
            .values()
            .iter()
            .filter(|(key, _)| !["log.module_path", "log.file", "log.target", "log.line", "message"].contains(key))
        {
            map_serializer.serialize_entry(key, value)?;
        }

        // Add all the fields from the current span, if we have one.
        if let Some(span) = &current_span {
            let extensions = span.extensions();
            if let Some(visitor) = extensions.get::<JsonStorage>() {
                for (key, value) in visitor.values() {
                    map_serializer.serialize_entry(key, value)?;
                }
            }
        }
        map_serializer.end()?;
        Ok(buffer)
    }

    fn message<S>(event: &Event, _: &Option<SpanRef<S>>, maybe_message: Option<&Value>) -> String
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        let message = maybe_message
            .as_ref()
            .and_then(|v| match v {
                Value::String(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or_else(|| event.metadata().target())
            .to_owned();

        message
    }

    fn emit(&self, mut buffer: Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_all(b"\n")?;
        self.make_writer.make_writer().write_all(&buffer)
    }
}

impl<S, W> Layer<S> for CustomFormattingLayer<W>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    W: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let result: std::io::Result<Vec<u8>> = self.format(event, ctx);
        if let Ok(formatted) = result {
            let _ = self.emit(formatted);
        }
    }
}
