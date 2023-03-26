CREATE TABLE books (
  id TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  author TEXT NOT NULL,
  user_id TEXT NOT NULL,
  publishing_house TEXT,
  release_date TEXT,
  cover_image TEXT,
  creation_date TEXT
);

CREATE TABLE book_relationships (
    id TEXT PRIMARY KEY NOT NULL,
    book_id TEXT NOT NULL,
    related_book_id TEXT NOT NULL,
    relationship_type INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    comment TEXT,
    text_fragments TEXT,
    FOREIGN KEY (book_id) REFERENCES Books(id),
    FOREIGN KEY (related_book_id) REFERENCES Books(id)
);  

CREATE TABLE book_relationship_types (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    icon TEXT NOT NULL
);

CREATE TABLE users (
  id TEXT PRIMARY KEY NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  email TEXT NOT NULL,
  creation_date TEXT NOT NULL,
  auth_token TEXT
);

CREATE TABLE categories (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  parent_category TEXT
);

CREATE TABLE book_notes (
  id TEXT PRIMARY KEY NOT NULL,
  book_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  content TEXT NOT NULL,
  creation_date TEXT NOT NULL,
  header TEXT,
  relationships TEXT,
  FOREIGN KEY (book_id) REFERENCES Books(id)
);

CREATE TABLE text_fragments (
  id TEXT PRIMARY KEY NOT NULL,
  book_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  text TEXT NOT NULL,
  creation_date TEXT NOT NULL,
  FOREIGN KEY (book_id) REFERENCES Books(id)
);