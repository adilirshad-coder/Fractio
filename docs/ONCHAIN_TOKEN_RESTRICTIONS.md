# On-chain token restrictions

Do not use a mint-wide NonTransferable extension as a shortcut to restrict only the Seed allocation. Candidate mechanisms include separate escrowed vesting balances and a transfer hook with identity/position extra accounts. Hook account resolution, beneficial-owner aggregation and compatibility with Meteora/Raydium require an explicit design and test. This scaffold exposes the policy boundary and does not authorize transfers.
