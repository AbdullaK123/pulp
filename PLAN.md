# Pulp — Project Plan

> Fast, private PDF tools. No accounts, no ads, no bullshit.

**Stack:** Rust/Axum · React/Vite/TypeScript · PostgreSQL · Redis · Stripe · Railway

---

## Phase 1 — Foundation

Get the Rust project compiling, the database running, and a health check endpoint responding.

- [X] Initialize Cargo workspace with two crates: `pulp-api` (binary) and `pulp-core` (library — PDF processing logic)
- [X] Set up Axum with a basic `GET /health` endpoint
- [X] Configure `sqlx` with Postgres and run first migration
- [X] Set up Redis connection (rate limiting + ephemeral data)
- [X] Create database schema:
    - [X] `users` table (id, email, password_hash, auth_provider, tier, created_at, updated_at)
    - [X] `api_keys` table (id, user_id, key_hash, name, last_used_at, revoked_at, created_at)
    - [X] `usage_events` table (id, user_id, api_key_id, operation, file_size_bytes, processing_ms, status, created_at)
    - [X] `subscriptions` table (id, user_id, stripe_customer_id, stripe_subscription_id, status, current_period_start, current_period_end, created_at, updated_at)
- [X] Environment config with `dotenvy` + a typed config struct
- [ ] Error handling scaffold — unified API error type with proper HTTP status codes
- [X] Set up `tracing` + `tracing-subscriber` for structured logging
- [ ] Verify everything compiles, health endpoint responds, DB connects

**Milestone: Axum server running, Postgres connected, clean project structure.**

---

## Phase 2 — Auth

Users need to exist before they can do anything.

- [X] Password hashing with `argon2`
- [ ] `POST /api/v1/auth/register` — email + password, returns user + session
- [ ] `POST /api/v1/auth/login` — email + password, returns session
- [ ] `POST /api/v1/auth/logout` — invalidate session
- [ ] Session management (JWT or server-side sessions in Redis — pick one)
- [ ] Auth middleware — extract user from session, attach to request context
- [ ] OAuth 2.0 flow for Google
- [ ] OAuth 2.0 flow for GitHub
- [ ] `GET /api/v1/auth/me` — return current user profile
- [ ] API key generation: `POST /api/v1/keys`
- [ ] API key listing: `GET /api/v1/keys`
- [ ] API key revocation: `DELETE /api/v1/keys/:id`
- [ ] API key auth middleware — check `X-API-Key` header, resolve user
- [ ] Unified auth middleware that accepts either session cookie OR API key

**Milestone: Users can register, log in, generate API keys, and authenticate requests both ways.**

---

## Phase 3 — Core PDF Engine

This is the heart of Pulp. All processing logic lives in `pulp-core`, completely decoupled from HTTP.

- [ ] Define a clean trait/interface for PDF operations (input bytes → output bytes)
- [ ] **Merge** — combine multiple PDFs into one
- [ ] **Split** — split a PDF into individual pages or page ranges
- [ ] **Compress** — reduce file size (image resampling, stream optimization)
- [ ] **PDF → Images** — render pages to PNG/JPEG
- [ ] **Images → PDF** — combine images into a single PDF
- [ ] **Rotate** — rotate specific pages (90°, 180°, 270°)
- [ ] **Reorder** — rearrange pages in a specified order
- [ ] File validation — reject non-PDFs, malformed files, encrypted files (with clear errors)
- [ ] In-memory processing pipeline — no unnecessary disk writes
- [ ] Encrypted temp file fallback for large files (paid tier)
- [ ] File cleanup — guaranteed destruction after processing, reaper for orphaned files
- [ ] Unit tests for every operation
- [ ] Benchmark suite — track processing time by operation and file size

**Milestone: Every launch operation works in isolation with tests passing. No HTTP, no auth — pure logic.**

---

## Phase 4 — API Layer

Wire the engine to HTTP. Every endpoint follows the same pattern: authenticate → validate → rate limit → process → respond.

- [ ] Multipart file upload handling with size limits (30MB free / unlimited paid)
- [ ] `POST /api/v1/merge` — upload multiple files, return merged PDF
- [ ] `POST /api/v1/split` — upload file + page ranges, return zip of PDFs
- [ ] `POST /api/v1/compress` — upload file, return compressed PDF
- [ ] `POST /api/v1/pdf-to-images` — upload file, return zip of images
- [ ] `POST /api/v1/images-to-pdf` — upload images, return PDF
- [ ] `POST /api/v1/rotate` — upload file + rotation spec, return PDF
- [ ] `POST /api/v1/reorder` — upload file + page order, return PDF
- [ ] Rate limiting middleware (sliding window in Redis)
    - [ ] Free tier: 20 operations/hour
    - [ ] Paid tier: 500 operations/hour
    - [ ] Rate limit headers (`X-RateLimit-Remaining`, `X-RateLimit-Reset`)
