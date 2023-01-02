CREATE TABLE budget (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE item (
    budget_id UUID NOT NULL,
    category TEXT NOT NULL,
    name TEXT NOT NULL,
    amount INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    modified TIMESTAMP NOT NULL DEFAULT current_timestamp,

    CONSTRAINT fk_budget FOREIGN KEY(budget_id) REFERENCES budget(id)
);

CREATE OR REPLACE FUNCTION updated_modified_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.modified = current_timestamp;
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_budget_item_modified_timestamp BEFORE UPDATE
ON item FOR EACH ROW EXECUTE PROCEDURE updated_modified_timestamp();
