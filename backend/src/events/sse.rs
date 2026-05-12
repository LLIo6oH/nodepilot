use std::convert::Infallible;

use axum::response::sse::{Event, KeepAlive, Sse};
use tokio::sync::broadcast;
use tokio_stream::{Stream, StreamExt, wrappers::BroadcastStream};

use crate::events::broadcaster::LifecycleEvent;

pub fn stream(
    receiver: broadcast::Receiver<LifecycleEvent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = BroadcastStream::new(receiver).filter_map(|event_result| match event_result {
        Ok(event) => serde_json::to_string(&event)
            .ok()
            .map(|payload| Ok(Event::default().event("agent_event").data(payload))),
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
