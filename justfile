#!/usr/bin/env just --justfile

dbpush:
    cd prisma && cargo run db push

generate:
    cd prisma && cargo run generate

watch:
    cargo watch -x run

run:
    cargo run
