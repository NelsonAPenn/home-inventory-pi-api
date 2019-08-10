use actix_web::{web, App, HttpResponse, HttpServer, Responder};

extern crate mongodb;

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
// use std::cell::RefCell;
// 
// thread_local!(static db:Client);

fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}

fn main() {
  // Direct connection to a server. Will not look for other servers in the topology.
  let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize client.");
  // db = client;


  HttpServer::new(|| {
      App::new()
      .route("/", web::post().to(index))
      .route("/again", web::post().to(index2))
      })
  .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
