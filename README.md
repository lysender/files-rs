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

## Models

Client:
- id
- name

Bucket:
- id
- client_id
- name
- label

Directory:
- id
- bucket_id
- dir_type
- name
- label
- file_count
- created_at
- updated_at

Files do not have a model represented in the database. They are stored in the storage service.

To list directory files, simply fetch them from the cloud storage service.

## API Endpoints

- GET /api/v1/buckets/:bucket_id/dirs
- POST /api/v1/buckets/:bucket_id/dirs
- GET /api/v1/buckets/:bucket_id/dirs/:dir_id/files

