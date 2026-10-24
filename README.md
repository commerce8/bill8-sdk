# bill8-sdk - Bill8 Namespace Facade
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](../LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Docs](https://img.shields.io/badge/docs/read-3aa757.svg)](https://bill8.dev/docs/services/sdk)
[![Website](https://img.shields.io/badge/website-bill8.dev-1a73e8.svg)](https://bill8.dev/docs/sdk)

## 1. Summary

The Bill8 vertical's SDK workspace: one cargo workspace holding the
vertical's nine contracts crates and the `bill8` facade crate.
Imports go through the facade - `use bill8::ledger::contracts::…`
(namespace rule, architecture-principles §2.1).

## 2. Crates

| Crate | Purpose |
|---|---|
| `facade` (package `bill8`) | The namespace facade |
| `crates/billing/contracts` | Invoice/billing wire types |
| `crates/checkout/contracts` | Checkout session types |
| `crates/ledger/contracts` | Balance transactions, transfers, payouts |
| `crates/merchants/contracts` | Merchant registry types |
| `crates/payments/contracts` | Payment method + charge types |
| `crates/reporting/contracts` | Report wire types |
| `crates/risk/contracts` | Risk-scoring types |
| `crates/webhooks/contracts` | Webhook endpoint types |
| `crates/wire/contracts` | Bank-transfer wire types |

## 3. Consumption

```toml
bill8 = { git = "https://github.com/commerce8/bill8-sdk", tag = "bill8-v0.1.0" }
```

The facade pins the platform SDK (`sentin8` - shared plane only,
SDD §2.2); consumers inherit it transitively.

---

**Architecture & Design:** bill8:sdk and its underlying concepts were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).
