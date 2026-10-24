# bill8-merchants-contracts
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](../../../../LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Docs](https://img.shields.io/badge/docs/read-3aa757.svg)](https://bill8.dev/docs/sdk/facade)
[![Website](https://img.shields.io/badge/website-bill8.dev-1a73e8.svg)](https://bill8.dev/docs/sdk)

Wire types for the Bill8:Merchants HTTP API.

## 1. Summary

`bill8-merchants-contracts` defines the serde DTOs exchanged with
Bill8:Merchants (`Account`, `Person`, `ExternalAccount`,
`AccountLink`, `LoginLink`). It is the single source of truth for the
merchants wire format.

## 2. Motivation

Connected-account onboarding spans the merchant app, the portal, and the
admin surface; a shared contracts crate keeps their wire shapes in lock
step.

## 3. Architecture & High-Level Design

```mermaid
graph TD
    SVC["Bill8:Merchants service"] --> CT["bill8-merchants-contracts"]
    SDK["bill8 SDK merchants client"] --> CT
```

## 4. Key Traits and Data Structures

- `Account` - a connected account with `verification_status`
  (`unverified/pending/verified/rejected`).
- `Person` - an individual associated with an account.
- `ExternalAccount` - a payout destination (secret-by-reference fields).

## 5. Rationale, Trade-offs, and Alternatives

- Inert data types; string-typed money (see the payments contracts
  rationale).

## 6. Internal Implementation & Invariants

- Verification status uses the canonical enum spellings.

## 7. Unresolved Questions / Future Designs

- None.

---

**Architecture & Design:** bill8:merchants and its underlying concepts were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).
