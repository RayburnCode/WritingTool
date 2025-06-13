-- Corrected migration
CREATE TABLE maintenance_windows (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  starts_at TIMESTAMPTZ NOT NULL,
  ends_at TIMESTAMPTZ NOT NULL,
  description TEXT NOT NULL,
  created_by UUID REFERENCES users(id),
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add status column (not generated)
ALTER TABLE maintenance_windows ADD COLUMN is_active BOOLEAN DEFAULT false;

-- Add trigger (from Solution 1)
CREATE OR REPLACE FUNCTION update_maintenance_status()
RETURNS TRIGGER AS $$
BEGIN
    NEW.is_active := (NEW.starts_at <= NOW() AND NEW.ends_at >= NOW());
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER maintenance_window_status
BEFORE INSERT OR UPDATE ON maintenance_windows
FOR EACH ROW EXECUTE FUNCTION update_maintenance_status();

-- Add session maintenance flag
ALTER TABLE sessions ADD COLUMN is_maintenance BOOLEAN DEFAULT false;