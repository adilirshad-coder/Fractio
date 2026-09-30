# API

Routes are versioned under `/v1`; bearer JWT authentication is required and health routes are public. Responses and error envelopes are being established in the API module. Business endpoints deliberately return NOT_IMPLEMENTED. Future list routes should use bounded cursor pagination and investment/withdrawal intent should require an idempotency key. OpenAPI generation and WebSocket contracts remain pending.
