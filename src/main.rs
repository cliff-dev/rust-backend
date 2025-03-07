use actix_web::{web, App, HttpResponse, HttpServer, Responder, ResponseError};

async fn hello() -> impl Responder {
    "Hello, world!"
}

//get all post
async fn get_posts() -> impl Responder {
    HttpResponse::Ok()
}

//get user posts
async fn get_posts_by_id() -> impl Responder {
    HttpResponse::Ok()
}

//get user post by id
async fn get_post_by_id() -> impl Responder {
    HttpResponse::Ok()
}

//create user post id
async fn create_post() -> impl Responder {
    HttpResponse::Ok()
}

//delete user post id
async fn delete_post() -> impl Responder {
    HttpResponse::Ok()
}

//update user post id
async fn delete_post_by_id() -> impl Responder {
    HttpResponse::Ok()
}

//update user post by id
async fn edit_post() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create and run the HTTP server
    HttpServer::new(|| {
        App::new()
            // Register a GET route at the root path
            .route("/", web::get().to(hello))
    })
        // Bind the server to localhost on port 8080
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
