# API

All `/v1` routes require a bearer JWT; liveness/readiness are public. Responses that complete successfully use `Envelope<T>` with a request ID. Errors carry a request ID in the response body and `x-request-id`; callers may provide the header or the API generates a UUID. Roles come from the database, never JWT claims. `POST /v1/register` requires authentication but no role; it accepts investor or founder registrations, while admin self-registration returns 403. Repeating registration is idempotent. `GET /v1/me` returns the caller's roles and identity summaries.

`GET /v1/projects` accepts any authenticated role, exposes only public lifecycle states, defaults to 20 rows, and caps `limit` at 100. Its opaque base64 cursor paginates by `(created_at,id)` descending. Filters map `Explore` to all public states, `New` to `seed_live`, `Bonding` to `bonding_live`, and `Graduated` to `graduated` (product decision pending).

Investor routes: `POST /kyc/lite`, `GET /wallet`, `POST /deposits`, `POST /withdrawals`, `POST /projects/{id}/invest`, and `POST /projects/{id}/sell`. Authenticated project browsing is `GET /projects` with bounded `limit` and `status` (`Explore`, `New`, `Bonding`, `Graduated`). Founder routes: `POST /founders`, `POST /founders/kyb`, `POST /projects`, `PUT /projects/{id}/landing-page`, `POST /projects/{id}/listing`, `POST /projects/{id}/updates`, `GET /founders/creator-fees`, `POST /founders/creator-fees/claim`, and `GET /projects/{id}/milestones`. Role assignments are `investor`, `founder`, or `admin`; production roles are read from `user_roles`.

