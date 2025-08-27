use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use demo::models::{NewComment, UpdateComment};

async fn create_comment(comment: web::Json<NewComment>) -> impl Responder {
    demo::create(comment.into_inner());
    HttpResponse::Ok().body("comment has been saved!")
}

async fn update_comment(comment: web::Json<UpdateComment>) -> impl Responder {
    demo::update(comment.into_inner());
    HttpResponse::Ok().body("comment has been updated!")
}

#[get("/{wiki_entry}")]
async fn get_comments(path: web::Path<String>) -> impl Responder{
    let comments = demo::get(path.into_inner());
    HttpResponse::Ok().body(format!("{:?}", comments))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/")
                //To do: Create a web::get route, for specific id too!;
                .route(web::post().to(create_comment))
                .route(web::put().to(update_comment))
        )
            .service(get_comments)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
    .await
}
