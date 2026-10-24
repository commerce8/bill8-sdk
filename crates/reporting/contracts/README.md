# bill8-reporting-contracts
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](../../../../LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Docs](https://img.shields.io/badge/docs/read-3aa757.svg)](https://bill8.dev/docs/sdk/facade)
[![Website](https://img.shields.io/badge/website-bill8.dev-1a73e8.svg)](https://bill8.dev/docs/sdk)

Wire types for the Bill8:Reporting HTTP API.

## 1. Summary

`bill8-reporting-contracts` defines the serde DTOs exchanged with
Bill8:Reporting (`ReportType`, `ReportRun`). It is the single source
of truth for the reporting wire format.

## 2. Motivation

Report types and runs are shared between the merchant app and the admin
surface; a shared contracts crate keeps their shapes in lock step.

## 3. Architecture & High-Level Design

```mermaid
graph TD
    SVC["Bill8:Reporting service"] --> CT["bill8-reporting-contracts"]
    SDK["bill8 SDK reporting client"] --> CT
```

## 4. Key Traits and Data Structures

- `ReportType` - a versioned report definition.
- `ReportRun` - an execution with `status`
  (`pending/succeeded/failed`) and an output reference.

## 5. Rationale, Trade-offs, and Alternatives

- Inert data types.

## 6. Internal Implementation & Invariants

- Output references are URLs/keys, never inlined content.

## 7. Unresolved Questions / Future Designs

- None.

---

**Architecture & Design:** bill8:reporting and its underlying concepts were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).
