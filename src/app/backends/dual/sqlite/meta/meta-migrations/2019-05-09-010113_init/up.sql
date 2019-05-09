create table inodes (
    id integer primary key,
    size integer,
    blocks integer,
    atime integer,
    mtime integer,
    ctime integer,
    crtime integer,
    kind text,
    perm integer,
    nlink integer,
    uid integer,
    gid integer,
    rdev integer,
    flags integer
)
