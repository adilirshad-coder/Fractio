# Curve model

`BondingCurveEngine` exposes buy/sell quotes without coupling the domain to an SDK. Candidate implementations are a custom constant-product curve and a Meteora DBC adapter. No pricing or invariant math is implemented. Future math must use checked integer arithmetic, fixed-point units, explicit slippage bounds, and property/fuzz tests.
