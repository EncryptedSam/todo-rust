-- Your SQL goes here
CREATE TABLE todo_groups (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    title VARCHAR(255),
    description TEXT,
    group_priority INT DEFAULT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- id, user_id, title, description, group_priority, created_at, updated_at 