use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use hyper::body::Bytes;
use image::DynamicImage;

pub enum StoredType {
    Image(DynamicImage),
    Other(Bytes),
}

#[derive(Default)]
pub struct AppState {
    pub db: HashMap<String, StoredType>,
}

/// Custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;
