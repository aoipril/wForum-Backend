// The `prisma` module.
// This module contains the Prisma ORM (Object-Relational Mapping) for the application.
// Prisma is an open-source database toolkit that makes it easy to reason about database operations in Rust.

// The `prisma` submodule.
// This submodule contains the generated Prisma client for the application.
// The Prisma client provides a type-safe API for executing database queries.
// It is generated based on the Prisma schema defined in the `schema.prisma` file.
// Not allow warnings because of the generated code contains warnings.
#![allow(warnings)]
pub mod prisma;