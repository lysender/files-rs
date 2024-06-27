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

### Setup Admin User

Run the following:
```bash
# Dev mode
cargo run -- generate-login
# Prod mode
files-rs generate-login
```

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

## Database client setup

```
# Only when not yet installed 
sudo apt-get -y install libsqlite3-dev

# Required by our ORM and migration tool
cargo install diesel_cli --no-default-features --features sqlite
```

## Configuration by ENV variables

```
DATABASE_URL=sqlite://db/db.sqlite3
CLIENT_ID=value
ADMIN_HASH=value
JWT_SECRET=value
PORT=11001

GOOGLE_PROJECT_ID=value
GOOGLE_APPLICATION_CREDENTIALS=/path/to/credentials.json
```

## Build binary

```
cargo build --release
```

## Deployment

You can deploy the application in many ways. In this example, we deploy
it as a simple systemd service.

### Setup systemd

Edit systemd service file:

```
sudo systemctl edit --force --full files-rs.service
```

File: `/etc/systemd/system/files-rs.service`

```
[Unit]
Description=files-rs File Management in the cloud

[Service]
User=www-data
Group=www-data


Environment="DATABASE_URL=sqlite:///path/to/db.sqlite3
Environment="CLIENT_ID=value"
Environment="ADMIN_HASH=value"
Environment="JWT_SECRET=value"
Environment="PORT=11001"
Environment="GOOGLE_PROJECT_ID=value"
Environment="GOOGLE_APPLICATION_CREDENTIALS=/path/to/credentials.json"

WorkingDirectory=/data/www/html/sites/files-rs/
ExecStart=/data/www/html/sites/files-rs/target/release/files-rs

[Install]
WantedBy=multi-user.target
```

To enable it for the first time:

```
sudo systemctl enable files-rs.service
```

Various commands:

```
sudo systemctl start files-rs.service
sudo systemctl stop files-rs.service
sudo systemctl restart files-rs.service
sudo systemctl status files-rs.service
```
