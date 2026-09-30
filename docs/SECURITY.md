# Security

No production signing keys in source. Production signer is a KMS/HSM/custodial adapter; segregate treasury, admin, emergency and upgrade authority, preferably under multisig/timelock. The current JWT middleware is a local HS256 boundary and needs issuer/audience/key rotation, role policy and secure secret management before production. Add rate limiting, webhook signatures/replay protection, privileged audit review and dependency scanning before launch.
