table! {
    file_attributes (ino) {
        ino -> Binary,
        size -> Binary,
        blocks -> Binary,
        atime -> Binary,
        mtime -> Binary,
        ctime -> Binary,
        crtime -> Binary,
        kind -> Text,
        perm -> Binary,
        nlink -> Binary,
        uid -> Binary,
        gid -> Binary,
        rdev -> Binary,
        flags -> Binary,
    }
}

table! {
    files (parent, name) {
        parent -> Binary,
        name -> Text,
        ino -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(
    file_attributes,
    files,
);
