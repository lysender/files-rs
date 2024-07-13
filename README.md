# files-rs

Simple file storage service written in Rust

## Objectives

- [x] Serves JSON API endpoints to manage files
- [x] Multi-tenant support
- [x] Multi-bucket support
- [x] Supports Google Cloud Storage for now
- [x] SQLite for database
- [x] Simple JWT authentication
- [x] Tenanats/clients management via CLI
- [x] Teams/users management via CLI
- [x] Buckets management via CLI
- [x] Role based authorization
- [x] Directories CRUD
- [ ] Files upload/delete

## Workflow

- Tenants or clients are assigned with an ID
- Each API request are authenticated against a client 
- A client can have multiple buckets
- Each bucket can have multiple container directories
- Each directory can have the following directories:
  - Contents - original contents
  - Versions
    - Thumbnails
    - Large
    - Etc
- Contents can be any files supported
- Clients may organize their files like a regular storage or an online photo album
- All files must be uploaded through the application, either online or though cli
- Collects mime type, size and image dimentions

### Directory Structure

```
- Bucket
  - Directory
    - Contents
    - Versions
      - Thumbnails
      - Large
      - Etc
```

## Authentication

Acquire auth tokens:
- Send login request to auth endpoint
- Return access token
- Note: Just use a hardcoded username and password

### Clients

Clients are the tenants or customers of the service. They are assigned with an ID.

Each client has access to the following resources:
- teams
- users
- buckets
- directories
- files

All clients are managed via the CLI only.

```bash
./files-rs clients list
./files-rs clients create name
./files-rs clients disable client_id
./files-rs clients enable client_id
./files-rs clients delete client_id
```

Client:
- id
- name
- status: active, inactive
- created_at

### Teams/Users

Each clients are provided with a team, which are users able to access the client resources.
Teams are managed via CLI only as well.

```bash
./files-rs users list client_id
./files-rs users create client_id username
./files-rs users enable id 
./files-rs users disable id 
./files-rs users delete id 
```

User:
- id
- client_id
- username
- password
- status: active, inactive
- roles: csv of roles
- created_at
- updated_at

Usename is unique globally although it is namespaced by client_id.

### Roles

- FilesAdmin
- FilesEditor
- FilesViewer

### Permissions

- dirs.create
- dirs.edit
- dirs.delete
- dirs.list
- dirs.view
- dirs.manage
- files.create
- files.edit
- files.delete
- files.list
- files.view
- files.manage

### Roles to Permissions Mapping

FilesAdmin:
- dirs.create
- dirs.edit
- dirs.delete
- dirs.list
- dirs.view
- dirs.manage
- files.create
- files.edit
- files.delete
- files.list
- files.view
- files.manage

Summary: Admins have full access to directories and files

FilesEditor:
- dirs.list
- dirs.view
- files.create
- files.list
- files.view

Summary: Editors can view and upload new files

FilesViewer:
- dirs.list
- dirs.view
- files.list
- files.view

Summary: Viewers can only view directories and files

## Buckets

Buckets are created outside of the application, like in Google Console or using gsutil.

They are added into the client resources via the CLI.

```bash
./files-rs buckets list client_id
./files-rs buckets create client_id bucket_name
./files-rs buckets delete bucket_id
```

### Setup Admin User

Run the following:

```bash
./files-rs generate-login
```

Verifying authenticated requests:
- Send Authorization header with the following data:
  - Subject -> client_id
  - Scope -> auth files
  - Expires
- Validate authorization header using a middleware
- Attach client info as request extension

## Models

Bucket:
- id
- client_id
- name
- created_at

Dir:
- id
- bucket_id
- name
- label
- file_count
- created_at
- updated_at

File:
- id
- dir_id 
- name
- filename
- content_type
- size
- is_image
- img_dimention 
- img_versions
- created_at
- updated_at

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
JWT_SECRET=value
UPLOAD_DIR=/path/to/upload_dir
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


Environment="DATABASE_URL=sqlite:///path/to/db.sqlite3"
Environment="UPLOAD_DIR=/path/to/upload_dir"
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
