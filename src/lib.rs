#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

pub mod controllers;
pub mod database;
pub mod models;
pub mod routes;
pub mod schema;
