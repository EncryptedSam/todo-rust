-- Your SQL goes here
CREATE TABLE todo_list (
    id SERIAL PRIMARY KEY,
    group_id INT REFERENCES todo_groups(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    task_priority INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_group_id FOREIGN KEY (group_id) REFERENCES todo_groups(id)
);
