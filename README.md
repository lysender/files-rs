# files-rs

Simple file storage service written in Rust

## Objectives

- [ ] Serves JSON API endpoints to manage files
- [ ] Multi-tenant support
- [ ] Multi-bucket support - mapped to a tenant
- [ ] Supports Google Cloud Storage for now
- [ ] SQLite for database
- [ ] Oauth2 and JWT for client authentication

## Workflow

- Define tenants as configuration that includes the following:
  - ID
  - Name
  - Storage Type (Google Cloud Storage, AWS S3, etc)
  - Storage Bucket
  - Oauth2 Client ID and Secret
- Each API request are authenticated against a tenant
- Files are structured as one level directory only:
  - /bucket/contents/dir/files
- Images can have sizes like orig, thumbnail and large
  - /bucket/contents/dir/sizes/thumbnail/dir/files
  - /bucket/contents/dir/sizes/large/dir/files

## Authentication

Acquire auth tokens:
- Send login request to oauth endpoint
- Return access token
- Note: Just use a hardcoded username and password

Verifying authenticated requests:
- Send Authorization header with the following data:
  - Subject -> client_id
  - Scope -> auth files
  - Expires -> should not exceed 1 week
- Validate authorization header using a middleware
- Attach client info as request extension

## Models

Client (don't need persistence for now):
- id
- name

Bucket:
- id
- client_id
- name
- label

Directory:
- id
- dir_type
- bucket_id
- name
- label
- file_count
- created_at
- updated_at

File:
- name
- url

Files do not have a model represented in the database. They are stored in the storage service.

To list directory files, simply fetch them from the cloud storage service.

## API Endpoints

- GET /v1/auth/token
- GET /v1/buckets
- POST /v1/buckets?page=1&per_page=10
- GET /v1/buckets/:bucket_id
- PATCH /v1/buckets/:bucket_id
- DELETE /v1/buckets/:bucket_id
- GET /v1/buckets/:bucket_id/dirs?page=1&per_page=10
- POST /v1/buckets/:bucket_id/dirs
- GET /v1/buckets/:bucket_id/dirs/:dir_id
- PATCH /v1/buckets/:bucket_id/dirs/:dir_id
- DELETE /v1/buckets/:bucket_id/dirs/:dir_id
- GET /v1/buckets/:bucket_id/dirs/:dir_id/files

