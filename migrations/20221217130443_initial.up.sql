CREATE TABLE budget (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE item (
    budget_id UUID NOT NULL,
    category TEXT NOT NULL,
    name TEXT NOT NULL,
    amount INT NOT NULL,

    CONSTRAINT fk_budget FOREIGN KEY(budget_id) REFERENCES budget(id)
);