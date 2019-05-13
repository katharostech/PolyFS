#!/bin/sh
export DATABASE_URL=diesel.sqlite3
diesel migration --migration-dir kv-migrations $@
diesel print-schema > kv_schema.rs
