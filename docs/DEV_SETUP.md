# Local development

1. Copy `.env.example` to `.env` and set a development-only JWT secret.
2. Start dependencies: `docker compose up -d postgres redis`.
3. Apply migrations: `cargo run -p fractio-backend -- migrate`.
4. Provision a first administrator: `cargo run -p fractio-backend -- grant-role --subject admin-did --role admin --reason "local development"`.
5. Create development tokens with `cargo run -p fractio-backend -- dev-token --sub investor-did`, `--sub founder-did`, or `--sub admin-did`. Grant investor/founder roles with `grant-role` first when needed.
6. Register an investor: `curl -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" -d '{"role":"investor","display_name":"Investor"}' http://localhost:8080/v1/register`.

The full KYC, project editing, listing review, and admin review workflow is not yet implemented by the backend handlers. KYC and the remaining investment endpoints return `NOT_IMPLEMENTED`; no funds or identity documents should be submitted.
