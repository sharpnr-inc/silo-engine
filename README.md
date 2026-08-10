<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="brandings/banners/twitter-header-1500x500.png">
    <img src="brandings/banners/twitter-header-1500x500.png" alt="Silo Engine" width="1000">
  </picture>
</p>

<p align="center">
  Sharpnr's self-hosted storage, database, and data infrastructure platform.<br>
  <sub>Rust · Axum · Tokio · PostgreSQL · RAID-backed storage</sub>
</p>

---

## What is Silo Engine?

**Silo Engine** is Sharpnr's self-hosted infrastructure platform.

It is being designed as the infrastructure layer underneath Sharpnr Engine and other Sharpnr applications, with the long-term goal of providing organizations with a unified platform for:

- File and object storage
- CDN-backed file delivery
- PostgreSQL database provisioning
- Database exploration and management
- Organization and employee storage quotas
- Compression and checksums
- Audit logging
- Data Lakes
- Data Warehouses
- Data ingestion and transformation
- Analytics infrastructure
- Backups and snapshots
- Future search and AI/data services

The important boundary is:

> **Sharpnr Engine owns business logic. Silo Engine owns infrastructure and data resources.**

Silo is not intended to become the Hospitality, Accounting, HR, or other business-domain application database.

---

## Product Vision

Silo is intentionally evolving beyond a simple "Google Drive clone".

The long-term vision is a self-hosted **Storage + Database + Data Platform as a Service**.

```text
                             SILO
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
          ▼                   ▼                   ▼
      STORAGE              DATABASE              DATA
          │                   │                   │
          │              PostgreSQL       ┌────────┴────────┐
          │                   │            │                 │
          ▼                   ▼            ▼                 ▼
        Files             DB Explorer   Data Lake       Data Warehouse
          │                   │            │                 │
          ▼                   │            └────────┬────────┘
        CDN                     │                     ▼
                                │               Query / Analytics
                                │
                                ▼
                           Backups / Snapshots
```

An organization should eventually be able to log into Silo and create infrastructure resources without needing to understand the physical servers, RAID layout, or storage paths underneath them.

---

## Core Resources

### 1. File / Object Storage

Organizations can store:

- Images
- Documents
- Videos
- Attachments
- Backups
- Exports
- Logs
- Arbitrary objects

A public resource can be addressed using a stable URL such as:

```text
https://cdn.sharpnr.com/{org_id}/{employee_id}/{folder}/{filename}
```

Example:

```text
https://cdn.sharpnr.com/org_123/employee_456/profile/avatar.png
```

The public URL is deliberately independent of the physical filesystem.

```text
Public URL
    │
    ▼
CDN / Nginx
    │
    ▼
Silo Engine
    │
    ▼
Storage abstraction
    │
    ▼
Physical storage
```

Authentication and authorization for protected resources are separate from the physical storage path. Public and private file access can therefore be handled differently without changing the underlying object.

---

### 2. PostgreSQL as a Service

Organizations can eventually create PostgreSQL databases through Silo.

Conceptually:

```text
Organization
      │
      ▼
Create Database
      │
      ▼
Silo Engine
      │
      ├── Create database
      ├── Create PostgreSQL roles
      ├── Apply permissions
      ├── Generate credentials
      └── Return connection information
```

A connection may look like:

```text
postgres://org_123_app:PASSWORD@silo-engine-db.sharpnr.com:5432/org_123_db?sslmode=require
```

The physical PostgreSQL server or cluster is an implementation detail.

### PostgreSQL roles

An organization should not receive PostgreSQL superuser credentials.

Silo can create organization-scoped roles such as:

```text
org_123_app
org_123_migrator
org_123_readonly
org_123_admin
```

Group roles can be used when direct database access needs to be granted to multiple database users.

However, Sharpnr employees are primarily application-level identities. Silo should not automatically create a PostgreSQL login for every employee simply because that employee exists in Sharpnr.

---

## Database Explorer

Silo will also act as a lightweight PostgreSQL client through its web interface.

For V1, **there will be no arbitrary SQL editor**.

Instead, the Silo frontend will expose controlled database operations:

```text
Database
├── Schemas
├── Tables
├── Views
├── Functions
├── Indexes
├── Constraints
└── Data
```

A table can be inspected through:

```text
Table
├── Schema
├── Columns
├── Constraints
├── Indexes
└── Rows
```

The browser should communicate with Silo over HTTPS:

```text
Silo Frontend
      │
      │ HTTPS
      ▼
Silo Engine
      │
      │ PostgreSQL
      ▼
Organization Database
```

The browser should **not** connect directly to PostgreSQL.

