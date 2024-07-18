#!/usr/bin/env just --justfile

push:
    cargo prisma db push

generate:
    cargo prisma generate

run:
    cargo run

watch:
    cargo watch -x run


