ALTER TABLE investor_identities ADD COLUMN IF NOT EXISTS kyc_level TEXT NOT NULL DEFAULT 'none' CHECK (kyc_level IN ('none','lite','full'));
ALTER TABLE projects ADD COLUMN IF NOT EXISTS token_mint TEXT;
ALTER TABLE projects ADD COLUMN IF NOT EXISTS total_supply NUMERIC(39,0);
ALTER TABLE projects ADD COLUMN IF NOT EXISTS landing_page JSONB NOT NULL DEFAULT '{}'::jsonb;
ALTER TABLE offerings ADD COLUMN IF NOT EXISTS raise_target_minor BIGINT;
ALTER TABLE offerings ADD COLUMN IF NOT EXISTS raise_deadline TIMESTAMPTZ;
ALTER TABLE offerings ADD COLUMN IF NOT EXISTS asset TEXT;
ALTER TABLE payments ADD COLUMN IF NOT EXISTS method TEXT CHECK (method IN ('raast','ach','ewallet','onchain_usdt','onchain_usdc'));

CREATE TABLE IF NOT EXISTS trades (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), project_id UUID NOT NULL REFERENCES projects(id),
  identity_id UUID NOT NULL REFERENCES investor_identities(id), wallet_id UUID,
  side TEXT NOT NULL CHECK (side IN ('buy','sell')), base_amount NUMERIC(39,0) NOT NULL,
  quote_amount NUMERIC(39,0) NOT NULL, price NUMERIC(39,0) NOT NULL,
  chain_signature TEXT UNIQUE, slot BIGINT, created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS trades_project_created_idx ON trades(project_id, created_at DESC);
CREATE INDEX IF NOT EXISTS trades_identity_created_idx ON trades(identity_id, created_at DESC);

CREATE TABLE IF NOT EXISTS fx_rates (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), pair TEXT NOT NULL, rate NUMERIC NOT NULL,
  source TEXT NOT NULL, as_of TIMESTAMPTZ NOT NULL
);
CREATE INDEX IF NOT EXISTS fx_rates_pair_as_of_idx ON fx_rates(pair, as_of DESC);

CREATE TABLE IF NOT EXISTS ledger_entries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), user_id UUID NOT NULL, asset TEXT NOT NULL,
  amount NUMERIC(39,0) NOT NULL, direction TEXT NOT NULL CHECK (direction IN ('credit','debit')),
  ref_type TEXT NOT NULL, ref_id UUID, created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS ledger_user_created_idx ON ledger_entries(user_id, created_at DESC);

CREATE TABLE IF NOT EXISTS withdrawals (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), user_id UUID NOT NULL,
  method TEXT NOT NULL CHECK (method IN ('raast','ach','ewallet','onchain')),
  amount_minor BIGINT NOT NULL CHECK (amount_minor > 0), currency TEXT NOT NULL,
  status TEXT NOT NULL, idempotency_key TEXT NOT NULL UNIQUE, provider_reference TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS withdrawals_user_created_idx ON withdrawals(user_id, created_at DESC);
CREATE TABLE IF NOT EXISTS user_roles (
  user_id UUID NOT NULL REFERENCES users(id), role TEXT NOT NULL CHECK (role IN ('investor','founder','admin')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(), PRIMARY KEY(user_id, role)
);
CREATE INDEX IF NOT EXISTS user_roles_role_idx ON user_roles(role);
