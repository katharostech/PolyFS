table! {
    file_attributes (ino) {
        ino -> Binary,
        attributes -> Binary,
    }
}

table! {
    links (name, parent_ino) {
        name -> Text,
        parent_ino -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(
    file_attributes,
    links,
);
