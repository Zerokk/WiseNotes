// @generated automatically by Diesel CLI.

diesel::table! {
    book_notes (id) {
        id -> Text,
        book_id -> Text,
        user_id -> Text,
        content -> Text,
        creation_date -> Text,
        header -> Nullable<Text>,
        relationships -> Nullable<Text>,
    }
}

diesel::table! {
    book_relationship_types (id) {
        id -> Text,
        name -> Text,
        description -> Text,
        icon -> Text,
    }
}

diesel::table! {
    book_relationships (id) {
        id -> Text,
        book_id -> Text,
        related_book_id -> Text,
        relationship_type -> Integer,
        user_id -> Text,
        comment -> Nullable<Text>,
        text_fragments -> Nullable<Text>,
    }
}

diesel::table! {
    books (id) {
        id -> Text,
        title -> Text,
        author -> Text,
        user_id -> Text,
        publishing_house -> Nullable<Text>,
        release_date -> Nullable<Text>,
        cover_image -> Nullable<Text>,
        creation_date -> Nullable<Text>,
    }
}

diesel::table! {
    categories (id) {
        id -> Text,
        name -> Text,
        parent_category -> Nullable<Text>,
    }
}

diesel::table! {
    text_fragments (id) {
        id -> Text,
        book_id -> Text,
        user_id -> Text,
        text -> Text,
        creation_date -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        password -> Text,
        email -> Text,
        creation_date -> Text,
        auth_token -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    book_notes,
    book_relationship_types,
    book_relationships,
    books,
    categories,
    text_fragments,
    users,
);
