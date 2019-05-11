create table file_attributes (
    ino blob primary key not null,
    attributes blob not null
);

create table links (
    name text not null,
    parent_ino blob not null,
    primary key (name, parent_ino)
);
