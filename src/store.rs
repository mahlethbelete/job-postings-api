use std::{collections::HashMap, fs, path::Path};

use thiserror::Error;

use crate::models::JobPosting;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("failed to read {path}: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("failed to parse {path}: {source}")]
    Parse {
        path: String,
        source: serde_json::Error,
    },
}

pub struct PostingStore {
    postings: Vec<JobPosting>,
    by_id: HashMap<u32, usize>,
}

impl PostingStore {
    pub fn new(postings: Vec<JobPosting>) -> Self {
        let by_id = postings
            .iter()
            .enumerate()
            .map(|(index, posting)| (posting.id, index))
            .collect();
        Self { postings, by_id }
    }

    pub fn from_json_file(path: &Path) -> Result<Self, LoadError> {
        let path_str = path.display().to_string();
        let contents = fs::read_to_string(path).map_err(|source| LoadError::Io {
            path: path_str.clone(),
            source,
        })?;
        let postings = serde_json::from_str(&contents).map_err(|source| LoadError::Parse {
            path: path_str,
            source,
        })?;
        Ok(Self::new(postings))
    }

    pub fn all(&self) -> &[JobPosting] {
        &self.postings
    }

    pub fn get(&self, id: u32) -> Option<&JobPosting> {
        self.by_id.get(&id).map(|&index| &self.postings[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn posting(id: u32) -> JobPosting {
        JobPosting {
            id,
            title: "Backend Engineer".into(),
            company: "Acme".into(),
            location: "Remote".into(),
            employment_type: "Full-time".into(),
            salary_min: None,
            salary_max: None,
        }
    }

    #[test]
    fn get_returns_posting_by_id() {
        let store = PostingStore::new(vec![posting(1), posting(2)]);
        assert_eq!(store.get(2).map(|p| p.id), Some(2));
    }

    #[test]
    fn get_returns_none_for_missing_id() {
        let store = PostingStore::new(vec![posting(1)]);
        assert!(store.get(99).is_none());
    }

    #[test]
    fn from_json_file_loads_sample_data() {
        let store = PostingStore::from_json_file(Path::new("data/job_postings.json")).unwrap();
        assert_eq!(store.all().len(), 150);
    }

    #[test]
    fn from_json_file_reports_missing_file() {
        let result = PostingStore::from_json_file(Path::new("missing.json"));
        assert!(matches!(result, Err(LoadError::Io { .. })));
    }
}
