<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="brandings/lockup-dark.svg">
    <img src="brandings/lockup-light.svg" alt="Silo Engine" width="800">
  </picture>
</p>

<p align="center">
  Sharpnr's self-hosted storage and infrastructure layer.<br>
  <sub>Rust · Axum · Tokio · PostgreSQL · RAID-backed storage</sub>
</p>

---

Silo Engine is Sharpnr's self-hosted infrastructure service for **file
storage, database provisioning, and storage-related resources**.

It is designed to sit underneath Sharpnr Engine and other Sharpnr
applications, providing a centralized storage and infrastructure layer
without making the application services responsible for managing
physical storage directly.

## Overview

Silo is intended to provide an API through which Sharpnr services can:

- Upload and store files
- Retrieve files
- Delete files
- Organize files by organization, employee, and folder
- Provision PostgreSQL databases
- Manage database credentials and connection information
- Track storage and infrastructure metadata
- Authenticate requests from trusted Sharpnr services

A typical file URL will eventually look like:

```text
https://cdn.sharpnr.com/{org_id}/{employee_id}/{folder}/{filename}
```

For example:

```text
https://cdn.sharpnr.com/org_123/employee_456/profile/avatar.png
```

The CDN hostname is intentionally decoupled from the physical storage
implementation. Silo is responsible for resolving the requested resource
and serving it from the configured storage backend.

---

## Architecture

Silo is currently being designed as a standalone Rust service.

```text
                         Internet
                            │
                            ▼
                     cdn.sharpnr.com
                            │
                         Nginx
                            │
                            ▼
                      Silo Engine
                      Rust + Axum
                            │
              ┌─────────────┼─────────────┐
              │             │             │
              ▼             ▼             ▼
          PostgreSQL     Storage        Auth
          Metadata       Backend        Service
              │             │
              │             ▼
              │          RAID Storage
              │
              ▼
        Silo Metadata
```

Silo is **not currently tied to Kubernetes**. The initial deployment
target is self-hosted physical infrastructure.

The current infrastructure includes approximately:

- 10 CPUs combined
- 160 GB RAM combined
- Approximately 1 PB of RAID-backed storage
- Multiple machines connected through a local network switch

The first version of Silo will run as a normal service, with
systemd/Nginx used for deployment and traffic routing.

---

## Responsibilities

### Silo Engine owns

Silo is responsible for infrastructure-level concerns:

- File storage
- File metadata
- Storage paths
- Storage backends
- PostgreSQL database provisioning
- Database credentials
- Resource lifecycle
- Storage usage metadata
- Authentication between Sharpnr services
- File access control

### Sharpnr Engine owns

Sharpnr Engine remains responsible for application/business data.

For example:

```text
Sharpnr Engine
│
├── Hospitality
├── Accounting
├── HR
└── Other business modules
```

Silo provisions and manages infrastructure resources, but it does not
become the business-domain database layer for Sharpnr.

---

## File Storage Model

A file can be addressed using an organization/resource-oriented path:

```text
/org/{org_id}/{employee_id}/{folder}/{filename}
```

Example:

```text
/org_123/employee_456/profile/avatar.png
```

Internally, Silo should not depend on the public URL being the physical
filesystem path.

The public path:

```text
cdn.sharpnr.com/org_123/employee_456/profile/avatar.png
```

should resolve through Silo's storage abstraction to an actual storage
object.

This allows the physical storage implementation to change without
changing public URLs.

---

## Storage Abstraction

Silo will use a storage abstraction rather than coupling the service
directly to one storage technology.

Conceptually:

```text
StorageBackend
│
├── LocalRaidStorage
├── S3Storage
├── CephStorage
└── Other implementations
```

The initial implementation will target the RAID-backed storage available
on the Sharpnr infrastructure.

The goal is to keep the service layer independent of the physical
storage implementation.

For example:

```rust
pub trait StorageBackend {
    async fn put(&self, key: &str, data: &[u8]) -> Result<(), SiloError>;

    async fn get(&self, key: &str) -> Result<Vec<u8>, SiloError>;

    async fn delete(&self, key: &str) -> Result<(), SiloError>;
}
```

