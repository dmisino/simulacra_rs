use warp::{Filter, Rejection, Reply};

async fn hello() -> Result<impl Reply, Rejection> {
    Ok("Hello, world!")
}

#[tokio::main]
async fn main() {
    let routes = warp::path::end().and_then(hello);
    println!("Server started at http://localhost:3030/");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}