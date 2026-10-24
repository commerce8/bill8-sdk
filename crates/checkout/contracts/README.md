# bill8-checkout-contracts
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](../../../../LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Docs](https://img.shields.io/badge/docs/read-3aa757.svg)](https://bill8.dev/docs/sdk/facade)
[![Website](https://img.shields.io/badge/website-bill8.dev-1a73e8.svg)](https://bill8.dev/docs/sdk)

Wire types for the Bill8:Checkout HTTP API.

## 1. Summary

`bill8-checkout-contracts` defines the serde DTOs exchanged with
Bill8:Checkout (`Session`, `LineItem`, `PaymentLink`). It is the
single source of truth for the checkout wire format.

## 2. Motivation

Sessions and links are shared between the merchant app, the checkout app,
and the checkout service; a shared contracts crate keeps them in lock
step.

## 3. Architecture & High-Level Design

```mermaid
graph TD
    SVC["Bill8:Checkout service"] --> CT["bill8-checkout-contracts"]
    SDK["bill8 SDK checkout client"] --> CT
```

## 4. Key Traits and Data Structures

- `Session` - a checkout session with `mode`
  (`payment/subscription/setup`), success/cancel URLs, and `status`
  (`open/complete/expired`).
- `LineItem` - an item referencing a price or inline price data.
- `PaymentLink` - a reusable link to a session.

## 5. Rationale, Trade-offs, and Alternatives

- Inert data types; string-typed money.

## 6. Internal Implementation & Invariants

- Money fields are JSON strings.

## 7. Unresolved Questions / Future Designs

- None.

---

**Architecture & Design:** bill8:checkout and its underlying concepts were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).
