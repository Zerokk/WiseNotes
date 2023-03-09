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
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
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