use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Movie {
    id: MovieId,
    name: String,
    year: u16,
    was_good: bool,
}

type MovieId = String;
type Db = Arc<RwLock<HashMap<MovieId, Movie>>>;

#[tokio::main]
async fn main() {
    let db: Db = Db::default();

    let app = Router::new()
        .route("/movie", post(movie_create))
        .route("/movie/{id}", get(movie_get))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Running on port 3000");

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
pub struct GetMovieInput {
    movie_id: String,
}

async fn movie_get(
    Path(movie_id): Path<MovieId>,
    State(db): State<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    let movie = db
        .read()
        .unwrap()
        .get(&movie_id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(movie))
}

#[derive(Serialize, Deserialize)]
pub struct CreateMovie {
    name: String,
    year: u16,
    was_good: bool,
}

async fn movie_create(State(db): State<Db>, Json(input): Json<CreateMovie>) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let new_movie = Movie {
        id: id.clone(),
        name: input.name,
        year: input.year,
        was_good: input.was_good,
    };

    db.write().unwrap().insert(id, new_movie.clone());

    (StatusCode::CREATED, Json(new_movie))
}
