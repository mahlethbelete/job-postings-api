# Job Postings API

A small REST API in Rust, built with Axum, that serves job postings with pagination, filtering and lookup by ID.

## Requirements

Rust 1.85 or newer (install with [rustup](https://rustup.rs)).

## Run

```bash
git clone https://github.com/mahlethbelete/job-postings-api.git
cd job-postings-api
cargo run
```

The server starts on `http://127.0.0.1:3000` and loads `data/job_postings.json` at startup.

## Endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/postings?page=1&per_page=20` | List postings with pagination |
| GET | `/postings/search?company=&role=&location=` | Filter postings, paginated |
| GET | `/postings/{id}` | Get one posting by ID |

Pagination: `page` starts at 1 (default 1), `per_page` is 1 to 100 (default 20).

Search: case insensitive partial match. `role` matches the job title. At least one filter is required. Multiple filters are combined.

### Examples

```bash
curl "http://127.0.0.1:3000/postings?page=2&per_page=5"
curl "http://127.0.0.1:3000/postings/search?role=manager&location=CA"
curl "http://127.0.0.1:3000/postings/1"
```

List response:

```json
{
  "data": [{ "id": 1, "title": "Marketing Coordinator", "company": "Corcoran Sawyer Smith", "location": "Princeton, NJ", "employment_type": "Full-time", "salary_min": 17, "salary_max": 20 }],
  "page": 1,
  "per_page": 1,
  "total": 150
}
```

### Errors

Errors return JSON with a matching status code:

```json
{ "error": "posting 9999 not found" }
```

| Status | When |
|---|---|
| 400 | Invalid ID, invalid `page` or `per_page`, search with no filters |
| 404 | Posting ID does not exist |

## Tests

```bash
cargo test
```

11 tests: unit tests for the data store and integration tests that send HTTP requests to every endpoint, including error cases.

## Data

`data/job_postings.json` contains 150 postings sampled from the Kaggle "LinkedIn Job Postings 2023 2024" dataset. I reduced each posting to the fields the API needs, assigned sequential IDs and converted missing salaries to `null`. The conversion was a one time Python step and is not part of this repo.

## Design

* Data is loaded once at startup and shared across requests with `Arc`.
* Postings are indexed by ID in a `HashMap`, so lookup by ID is constant time.
* Pagination clones only the postings on the requested page.
* `per_page` is capped at 100 to keep responses small.

## Known limitations

* Salaries are not comparable: some are hourly and some yearly, because the source `pay_period` field was not kept.
* Some company names are empty or have trailing spaces, as in the source data.
* Data is in memory and read only. Changes to the file need a restart.
* Search scans all postings on each request. Fine for 150 rows, not for large datasets.
* Address and data path are hardcoded.
* No authentication or rate limiting.

## What I would improve next

* Store postings in SQLite or PostgreSQL with indexes on company, title and location.
* Normalize salaries using the pay period and clean up company names.
* Read the address and data path from environment variables.
* Add structured request logging with `tracing`.
* Add a summary endpoint, for example postings per company.
* Add a Dockerfile and a CI workflow that runs fmt, clippy and tests.