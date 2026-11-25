# Silo-Engine

So, what is Silo or Silo-Engine?

Silo is Sharpnr's self-hosted storage and data infrastructure platform. It started as a storage solution and is growing into the layer that every Sharpnr application stands on: the place where files live, where databases get created, and where an organization's data is kept, moved, and analysed.

An organization should be able to log into Silo and say "give me 100 GB of storage", "give me a PostgreSQL database", or "give me a data lake", and get it without knowing which server, disk, or RAID array is underneath.

## Why it exists

Every business application eventually needs the same things: somewhere to put files, a database, backups, and a way to work with its data at scale. If each Sharpnr application solved those on its own, the result would be duplicated code, inconsistent security, and storage scattered across machines.

Silo solves them once, in one service, so the applications can focus on their own work.

## The boundary

> **Sharpnr Engine owns business logic. Silo Engine owns infrastructure and data resources.**

Silo does not know what a hotel booking, an invoice, or a payslip is. It knows about objects, quotas, databases, roles, datasets, and audit events. Hospitality, Accounting, HR, CRM, and the other Sharpnr modules consume what Silo provides; they do not live inside it.

## What Silo provides

| Area | What it means |
| --- | --- |
| **Storage** | File and object storage on RAID-backed disks, served through stable CDN URLs that never expose the physical layout. Organization and employee quotas, checksums, and streaming compression. |
| **Databases** | PostgreSQL databases provisioned on demand, with organization-scoped roles and credentials, and a web-based explorer for schemas, tables, and rows. |
| **Data** | Data lakes for raw and semi-structured data (Parquet first), data warehouses for analytics, and the pipelines that move data between them. |
| **Operations** | Backups, snapshots, audit logging, and background workers for anything too large or slow to finish within a request. |

Storage comes first. Databases, data lakes, and warehouses come in that order, each built on the same storage foundation rather than as a separate system.

## How it is built

- **Rust**, with **Axum** and **Tokio**, as a single standalone service
- **OpenAPI** as the contract with consuming services, browsable through Swagger UI and ReDoc
- **Self-hosted** on Sharpnr's own physical machines, run with systemd behind Nginx, with no Kubernetes
- **Simple first**: mature open-source tools are reused, and distributed infrastructure is added only when the workload needs it

## Where it stands today

Silo is at the foundation stage. Running today:

- Centralized configuration loaded once at startup from `APP_ENV`-selected env files
- The Axum server with structured request logging
- Engine endpoints: `/v1/health`, `/v1/health/ready`, `/v1/version`, `/v1/status`
- API documentation at `/swagger-ui`, `/redoc`, and `/v1/openapi.json`

Written but not yet wired into the service: the Zstd compression subsystem and the UUID v7 helpers.

Next: storage — the backend abstraction, upload, download, metadata, and quotas.

## The name

A silo holds something valuable, keeps it separate, and keeps it safe. Silo does the same for Sharpnr: it keeps storage, databases, and data infrastructure apart from the applications that use them, behind one platform that owns them properly.

---

For the full architecture, roadmap, and design decisions, see the [README](../README.md).