- [ ] File size enforcement middleware (check tier before processing)
- [ ] Usage event logging — write to `usage_events` on every operation
- [ ] Proper error responses — 400 for bad input, 401 for unauthed, 413 for file too large, 429 for rate limited
- [ ] Response headers — processing time, file size, operation metadata
- [ ] OpenAPI spec (consider `utoipa` for auto-generation from Axum handlers)

**Milestone: Full API functional. Can process PDFs via curl or any HTTP client with auth and rate limiting.**

---

## Phase 5 — Payments

Stripe integration. Keep it simple — one plan, one price.

- [ ] Stripe account setup + API keys in config
- [ ] `POST /api/v1/billing/checkout` — create Stripe Checkout session, return URL
- [ ] `POST /api/v1/billing/portal` — create Stripe Customer Portal session (manage/cancel)
- [ ] Stripe webhook handler (`POST /api/v1/webhooks/stripe`)
    - [ ] `checkout.session.completed` — activate subscription, update user tier
    - [ ] `invoice.paid` — extend subscription period
    - [ ] `invoice.payment_failed` — flag account
    - [ ] `customer.subscription.deleted` — downgrade to free tier
- [ ] Webhook signature verification
- [ ] Sync user tier from subscription status on every auth check
- [ ] `GET /api/v1/billing/status` — return current plan, usage this period, renewal date
- [ ] Handle edge cases: failed payments, plan changes mid-cycle, refunds

**Milestone: Users can subscribe, pay, manage billing, and get downgraded automatically when they cancel.**

---

## Phase 6 — Frontend

React app that consumes the API. Clean, fast, zero-friction UX.

- [ ] Vite + React + TypeScript project scaffold
- [ ] Routing (TanStack Router or React Router)
- [ ] Auth pages — register, login, OAuth buttons
- [ ] Dashboard — usage stats, current plan, quick access to tools
- [ ] PDF tool interface:
    - [ ] Drag-and-drop file upload zone
    - [ ] Operation selector (merge, split, compress, etc.)
    - [ ] Operation-specific controls (page ranges for split, rotation degrees, reorder UI)
    - [ ] Progress indicator during processing
    - [ ] Download result
- [ ] API key management page — generate, name, copy, revoke
- [ ] Billing page — current plan, upgrade button (Stripe Checkout), manage (Stripe Portal)
- [ ] Responsive design — works on mobile (people merge PDFs from their phones)
- [ ] Error handling — clear messages for every failure state (file too large, rate limited, etc.)

**Milestone: Full working web app. A user can sign up, upload a PDF, process it, download the result, and pay for a subscription.**

---

## Phase 7 — Infrastructure & Deployment

Ship it on Railway.

- [ ] Railway project setup — API service, Postgres, Redis
- [ ] Dockerfile for the Rust API (multi-stage build for small images)
- [ ] Environment variables configured in Railway (DB URL, Redis URL, Stripe keys, OAuth secrets)
- [ ] Custom domain + SSL
- [ ] CI pipeline — run tests and clippy on every push (GitHub Actions)
- [ ] Automated deployment on merge to main
- [ ] Healthcheck endpoint registered with Railway
- [ ] Logging and monitoring — structured logs visible in Railway dashboard
- [ ] Temp file reaper — background task that runs every 5 minutes, nukes anything stale
- [ ] Basic alerting — notify on high error rates or failed deployments

**Milestone: Pulp is live on the internet. Deploys automatically. Monitored.**

---

## Phase 8 — Launch Prep

The last 10% that makes it a real product.

- [ ] Landing page — what it does, pricing, trust story ("your files are never stored"), CTA
- [ ] API documentation — every endpoint, request/response examples, auth guide
- [ ] Terms of service + privacy policy (keep it human-readable)
- [ ] Rate limit / error page for the web UI
- [ ] Open Graph / SEO meta tags (people will share "merge PDF" links)
- [ ] Analytics — basic usage metrics (PostHog or Plausible, not Google Analytics)
- [ ] Load testing — simulate concurrent users, find the breaking point
- [ ] Security review — check file upload handling, auth flows, Stripe webhook verification
- [ ] Soft launch — share with a small group, collect feedback
- [ ] Public launch

**Milestone: Pulp is a product. People can find it, use it, pay for it, and trust it.**

---

## Post-Launch Roadmap (Future Phases)

- [ ] OCR (scanned PDF → searchable PDF)
- [ ] Watermarks (add/remove)
- [ ] Encrypt/decrypt PDFs
- [ ] Extract text, images, tables
- [ ] HTML → PDF conversion
- [ ] DOCX → PDF conversion
- [ ] PDF comparison / visual diff
- [ ] Form filling + flattening
- [ ] Batch processing (paid feature)
- [ ] API usage dashboard with graphs
- [ ] Team accounts / multi-seat plans
- [ ] Metadata editing
- [ ] Header/footer/page number insertion

---

*Built with Rust. Deployed on Railway. No AI. No bullshit. Just PDFs.*