use std::collections::HashSet;

use serde::{de::DeserializeOwned, Serialize};
use sqlx::{types::Json, FromRow, Pool, Postgres, QueryBuilder};
use squalid::IntoCow;
use tokio::sync::MutexGuard;
use tracing::instrument;
use uuid::Uuid;

#[instrument(level = "trace", skip(_db_guard, db_pool))]
pub async fn insert_event(
    event: EventForInsertion,
    _db_guard: MutexGuard<'_, ()>,
    db_pool: &Pool<Postgres>,
) {
    let mut query_builder = QueryBuilder::new("INSERT INTO events (id, type, payload)");
    query_builder.push_values([event], |mut builder, event| {
        builder
            .push_bind(event.id)
            .push_bind(event.type_)
            .push_bind(Json(event.payload));
    });
    let query = query_builder.build();

    query.execute(db_pool).await.unwrap();
}

#[instrument(level = "trace", skip(events, _db_guard, db_pool))]
pub async fn insert_events<'a, TItem: IntoCow<'a, EventForInsertion>>(
    events: impl IntoIterator<Item = TItem>,
    _db_guard: MutexGuard<'_, ()>,
    db_pool: &Pool<Postgres>,
) {
    let mut query_builder = QueryBuilder::new("INSERT INTO events (id, type, payload)");
    let events = events
        .into_iter()
        .map(|event| event.into_cow().into_owned());
    query_builder.push_values(events, |mut builder, event| {
        builder
            .push_bind(event.id)
            .push_bind(event.type_)
            .push_bind(Json(event.payload));
    });
    let query = query_builder.build();

    query.execute(db_pool).await.unwrap();
}

#[derive(Clone, Debug)]
pub struct EventForInsertion {
    pub id: Option<Uuid>,
    pub type_: String,
    pub payload: serde_json::Value,
}

impl EventForInsertion {
    pub fn new(id: Option<Uuid>, type_: String, payload: serde_json::Value) -> Self {
        Self { id, type_, payload }
    }
}

#[instrument(level = "trace", skip(db_pool))]
pub async fn read_events(
    // TODO: narrow to only select event types
    event_types: Option<&HashSet<String>>,
    db_pool: &Pool<Postgres>,
) -> Vec<ReadEvent> {
    sqlx::query_as::<_, ReadEvent>("SELECT id, type, payload FROM events ORDER BY event_id ASC")
        .fetch_all(db_pool)
        .await
        .unwrap()
}

#[derive(FromRow)]
pub struct ReadEvent {
    pub id: Option<Uuid>,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub payload: String,
}

impl ReadEvent {
    pub fn new(id: Option<Uuid>, type_: String, payload: String) -> Self {
        Self { id, type_, payload }
    }
}

#[instrument(level = "trace", skip(value))]
pub fn to_serde_json_value_without_id<TSerializable: Serialize>(
    value: &TSerializable,
) -> serde_json::Value {
    let mut value = serde_json::to_value(value).unwrap();
    let _ = value.as_object_mut().unwrap().remove("id").unwrap();
    value
}

#[instrument(level = "trace")]
pub fn from_json_str_with_id<TTarget: DeserializeOwned>(json_str: &str, id: Uuid) -> TTarget {
    let mut value: serde_json::Value = serde_json::from_str(json_str).unwrap();
    let id_value = serde_json::to_value(id).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("id".to_owned(), id_value);
    serde_json::from_value(value).unwrap()
}
