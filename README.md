# verbiste-rust

![Server License](https://img.shields.io/badge/server-GPL%20v2-blue)
![Frontend License](https://img.shields.io/badge/frontend-MIT-green)

A Rust implementation of French verb conjugation.

The `server/` and `data/` folders are a derivative work of [Verbiste](http://gvlsywt.cluster051.hosting.ovh.net/dev/verbiste.html) by Pierre Sarrazin.

This project is vibe coded and is used for personal needs (i.e., learning French). It may not follow all best practices and is primarily intended for personal use.

Note: Earlier versions of this repository used a repository-wide GPL license.
The current licensing model applies per component as described above.

## Licensing

This repository contains multiple independent components that are licensed
separately.

### Components

- **server/**
  - License: GNU General Public License (GPL)
  - Reason: This component uses GPL-licensed Verbiste linguistic data.

- **data/**
  - License: GNU General Public License (GPL)
  - Contains Verbiste data and derived datasets.

- **frontend/**
  - License: MIT
  - A standalone React frontend that does not link against or derive from
    GPL-licensed code or data.

### Notes

- The presence of multiple licenses in this repository does **not** imply that
  all components are licensed under the GPL.
- Licenses apply at the component level, not repository-wide.
- When run as a hosted service, the software is not distributed.

## Building

Server:

```bash
cd server
cargo build --release
```

Frontend:

```bash
cd frontend
pnpm i
pnpm build
```

## Usage

```bash
RUST_LOG=info cargo run
```
