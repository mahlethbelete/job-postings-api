use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: u32,
    pub title: String,
    pub company: String,
    pub location: String,
    pub employment_type: String,
    pub salary_min: Option<f64>,
    pub salary_max: Option<f64>,
}