The exact API may evolve as streaming and large-file support are
implemented.

---

## Database Provisioning

Silo is also intended to provision PostgreSQL databases for Sharpnr
applications.

Conceptually:

```text
Sharpnr Engine
       │
       │ create database
       ▼
Silo Engine
       │
       ├── Generate database name
       ├── Generate credentials
       ├── Create PostgreSQL database
       ├── Create user
       ├── Apply permissions
       └── Return connection information
```

The resulting information may include:

```text
Host
Port
Database
Username
Password
Connection URL
```

The database itself belongs to the consuming application. Silo is
responsible for provisioning and managing the infrastructure resource.

---

## Authentication

Silo is not intended to expose unrestricted infrastructure operations
publicly.

Trusted Sharpnr services will authenticate with Silo.

For example:

```text
Sharpnr Engine
      │
      │ authenticated request
      ▼
Silo Engine
```

Public file access and internal infrastructure APIs are separate
concerns.

The eventual API design will distinguish between:

```text
Public / CDN access
Internal service-to-service access
Administrative operations
```

---

## API

Silo uses:

- Rust
- Axum
- Tokio
- OpenAPI
- Swagger UI
- ReDoc

API documentation will be exposed through:

```text
/swagger-ui
/redoc
/v1/openapi.json
```

The OpenAPI document is intended to become the contract between Silo and
consuming Sharpnr services.

---

## Configuration

Silo uses a centralized `EngineConfig`. The environment is read once, at
startup, inside the `config` module — nothing else in the crate touches
`std::env`.

The configuration is organized into settings groups:

```text
EngineConfig
├── HostSettings        implemented
├── DatabaseSettings    planned
├── StorageSettings     planned
└── AuthSettings        planned
```

Usage looks like:

```rust
let config = EngineConfig::from_env()?;

config.app_env;          // "development" | "production" | ...
config.host.port;
config.host.bind_addr(); // SocketAddr for the listener
```

### Environment selection

`APP_ENV` selects which env file is loaded, so it is read from the
process environment _before_ any file exists:

```bash
cargo run                     # loads .env.development
APP_ENV=production cargo run  # loads .env.production
```

Real environment variables always win over the file. A missing env file
is only a warning — the app then reads the process environment instead.

Required variables are validated at startup, and a bad value stops the
process with a message naming the variable rather than a panic:

```text
configuration error: SILO_ENGINE_PORT must be set (see .env.example)
configuration error: SILO_ENGINE_PORT=99999 is invalid: not a valid port number
```

Example `.env.development`:

```env
APP_ENV=development
SILO_ENGINE_PORT=9254
SHARPNR_ENGINE_PORT=8080
```

Planned variables, as their settings groups land:

```env
DATABASE_URL=postgres://silo:silo@localhost:5432/silo
DATABASE_MAX_CONNECTIONS=20

STORAGE_ROOT=/var/lib/silo

AUTH_SERVICE_URL=http://localhost:8082
AUTH_SERVICE_TOKEN=development-token
```

`.env.development` and `.env.production` are gitignored. `.env.example`
is the committed template and should never hold real credentials.

---

## Project Structure

The project is being organized around clear application boundaries:

```text
silo-engine/
├── Cargo.toml
├── README.md
├── .env.example
│
└── src/
    ├── main.rs
    │
    ├── config/
    │   ├── mod.rs
    │   ├── engine.rs
    │   ├── host.rs
    │   ├── error.rs
    │   ├── database.rs
    │   ├── storage.rs
    │   └── auth.rs
    │
    ├── api/
    │   ├── mod.rs
    │   ├── router.rs
    │   └── ...
    │
    ├── services/
    │   ├── mod.rs
    │   ├── file_service.rs
    │   └── database_service.rs
    │
    ├── storage/
    │   ├── mod.rs
    │   └── filesystem.rs
    │
    ├── databases/
    │   ├── mod.rs
    │   └── provisioner.rs
    │
    ├── auth/
    │   ├── mod.rs
    │   └── service.rs
    │
    └── errors/
        ├── mod.rs
        └── error.rs
```

