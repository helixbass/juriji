use std::collections::HashSet;

use sqlx::{types::Json, Pool, Postgres, QueryBuilder};
use tokio::sync::MutexGuard;
use uuid::Uuid;

pub async fn insert_event(
    event: EventForInsertion,
    _db_guard: &MutexGuard<'_, ()>,
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

pub async fn read_events(event_types: &HashSet<String>) -> Vec<ReadEvent> {
    unimplemented!()
}

pub struct ReadEvent {
    pub id: Option<Uuid>,
    pub type_: String,
    pub payload: String,
}

impl ReadEvent {
    pub fn new(id: Option<Uuid>, type_: String, payload: String) -> Self {
        Self { id, type_, payload }
    }
}

pub trait CobbleAll: Sized {
    fn relevant_event_types(&self) -> HashSet<String>;

    fn cobble(events: &[ReadEvent]) -> Vec<Self>;
}
