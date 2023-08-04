-- Create the 'categories' table
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    parent_id INTEGER REFERENCES categories(id) ON DELETE CASCADE
);

-- Create the 'articles' table
CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    heading VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    text TEXT NOT NULL
);

-- Create the 'article_categories' table to handle the many-to-many
-- relationship between articles and categories
CREATE TABLE article_categories (
    article_id INTEGER REFERENCES articles(id) ON DELETE CASCADE,
    category_id INTEGER REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (article_id, category_id)
);

-- Create the 'tags' table
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

-- Create the 'article_tags' table to handle the many-to-many relationship
-- between articles and tags
CREATE TABLE article_tags (
    article_id INTEGER REFERENCES articles(id) ON DELETE CASCADE,
    tag_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (article_id, tag_id)
);

