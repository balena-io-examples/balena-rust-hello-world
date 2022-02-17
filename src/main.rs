#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    let port = 80;

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./views/index.html"));

    let public = warp::path("public").and(warp::fs::dir("./views/public"));
    let routes = index.or(public);
    println!("Listenning for requests on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