An SQL workspace may be added later after the controlled database explorer is mature.

---

# Data Lake

A major part of Silo's future is providing **Data Lake as a Service**.

An organization should be able to create a lake:

```text
Silo
└── Data Lakes
    └── Acme Analytics
```

A lake is primarily a large, flexible storage layer for raw, semi-structured, and structured data.

Example:

```text
Acme Analytics
├── raw/
│   ├── orders/
│   ├── customers/
│   ├── events/
│   └── payments/
│
├── processed/
│   ├── orders/
│   └── customers/
│
└── curated/
    ├── daily_sales/
    ├── customer_metrics/
    └── product_metrics/
```

Supported or planned dataset formats include:

```text
Parquet
JSON
JSONL
CSV
Avro
ORC
```

For analytical workloads, **Parquet is the primary format being considered** because it is columnar and works well with analytical query engines.

### Data Lake ingestion

Silo should eventually support multiple ingestion paths:

```text
Files ──────────────┐
                    │
PostgreSQL ─────────┤
                    ├──► Data Lake
Events / APIs ──────┤
                    │
Scheduled Jobs ─────┘
```

A PostgreSQL dataset could, for example, be exported periodically:

```text
PostgreSQL
    │
    ▼
orders
    │
    ▼
Transformation
    │
    ▼
Parquet
    │
    ▼
Data Lake

lake://acme/orders/year=2026/month=08/day=10/
```

Partitioning, retention, ingestion, and lifecycle management will be handled by Silo rather than exposed as physical filesystem concerns.

---

# Data Warehouse

Silo is also intended to provide **Data Warehouse as a Service**.

The conceptual distinction is:

```text
Data Lake
    │
    │ raw / flexible / large-scale storage
    ▼
Transformation / ETL / ELT
    │
    ▼
Data Warehouse
    │
    │ analytical workloads
    ▼
Reports / Analytics / Applications
```

An organization could create:

```text
Analytics Warehouse
├── customers
├── orders
├── revenue
└── inventory
```

The first implementations should leverage mature open-source storage/query technologies rather than attempting to build an analytical database engine from scratch.

DuckDB is one technology being considered for early analytical/query workloads, particularly for querying Parquet directly. A distributed query engine can be introduced later if the scale and workload require it.

---

# Storage Architecture

Silo uses a storage abstraction rather than coupling application services directly to a physical storage implementation.

Conceptually:

```text
StorageBackend
│
├── LocalRaidStorage
├── S3Storage
├── CephStorage
└── Other implementations
```

The initial implementation targets the RAID-backed storage available on Sharpnr infrastructure.

The goal is:

```text
Application
    │
    ▼
Silo Storage API
    │
    ▼
StorageBackend
    │
    ▼
Physical Storage
```

This allows the physical storage implementation to change without changing the public Silo API.

A storage backend may eventually expose streaming operations rather than loading complete large files into memory.

---

# Organization and Employee Storage

Silo is intended to support organization-level and employee-level storage quotas.

For example:

```text
Organization
├── Quota: 100 GB
│
├── Employee A
│   └── Quota: 20 GB
│
├── Employee B
│   └── Quota: 10 GB
│
└── Remaining organization capacity
```

The organization can decide how much of its allocated capacity is available to each employee.

Silo should track:

```text
Organization storage limit
Organization storage used
Employee storage limit
Employee storage used
Object size
Object count
```

Quota enforcement belongs in Silo rather than in the consuming Sharpnr applications.

---

# Checksums

Silo will calculate checksums for stored objects.

The checksum can be used for:

- Integrity verification
- Duplicate detection
- Upload verification
- Auditing
- Storage consistency checks

Conceptually:

```text
Upload
  │
  ▼
Read stream
  │
  ├── checksum
  │
  └── storage write
          │
          ▼
      Object metadata
```

The exact checksum algorithm and implementation can evolve independently of the storage API.

---

# Compression

Compression is a storage subsystem rather than a generic utility.

The initial architecture is designed around streaming compression:

```text
Reader
  │
  ▼
Compressor
  │
  ▼
Writer
```

This is important for large objects because Silo should not load multi-gigabyte files entirely into memory.

The conceptual module is:

```text
compression/
├── mod.rs
├── error.rs
├── traits.rs
├── macros.rs
└── algorithms/
    ├── mod.rs
    └── zstd.rs
```

The initial algorithm under consideration is **Zstandard (Zstd)**.

A trait-based abstraction allows additional algorithms to be added later:

```text
Compressor
├── ZstdCompressor
├── BrotliCompressor
└── Lz4Compressor
```

