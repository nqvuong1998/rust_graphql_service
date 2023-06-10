CREATE TABLE "users" (
  id SERIAL PRIMARY KEY,
  username VARCHAR(255) unique not null,
  display_name VARCHAR(255),
  about_me TEXT,
  description TEXT,
  avatar VARCHAR(255)
);

CREATE TABLE "posts" (
  id SERIAL PRIMARY KEY,
  author INTEGER not NULL REFERENCES "users"(id),
  blocks text,
  created_at TIMESTAMPTZ default CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ default CURRENT_TIMESTAMP,
  status VARCHAR(20) not null CHECK (status IN ('PUBLISHED', 'DELETED', 'DRAFT'))
);

CREATE  FUNCTION update_updated_at_posts()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_posts_updated_at
    BEFORE UPDATE
    ON
        posts
    FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_posts();