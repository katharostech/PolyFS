create table file_attributes (
    ino blob primary key not null,
    size blob not null,
    blocks blob not null,
    atime blob not null,
    mtime blob not null,
    ctime blob not null,
    crtime blob not null,
    kind text not null,
    perm blob not null,
    nlink blob not null,
    uid blob not null,
    gid blob not null,
    rdev blob not null,
    flags blob not null
);

insert into file_attributes values (
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    "Directory",
    493, -- 755
    1,
    1001,
    1001,
    0,
    0
);

create table files (
    parent blob not null,
    name text not null,
    ino blob not null,
    primary key (parent, name)
);
