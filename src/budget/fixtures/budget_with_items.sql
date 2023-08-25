INSERT INTO budget (id, user_id, title)
VALUES ('b8d6ff4e-c12f-416b-a611-8ad0c90669fe', 'Alice', 'My budget with items');

INSERT INTO item (id, budget_id, category, name, amount)
VALUES
    ('5e666f18-de95-4513-abd8-1f09ed5ff98f', 'b8d6ff4e-c12f-416b-a611-8ad0c90669fe', 'Income', 'Paycheck', 100),
    ('c4af1e7a-4dfd-4338-ad31-caee4848a69b', 'b8d6ff4e-c12f-416b-a611-8ad0c90669fe', 'Home', 'Rent', 50),
    ('d831821b-1b50-41fc-a01e-19a1243c334a', 'b8d6ff4e-c12f-416b-a611-8ad0c90669fe', 'Food', 'Restaurants', 10)
;
