# Deployment

Use environment-specific configuration and secret manager injection. Local defaults are development-only. Deploy PostgreSQL migrations as a controlled job. Production requires non-root minimal containers, health checks, TLS, managed Redis/PostgreSQL, RPC failover, KMS signer, backups, alerting, and separate multisig-controlled program upgrade authority. These deployment controls are not all implemented in this scaffold.