The architecture intentionally favors:

```text
Controller
    ↓
Service
    ↓
Infrastructure
```

A repository layer is not required for every service. It will only be
introduced where it provides a meaningful abstraction.

---

## Dependency Injection

Silo uses explicit dependency injection rather than a dedicated DI
framework.

Dependencies are constructed during application startup and passed into
services.

Conceptually:

```text
main.rs
   │
   ├── EngineConfig
   ├── Database Pool
   ├── Storage Backend
   ├── Auth Client
   │
   ▼
Services
   │
   ▼
AppState
   │
   ▼
Axum
```

This keeps dependency construction centralized and makes services easier
to test.

Rust traits will be used where abstraction is useful, particularly for
infrastructure components such as storage backends and database
provisioners.

---

## Error Handling

Silo will use structured application errors.

The intended model is:

```text
SiloError
├── Configuration
├── Database
├── Storage
├── Authentication
├── NotFound
└── Internal
```

Functions that can fail will generally return:

```rust
Result<T, SiloError>
```

The `?` operator will be used to propagate errors through service
layers.

---

## Concurrency

Silo is an asynchronous Rust service built on Tokio.

The primary concurrency model is:

```text
Axum
  ↓
Tokio Runtime
  ↓
Async Tasks
```

Silo will not manually create an OS thread for every HTTP request.

For blocking or CPU-heavy work, Tokio's blocking facilities or dedicated
worker mechanisms can be used where appropriate.

---

## Deployment

The initial deployment model is intentionally simple:

```text
systemd
   │
   ▼
silo-engine
   │
   ├── HTTP :9255            (SILO_ENGINE_PORT)
   ├── PostgreSQL
   └── RAID-backed storage
```

Nginx can sit in front of Silo:

```text
cdn.sharpnr.com
        │
        ▼
      Nginx
        │
        ▼
   Silo Engine :9255
```

The port comes from `SILO_ENGINE_PORT`; development uses `9254` and
production `9255`, so both can run on one machine.

Container orchestration and Kubernetes are intentionally out of scope
for the initial implementation.

---

## Development

Check Rust:

```bash
rustc --version
cargo --version
```

Check installed Rust toolchains:

```bash
rustup toolchain list
```

Run the application:

```bash
cargo run
```

Build:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Check the project without producing a final binary:

```bash
cargo check
```

Format code:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy
```

---

## Design Principles

Silo follows a few core principles:

1.  **Infrastructure should be separated from business logic.**
2.  **Storage should be abstracted behind a backend interface.**
3.  **Configuration should be loaded once and injected.**
4.  **Services should not create their own dependencies.**
5.  **Sharpnr Engine owns application/business databases.**
6.  **Silo owns infrastructure provisioning and storage.**
7.  **Public file URLs should not depend on physical filesystem
    layout.**
8.  **The API should have an explicit OpenAPI contract.**
9.  **The first implementation should remain simple and self-hosted.**
10. **Distributed infrastructure should be introduced only when it is
    actually needed.**

---

## Current Status

Silo Engine is currently in the initial architecture and implementation
phase.

Current foundations include:

- Rust toolchain
- Cargo project
- Configuration module
- Environment-based configuration
- Axum/Tokio direction
- OpenAPI documentation direction
- Dependency injection architecture
- Storage abstraction design
- PostgreSQL provisioning design

The next implementation milestones are:

```text
Configuration
      ↓
Axum server
      ↓
Application state / DI
      ↓
PostgreSQL metadata
      ↓
Storage backend
      ↓
File upload
      ↓
File download
      ↓
Authentication
      ↓
Database provisioning
      ↓
CDN integration
```

---

## Name

**Silo Engine**

Silo is intended to act as Sharpnr's infrastructure/storage silo: a
centralized service that isolates storage and infrastructure management
from the business applications consuming those resources.
