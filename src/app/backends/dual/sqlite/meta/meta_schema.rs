table! {
    inodes (id) {
        id -> Nullable<Integer>,
        size -> Nullable<Integer>,
        blocks -> Nullable<Integer>,
        atime -> Nullable<Integer>,
        mtime -> Nullable<Integer>,
        ctime -> Nullable<Integer>,
        crtime -> Nullable<Integer>,
        kind -> Nullable<Text>,
        perm -> Nullable<Integer>,
        nlink -> Nullable<Integer>,
        uid -> Nullable<Integer>,
        gid -> Nullable<Integer>,
        rdev -> Nullable<Integer>,
        flags -> Nullable<Integer>,
    }
}