Macros may be used to generate repetitive implementation boilerplate, but the actual compression operation remains an explicit runtime service.

Silo should also avoid compressing formats that are already heavily compressed unless there is a specific reason to do so.

---

# Audit Log

Silo needs an audit system for infrastructure and data operations.

The long-term design is inspired by the durability and append-oriented characteristics of PostgreSQL WAL, but **the audit log is not intended to replace PostgreSQL WAL**.

The audit system should record events such as:

```text
file.uploaded
file.downloaded
file.deleted

database.created
database.deleted
database.permission_changed

storage.quota_changed

database.accessed
database.schema.modified

lake.created
dataset.created
dataset.ingested
```

A conceptual event:

```json
{
  "event": "file.uploaded",
  "organization_id": "...",
  "actor_id": "...",
  "resource_id": "...",
  "timestamp": "...",
  "metadata": {}
}
```

Audit events should be append-oriented, durable, and suitable for later querying and retention policies.

Sensitive payloads should not automatically be copied into audit records.

---

# Future Data Pipelines

The Data Lake and Warehouse services naturally lead to a pipeline system.

For example:

```text
PostgreSQL
    │
    ▼
Extract
    │
    ▼
Transform
    │
    ▼
Parquet
    │
    ▼
Data Lake
    │
    ▼
Warehouse
```

A scheduled pipeline could run:

```text
Every day at 02:00

Production DB
      ↓
orders
      ↓
Transform
      ↓
Parquet
      ↓
Data Lake
      ↓
Analytics Warehouse
```

Silo workers will eventually handle long-running and asynchronous operations such as:

- Large file processing
- Compression
- Checksum calculation
- Data ingestion
- PostgreSQL exports
- Lake/warehouse jobs
- Backups
- Snapshots
- Retention cleanup

---

# Future Services

The long-term Silo platform may grow into several infrastructure services:

```text
Silo
│
├── File / Object Storage
├── CDN
├── PostgreSQL
├── Database Explorer
├── Backups
├── Snapshots
│
├── Data Lakes
├── Data Warehouses
├── Data Ingestion
├── ETL / ELT
├── Query Engine
│
├── Search
├── Vector Storage
└── AI / RAG Data Infrastructure
```

These are **future capabilities**, not all current implementations.

The guiding principle is to add infrastructure only when the underlying storage and operational model can support it reliably.

---

# Architecture

Silo is currently being designed as a standalone Rust service.

```text
                             Internet
                                │
                ┌───────────────┴────────────────┐
                │                                │
                ▼                                ▼
        cdn.sharpnr.com                  silo-engine-db.sharpnr.com
                │                                │
              Nginx                         DB entry point
                │                                │
                ▼                                ▼
          Silo Engine                    PostgreSQL Cluster(s)
          Rust + Axum                         │
                │                    ┌──────────┴──────────┐
       ┌────────┼────────┐           ▼                     ▼
       │        │        │        Org Database         Org Database
       ▼        ▼        ▼
   Storage   Database   Auth
       │        │
       ▼        ▼
   RAID       Metadata
       │
       ▼
   Data Lake / Future Data Services
```

The exact network topology will evolve as the platform grows.

Silo is **not currently tied to Kubernetes**. The initial deployment target is self-hosted physical infrastructure.

The current infrastructure includes approximately:

- 10 CPUs combined
- 160 GB RAM combined
- Approximately 1 PB of RAID-backed storage
- Multiple machines connected through a local network switch

The first version will run as a normal service using systemd/Nginx and direct infrastructure management.

---

# Responsibilities

## Silo Engine owns

Silo is responsible for infrastructure-level concerns:

- File storage
- File metadata
- Storage paths
- Storage backends
- Storage quotas
- Checksums
- Compression
- PostgreSQL provisioning
- Database credentials
- Database resource lifecycle
- Database explorer APIs
- Data Lakes
- Data Warehouse infrastructure
- Data ingestion
- Backups and snapshots
- Audit logging
- Authentication between trusted services
- File/resource access control

## Sharpnr Engine owns

Sharpnr Engine remains responsible for application/business data and business logic.

For example:

```text
Sharpnr Engine
│
├── Hospitality
├── Accounting
├── HR
├── CRM
└── Other business modules
```

Silo provides the infrastructure resources those applications consume.

---

# Dependency Injection

Silo uses explicit dependency injection rather than a dedicated DI framework.

Dependencies are constructed during application startup and passed into services.

Conceptually:

