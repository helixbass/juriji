use std::collections::HashSet;

use serde::Serialize;
use sqlx::{types::Json, Pool, Postgres, QueryBuilder};
use tokio::sync::MutexGuard;
use uuid::Uuid;

pub async fn insert_event<TPayload: Serialize>(
    id: Option<Uuid>,
    type_: String,
    payload: &TPayload,
    _db_guard: &MutexGuard<'_, ()>,
    db_pool: &Pool<Postgres>,
) {
    let mut query_builder = QueryBuilder::new("INSERT INTO events (id, type, payload)");
    query_builder.push_values(&[(id, type_, Json(payload))], |mut builder, fields| {
        builder
            .push_bind(fields.0)
            .push_bind(fields.1.clone())
            .push_bind(fields.2);
    });
    let query = query_builder.build();

    query.execute(db_pool).await.unwrap();
}

pub async fn read_events<TPayload: Serialize>(event_types: &HashSet<String>) -> Vec<ReadEvent> {
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
