#!/usr/bin/env just --justfile

dbpush:
    cd prisma && cargo run db push

generate:
    cd prisma && cargo run generate

updb:
    docker compose up -d

downdb:
    docker compose down

watch:
    cargo watch -x run

run:
    cargo run

watchall:
    docker compose up -d &
    cargo watch -x run

runall:
    docker compose up -d &
    cargo run

