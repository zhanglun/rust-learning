#[macro_use]
extern crate diesel;

extern crate dotenv;

mod db;
use db::*;

#[macro_use]
extern crate serde_derive;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    let posts = get_posts();
    HttpResponse::Ok().json(posts)
}