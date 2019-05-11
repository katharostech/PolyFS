# Notes

> **WARNING:** These are just miscilaneous notes made during development and may not be very organized or complete in regards to the information that provide.

## Filesystem API Query Parameters And Serialization

### Attributes used to query the filesystem

These are the different attributes that are used in the filesystem API to query or otherwise indicate files, locks, etc. that need to be operated on. In other words these are the values that we need to be able to query on in the database.

The current plan is to create tables in the database that expose these values for querying, and using Blobs to store the actual information about the queryable entities by binary serializing the Rust datatypes.

- ino
- parent ino: newparent is also used when renaming and linking, but that doesn't really make a difference because we just access it by the inode like any other file.
- file name
- file handle: file handle is included in request when interacting with file contents such as writing and reading
- lock owner: lock owner is used in `flush` as well as lock functions

### Tables

The database tables needed to represent filesystem metadata. Note that that `ino`'s must be stored in `blob`s because they require 64 bits of storage and the Sqlite `int` type stores a max of `8` bytes.

#### `file_attributes`

This table maps the inodes to their attributes.

| Column Name | Data Type                   |
| ----------- | --------------------------- |
| ino         | `blob primary key not null` |
| attributes  | `blob not null`             |

#### `links`

This table maps the inodes to the filesystem tree.

| Column Name | Data Type                     |
| ----------- | ----------------------------- |
| name        | `string primary key not null` |
| parent_ino  | `blob primary key not null`   |
| ino         | `blob not null`               |

### Key-Value Mapping

Another idea is to skip using database tables and use only the Key-Value store for metadata. For every key we allocate a specific number of bytes that may be used to store a *prefix* for the key that is used to group keys like database tables group their data.

For now, lets assume that the prefix is 32 bytes so that we can fit 32 ASCII characters into the prefix. It may be more efficient to map friendly names to a single byte prefix in code so that we don't waste the storage for the keys. The table name headings will indicate the prefix and all keys are assumed to have the prefix.

#### `file_attributes`

Map inode to its attributes.

| Key   | Value                      |
| ----- | -------------------------- |
| inode | serialized file attributes |

#### `files`

Map (parent inode, file name) pairs to the inode that it references.

| Key                         | Value |
| --------------------------- | ----- |
| ( parent inode, file name ) | inode |

#### `inode_children`

> **Note:** This table would be an optmization designed to speed up directory indexes by preventing the need to scan all of the keys in the `files` table for keys that start with the target `parent inode`.'
>
> This table therefore may not be necessary, and would represent the potential for the different tables to become inconsistent with eachother. For example, creating a file involves updating both the `file_attributes` table and the `inode_children` tables. If `file_attributes` is updated, the `files` and `inode_children` tables would have to be updated as well, but I don't think there is a way to avoid that anyway. There will always be the fact that multiple tables must be updated for certain actions that cannot be done atomically. These should actually be able to be done in the same transaction to make the whole transaction valid, fixing the issue.

Map an inode ( directory ) to the list of children as (file name, inode) pairs.

| Key             | Value                                        |
| --------------- | -------------------------------------------- |
| inode ( `u64` ) | serialized vector of (inode, filename) pairs |

## Filesystem API

These are the callbacks of the filesystem API that must be implemented, documented fully [here](https://docs.rs/fuse/0.3.1/fuse/trait.Filesystem.html).

### `lookup()`

Gets the attributes of a file in a directory by filename.

#### Query

- parent inode
- filename

#### Returns

The file attributes.

#### Strategy

Query the `files` table with the given parent inode and filename to get the `ino` of the target file and use that `ino` to query the `file_attributes` table for the attributes.

### `getattr()`

#### Query

- ino

#### Returns

File attributes.

#### Strategy

Query `file_attributes` table to get the attributes.

### `setattr()`

#### Query

- ino

#### Returns

The new file attrs.

#### Strategy

1. Query the `file_attributes` table for the current file attributes
2. Update the attributes with the inputs to the callback
3. Push the new file attributes to the table

### `mknod()`

#### Query

- parent inode
- filename

#### Returns

The new files attributes.

#### Strategy

> **TODO:** Figure out how to determine available ino for new files.

1. Get an `ino` for the new file that isn't used by anotherr file
2. Instantiate a new FileAttr struct
3. Store the new file attrs in the `file_attributes` table
4. Add a new entry to the `files` table with ( parent inode, filename ) as the key and the new file ino as the value
5. Get the `inode_children` entry for the parent inode and append the new file's (inode, filename) pair to its list
6. Return the file's attributes to the callback

### `mkdir()`

Same as `mknode()` above.

### `unlink()`

#### Query

- parent ino
- filename

#### Returns

N/A

#### Strategy

1. Remove the record from the `files` table with the key ( parent inode, filename )
2. Update the `inode_children` record for its parent ino to remove this files ino from the list
3. Remove the `file_attributes` record for the file
4. Return the callback

### `rmdir()`

The same as `unlink()`.

### `symlink()`

Probably the same as `mknode()`.

> **TODO:** I don't know how exactly to store the symlink path, but that will have to be persisted when a symlink is creatd.

### `rename()`

#### Query

- parent ino
- filename

#### Returns

N/A

#### Strategy

1. Get the inode of the file from the `files` table using the ( parent inode, filename ) key
2. Delete the ( parent inode, filename ) entry in `files`
3. Create a new ( parent inode, filename ) = inode record in the `files` table

### `link()`

#### Query

- ino
- newparent ino
- newname

#### Returns

The file attributes.

#### Strategy

1. Create a new record in the `files` table with the (newparent, newname) as the key and the `ino` as the value.
2. Get the file attributes from the `file_attributes` table and increment the `nlink` property
3. Get the record for the newparent ino in the `inode_children` table and append `ino` to the list
4. Push the updated file attributes to the `file_atributes` table

### `readdir()`

List the items in a directory.

#### Query

- ino
- fh -- Don't do anything with this until we implement `readdir()`
- offset

#### Returns

List of inode entries in the directory.

#### Strategy

1. Get the entry for the given `ino` from the `inode_children` table.
2. Starting at element number `offset` in the list return the (inode, filename) pairs for each file in the list until the buffer is full. The offset value for each item in the buffer is the index of the item in the child list

### ``

#### Query

#### Returns

#### Strategy

### Other Callbacks

#### Pending

Callbacks that we don't know how we are going to implement yet:

- `readlink()` -- I don't know what the expected `data` for a symlink is.
- `open()`
- `read()`
- `write()`
- `flush()`
- `release()`
- `fsync()`
- `opendir()`
- `releasedir()`
- `fsyncdir()`
- `statfs()` -- Don't care to report on FS stats just yet
- `gstxattr()`
- `getxattr()`
- `listxattr()`
- `removeattr()`
- `access()`
- `create()` -- Opens file and we don't know how to do that yet
- `getlk()` -- Skip locking for now
- `setlk()`

#### Ignored Callbacks

Callbacks that we do not plan on implementing:

- `forget()` -- Only useful for items with limited lifetimes
- `bmap()` -- Only for block device backed filesystems
