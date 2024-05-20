# files-rs

Simple file storage service written in Rust

## Objectives

- [ ] Serves JSON API endpoints to manage files
- [ ] Multi-tenant support
- [ ] Supports Google Cloud Storage for now
- [ ] sqlite for database
- [ ] Oauth2 and JWT for client authentication

## Workflow

- Define tenants as configuration that includes the following:
  - ID
  - Name
  - Storage Type (Google Cloud Storage, AWS S3, etc)
  - Storage Credentials
  - Storage Bucket
  - Oauth2 Client ID and Secret
- Each API request are authenticated against a tenant
- Files are structured as one level directory only:
  - /tenant_bucket/contents/dir/files
- Images can have sizes like orig, thumbnail and large
  - /tenant_bucket/img-sizes/thumbnail/dir/files
  - /tenant_bucket/img-sizes/large/dir/files

## Models

Files do not have a model represented in the database. They are stored in the storage service.

Directories however, are represented in the database.

Directory:
- ID
- tenant_id
- name
- etc

To list directory files, simply fetch them from the cloud storage service.

## API Endpoints

- GET /api/v1/directories
- POST /api/v1/directories
- GET /api/v1/directories/:directory_id/files
