#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./views/index.html"));

    let public = warp::path("public").and(warp::fs::dir("./views/public"));
    let routes = index.or(public);
    warp::serve(routes).run(([127, 0, 0, 1], 80)).await;
}
