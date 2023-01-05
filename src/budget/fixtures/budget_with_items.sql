INSERT INTO budget (id, user_id, title)
VALUES ('b8d6ff4e-c12f-416b-a611-8ad0c90669fe', 'Alice', 'My budget with items');

INSERT INTO item (budget_id, category, name, amount)
VALUES
    ('b8d6ff4e-c12f-416b-a611-8ad0c90669fe', 'Income', 'Paycheck', 100),
    ('b8d6ff4e-c12f-416b-a611-8ad0c90669fe', 'Home', 'Rent', 50),
    ('b8d6ff4e-c12f-416b-a611-8ad0c90669fe', 'Food', 'Restaurants', 10)
;