```text
main.rs
   │
   ├── EngineConfig
   ├── Database Pool
   ├── Storage Backend
   ├── Auth Client
   └── Other infrastructure
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

This keeps dependency construction centralized and makes services easier to test.

Rust traits will be used where abstraction is useful, particularly for:

- Storage backends
- Compression
- Database provisioners
- Authentication clients
- Other infrastructure providers

A repository layer is **not required for every service**. It will only be introduced when it provides a meaningful abstraction.

---

# Module and Project Structure

Silo is currently organized as a single Rust crate with modules.

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
    │   ├── database_service.rs
    │   ├── lake_service.rs
    │   └── warehouse_service.rs
    │
    ├── storage/
    │   ├── mod.rs
    │   └── filesystem.rs
    │
    ├── databases/
    │   ├── mod.rs
    │   └── provisioner.rs
    │
    ├── compression/
    │   ├── mod.rs
    │   ├── error.rs
    │   ├── traits.rs
    │   ├── macros.rs
    │   └── algorithms/
    │       ├── mod.rs
    │       └── zstd.rs
    │
    ├── utils/
    │   ├── mod.rs
    │   └── uuid.rs
    │
    ├── auth/
    │   ├── mod.rs
    │   └── service.rs
    │
    ├── audit/
    │   ├── mod.rs
    │   └── ...
    │
    ├── workers/
    │   ├── mod.rs
    │   └── ...
    │
    ├── lakes/
    │   ├── mod.rs
    │   └── ...
    │
    └── warehouses/
        ├── mod.rs
        └── ...
```

The module tree is rooted at `main.rs`. Child modules declare their own submodules rather than registering every file directly in `main.rs`.

If Silo later grows into multiple independently reusable crates, a Cargo workspace can be introduced.

---

# Configuration

Silo uses a centralized `EngineConfig`.

Environment variables are loaded once at startup inside the `config` module. Other modules should consume configuration through injected `EngineConfig` rather than reading `std::env` directly.

The configuration is organized into settings groups:

```text
EngineConfig
├── HostSettings
├── DatabaseSettings
├── StorageSettings
└── AuthSettings
```

Usage:

```rust
let config = EngineConfig::from_env()?;

config.app_env;
config.host.port;
config.host.bind_addr();
```

`APP_ENV` selects which environment file is loaded:

```bash
cargo run
APP_ENV=production cargo run
```

The intended environment files are:

```text
.env.development
.env.production
.env.example
```

Real process environment variables take precedence over file-based values.

Required configuration should be validated at startup.

Example:

```env
APP_ENV=development
SILO_ENGINE_PORT=9254
SHARPNR_ENGINE_PORT=8080
```

Planned configuration includes:

```env
DATABASE_URL=postgres://silo:silo@localhost:5432/silo
DATABASE_MAX_CONNECTIONS=20

STORAGE_ROOT=/var/lib/silo

AUTH_SERVICE_URL=http://localhost:8082
AUTH_SERVICE_TOKEN=development-token
```

`.env.development` and `.env.production` should remain gitignored. `.env.example` is the committed template and must never contain real credentials.

---

# API and Documentation

Silo uses:

- Rust
- Axum
- Tokio
- OpenAPI
- Swagger UI
- ReDoc

The intended documentation endpoints are:

```text
/swagger-ui
/redoc
/v1/openapi.json
```

The OpenAPI document is intended to become the contract between Silo and consuming Sharpnr services.

---

# Error Handling

Silo uses structured application errors.

The intended model is:

```text
SiloError
├── Configuration
├── Database
├── Storage
├── Compression
├── Authentication
├── NotFound
└── Internal
```

Functions that can fail generally return:

```rust
Result<T, SiloError>
```

The `?` operator is used to propagate errors through service layers.

---

# Concurrency and Workers

Silo is an asynchronous Rust service built on Tokio.

The primary concurrency model is:

```text
Axum
  ↓
Tokio Runtime
  ↓
Async Tasks
```

Silo should not manually create an OS thread for every HTTP request.

Workers are intended for operations that should not block the request/response lifecycle:

```text
HTTP Request
    │
    ├── small operation ───────► complete immediately
    │
    └── large/long operation
              │
              ▼
           Worker
              │
       ┌──────┼────────┐
       ▼      ▼        ▼
   checksum compression backup
```

Potential worker jobs include:

- Large file checksum calculation
- Compression
- Large object movement
- Data ingestion
- PostgreSQL exports
- Data Lake processing
- Warehouse jobs
- Backup generation
- Retention cleanup

Blocking or CPU-heavy work can use Tokio's blocking facilities or dedicated worker mechanisms where appropriate.

---

# Deployment

The initial deployment model is intentionally simple:

