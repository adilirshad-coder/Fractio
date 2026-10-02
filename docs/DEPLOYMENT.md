# Deployment

Use environment-specific configuration and secret manager injection. Local defaults are development-only. Deploy PostgreSQL migrations as a controlled job. Production requires non-root minimal containers, health checks, TLS, managed Redis/PostgreSQL, RPC failover, KMS signer, backups, alerting, and separate multisig-controlled program upgrade authority. These deployment controls are not all implemented in this scaffold.

The local `target/deploy/fractio-keypair.json` is a deployment secret and must never be committed. For devnet or mainnet, generate a fresh program keypair for that deployment, run `anchor keys sync`, and run `scripts/check-program-id.sh` before building or deploying. Do not reuse a local deployment keypair for a public cluster.
