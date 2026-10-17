# bill8-wire-contracts — Business Requirements & Functionality

> Audience: product owners, business analysts, and anyone evaluating what
> the wire conventions **do for the business**. For architecture read
> `README.md`; for wire-level detail read `DESIGN.md`.

## 1. Business Purpose

- External integrators learn **one** API contract, then use every Bill8 surface — payments, billing, webhooks, reporting, and the rest — without re-learning error formats or pagination.
- Support and integration teams triage by **stable error codes**, not by parsing prose.
- Integrations survive platform evolution: the conventions are pinned, tested, and never silently changed.

## 2. Business Scope (What the Service Provides)

| Capability | Business rule |
|---|---|
| Uniform error envelope | Every failed request answers with a predictable `{ error: { code, message } }` body — no surprises per endpoint |
| Stable error-code set | Six machine-readable codes cover every failure class; they never change names |
| Cursor pagination | Every list answers "what's next" with `next_cursor`/`has_more` — no page drift |
| Header conventions | `Idempotency-Key`, version pinning, and correlation echo behave identically everywhere |
| Money-as-string | Amounts are always decimal strings — no floating-point rounding surprises in integrator ledgers |

**What the service does NOT own (business boundary):**

| Out of scope | Owning unit |
|---|---|
| Actual endpoint behavior | Each service's handlers |
| Auth decisions | Shield / the platform policy engine |
| OpenAPI generation per service | Each service's utoipa spec |

## 3. Business Capabilities

| Area | Capability | Business rule |
|---|---|---|
| Errors | Predictable failure contract | A failed integration call is diagnosable from the response alone |
| Lists | Complete, stable enumeration | Paginating a list under load never skips or duplicates rows |
| Idempotency | Safe retries | The same `Idempotency-Key` never double-charges; a differing key on a retried payload is rejected loudly |
| Compatibility | Version pinning | Breaking changes ship under a new version header value — never in place |

## 4. Business Events (Outbound)

None — the crate is a shared contract library, not a running service. It emits no events and consumes none.

## 5. Business Rules & Invariants

1. Every non-2xx body is exactly `{ "error": { "code", "message", "doc_ref"? } }`.
2. `error.code` is always one of: `forbidden`, `not_found`, `invalid_request`, `already_exists`, `idempotency_conflict`, `downstream_unavailable`.
3. Money is always a decimal string, never a JSON number.
4. `Idempotency-Key` and the body's `idempotency_key` are equivalent; a conflict between them is rejected.
5. List responses carry `data`, `next_cursor`, and `has_more`.

## 6. Target-User Roles & Use Cases

| Persona | What they do |
|---|---|
| External integrator | Builds against the contract once; expects identical shapes across all Bill8 endpoints |
| Support engineer | Triages merchant tickets by stable `error.code` |
| Platform architect | Evolves APIs without breaking integrations, by versioning and never renaming codes |

## 7. KPIs / Success Criteria

- Integration time-to-first-call (the contract is learnable from one read).
- Zero "format drift" defect reports across services.
- Zero double-charge incidents attributable to idempotency ambiguity.

## 8. Dependencies & Reliance on Other Services

None — the crate is a leaf library with no runtime dependencies beyond serialization.

## 9. Acceptance Criteria

1. Every service's published spec includes the envelope, pagination, and header conventions.
2. The convention tests assert the exact JSON shapes.
3. The stable code set is documented and exhaustive.

## 10. Open Business Questions

- Per-locale error `message` values — deferred until non-English locales are on the roadmap.

---

**Design attribution:** bill8:contracts and its underlying concepts were designed by Hamid Jahanzad <Hamid@Jahanzad.com>.
