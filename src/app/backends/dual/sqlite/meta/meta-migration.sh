#!/bin/sh
export DATABASE_URL=diesel.sqlite3
diesel migration --migration-dir meta-migrations $@
diesel print-schema > meta_schema.rs
