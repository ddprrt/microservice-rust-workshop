use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{body::Bytes, Router};

/// Custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Default)]
pub struct AppState {
    db: HashMap<String, Bytes>,
}

pub fn router(_state: &SharedState) -> Router<SharedState> {
    todo!()
}
