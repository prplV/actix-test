use actix_web::{get, http::StatusCode, post, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use syntax_analizer::{Compiler, Errs};
use std::{include_str, io::Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
struct ResponseType {
    initial_cmd: String,
    r#type: String,
    text: String,
    index: usize,
}

#[derive(Deserialize)]
struct RequestType {
    text: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./static/index.html"))
}

#[post("/synt")]
async fn synt_analize(req_body: web::Json<RequestType>) -> Result<web::Json<ResponseType>> {
    let req = req_body.text.to_owned().replace("\n", " ");

    // HttpResponse::Ok().body("valid cmd")
    // // req = req_body.replace("%0A", " ").replace("text=", "");
    // println!("request - {}", req);
    
    let mut comp = Compiler::new(req.to_owned());
    match comp.proccess() {
        Ok(_) => {
            println!("valid cmd");
            let mut txt = String::new();
            for (i, j) in comp.vars {
                txt = txt + &i + " = " + &j + "\n";
            }
            let obj = ResponseType {
                initial_cmd: comp.str.to_owned(),
                r#type: String::from("Ok"),
                text: txt,
                index: comp.error_id,
            };
            println!("{:?}", web::Json(&obj));
            return Ok(web::Json(obj));
        },
        Err(_er) => {
            println!("error");
            let obj = ResponseType {
                initial_cmd: comp.str,
                r#type: String::from("Error"),
                text: format!("{}", _er.print()),
                index: comp.error_id,
            };
            println!("{:?}", web::Json(&obj));
            return Ok(web::Json(obj));
        },
    }
}

async fn manual_hello(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(index)
            .service(synt_analize)
            // .service(echo)
            .route("/sperm", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}