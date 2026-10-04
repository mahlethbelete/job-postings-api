use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{
        Path, Query, State,
        rejection::{PathRejection, QueryRejection},
    },
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, models::JobPosting, store::PostingStore};

const DEFAULT_PER_PAGE: usize = 20;
const MAX_PER_PAGE: usize = 100;

type AppState = Arc<PostingStore>;

pub fn router(store: PostingStore) -> Router {
    Router::new()
        .route("/postings", get(list_postings))
        .route("/postings/{id}", get(get_posting))
        .with_state(Arc::new(store))
}

#[derive(Debug, Deserialize)]
struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Pagination {
    fn resolve(&self) -> Result<(usize, usize), ApiError> {
        let page = self.page.unwrap_or(1);
        let per_page = self.per_page.unwrap_or(DEFAULT_PER_PAGE);
        if page == 0 {
            return Err(ApiError::BadRequest("page must be at least 1".into()));
        }
        if per_page == 0 || per_page > MAX_PER_PAGE {
            return Err(ApiError::BadRequest(format!(
                "per_page must be between 1 and {MAX_PER_PAGE}"
            )));
        }
        Ok((page, per_page))
    }
}

#[derive(Debug, Serialize)]
struct Page {
    data: Vec<JobPosting>,
    page: usize,
    per_page: usize,
    total: usize,
}

fn paginate<'a, I>(items: I, page: usize, per_page: usize) -> Page
where
    I: ExactSizeIterator<Item = &'a JobPosting>,
{
    let total = items.len();
    let data = items
        .skip((page - 1).saturating_mul(per_page))
        .take(per_page)
        .cloned()
        .collect();
    Page {
        data,
        page,
        per_page,
        total,
    }
}

async fn list_postings(
    State(store): State<AppState>,
    pagination: Result<Query<Pagination>, QueryRejection>,
) -> Result<Json<Page>, ApiError> {
    let (page, per_page) = pagination?.0.resolve()?;
    Ok(Json(paginate(store.all().iter(), page, per_page)))
}

async fn get_posting(
    State(store): State<AppState>,
    id: Result<Path<u32>, PathRejection>,
) -> Result<Json<JobPosting>, ApiError> {
    let Path(id) = id?;
    store
        .get(id)
        .cloned()
        .map(Json)
        .ok_or(ApiError::NotFound(id))
}