```text
systemd
   │
   ▼
silo-engine
   │
   ├── HTTP :9255
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

The application port comes from `SILO_ENGINE_PORT`.

Container orchestration and Kubernetes are intentionally out of scope for the initial implementation.

The architecture should, however, avoid assumptions that would prevent future horizontal scaling.

---

# Security Principles

Silo is infrastructure software, so security is a first-class concern.

Important principles include:

1. Never expose PostgreSQL superuser credentials to customers.
2. Keep browser-to-database connections server-side through Silo APIs.
3. Separate public/CDN access from internal service-to-service APIs.
4. Use organization-scoped resource authorization.
5. Keep secrets out of source control.
6. Audit infrastructure and data-management operations.
7. Use TLS for customer-facing database connections.
8. Validate and limit large uploads and long-running operations.
9. Keep unsafe/native FFI boundaries isolated when native libraries are used.
10. Do not expose arbitrary SQL in the initial database explorer.

---

# Design Principles

Silo follows these principles:

1. **Infrastructure should be separated from business logic.**
2. **Storage should be abstracted behind a backend interface.**
3. **Configuration should be loaded once and injected.**
4. **Services should not create their own dependencies.**
5. **Sharpnr Engine owns application/business logic.**
6. **Silo owns infrastructure provisioning and storage.**
7. **Public resource URLs should not depend on physical filesystem layout.**
8. **The API should have an explicit OpenAPI contract.**
9. **Large files should be processed using streaming where possible.**
10. **Audit events should be durable and append-oriented.**
11. **The first implementation should remain simple and self-hosted.**
12. **Distributed infrastructure should be introduced only when it is actually needed.**
13. **Existing open-source infrastructure should be reused rather than reinvented without a strong reason.**
14. **Future Data Lake and Warehouse features should build on the same storage foundation instead of creating unrelated storage systems.**

---

# Current Status

Silo Engine is in the initial architecture and implementation phase.

Current foundations and decisions include:

- Rust/Cargo project
- Rust toolchain management with `rustup`
- Axum/Tokio service direction
- Centralized `EngineConfig`
- Environment-based configuration
- Explicit dependency injection
- Controller → Service → Infrastructure architecture
- OpenAPI documentation direction
- Swagger UI / ReDoc direction
- Storage abstraction design
- PostgreSQL provisioning design
- Organization/employee storage quota model
- UUID utility module with UUID v7 as the preferred default identifier
- Compression subsystem design
- Streaming compression direction
- Audit log design
- Worker architecture direction
- Data Lake and Data Warehouse product direction
- Self-hosted physical deployment
- RAID-backed storage as the initial storage foundation
- Kubernetes intentionally excluded from the initial deployment

---

# Roadmap

The implementation roadmap is intentionally incremental.

```text
Phase 1 — Foundation
    │
    ├── Configuration
    ├── Axum server
    ├── Application state / DI
    ├── Error handling
    └── Logging / tracing
    │
    ▼
Phase 2 — Storage
    │
    ├── Storage backend
    ├── Object metadata
    ├── Upload
    ├── Download
    ├── Delete
    ├── Checksums
    ├── Compression
    └── Organization / employee quotas
    │
    ▼
Phase 3 — Access
    │
    ├── Authentication
    ├── Authorization
    ├── CDN integration
    └── Audit logging
    │
    ▼
Phase 4 — PostgreSQL
    │
    ├── Database provisioning
    ├── PostgreSQL roles
    ├── Credentials
    ├── Public database endpoint
    ├── Database Explorer
    └── Backups / snapshots
    │
    ▼
Phase 5 — Data Lake
    │
    ├── Lake creation
    ├── Dataset management
    ├── Parquet
    ├── Data ingestion
    ├── PostgreSQL → Lake
    └── Retention / lifecycle
    │
    ▼
Phase 6 — Data Warehouse
    │
    ├── Warehouse creation
    ├── Lake → Warehouse
    ├── ETL / ELT
    ├── Query engine
    └── Analytics APIs
    │
    ▼
Phase 7 — Advanced Data Services
    │
    ├── SQL workspace
    ├── Search
    ├── Vector storage
    ├── AI / RAG infrastructure
    └── Advanced analytics
```

Not every roadmap item is committed to the initial release.

---

# Development

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

Check the project:

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

# Name

## Silo Engine

Silo is intended to act as Sharpnr's infrastructure silo: a centralized platform that isolates storage, databases, and future data infrastructure from the business applications consuming those resources.

The long-term goal is simple:

> **Give organizations infrastructure they can create, manage, and consume through Silo without requiring them to manage the physical infrastructure underneath it.**
