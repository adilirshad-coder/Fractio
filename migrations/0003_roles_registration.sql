ALTER TABLE user_roles ADD COLUMN IF NOT EXISTS granted_by UUID REFERENCES users(id);
ALTER TABLE user_roles ADD COLUMN IF NOT EXISTS grant_reason TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS display_name TEXT;
ALTER TABLE founder_entities ADD COLUMN IF NOT EXISTS kyb_status TEXT NOT NULL DEFAULT 'none';
ALTER TABLE founder_entities ADD COLUMN IF NOT EXISTS jurisdiction TEXT;
DO $$ BEGIN
  ALTER TABLE founder_entities ADD CONSTRAINT founder_entities_kyb_status_check
    CHECK (kyb_status IN ('none','pending','approved','rejected'));
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
  ALTER TABLE verifications ADD CONSTRAINT verifications_stage_check
    CHECK (stage IN ('application','documents','final_review'));
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
  ALTER TABLE verifications ADD CONSTRAINT verifications_status_check
    CHECK (status IN ('pending','approved','rejected','needs_information'));
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
  ALTER TABLE verifications ADD CONSTRAINT verifications_subject_type_check
    CHECK (subject_type IN ('project','founder_entity','investor'));
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
CREATE INDEX IF NOT EXISTS projects_created_id_desc_idx ON projects(created_at DESC, id DESC);
DO $$ BEGIN
  ALTER TABLE listings ADD CONSTRAINT listings_status_check
    CHECK (status IN ('draft','submitted','approved','rejected'));
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
