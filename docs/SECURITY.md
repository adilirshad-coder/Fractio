# Security

No production signing keys in source. Production signer is a KMS/HSM/custodial adapter; segregate treasury, admin, emergency and upgrade authority, preferably under multisig/timelock. The current JWT middleware is a local HS256 boundary and needs issuer/audience/key rotation, role policy and secure secret management before production. Add rate limiting, webhook signatures/replay protection, privileged audit review and dependency scanning before launch.
# Authentication and roles

JWTs authenticate a subject only. Authorization roles are read from `user_roles`; claims supplied by a token do not grant permissions. Admin roles are provisioned through the `fractio-backend grant-role` CLI only and cannot be self-registered. The development HS256 token command is disabled in production. A mock KYC provider must not be enabled in production.
