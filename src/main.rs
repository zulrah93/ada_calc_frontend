use actix_web::http::StatusCode;
use actix_web::web::{Data, Form};
use actix_web::Result;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Pool {
    json: String,
    verbose: bool,
    graph: bool,
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!(
            include_str!("../static/index.html"),
            format!("<b style='color:red'>{}</b>", "Nothing Calculated Yet")
        )))
}

#[get("/scripts/index.js")]
async fn index_js() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/javascript; charset=utf-8")
        .body(include_str!("../scripts/index.js")))
}

#[get("/vectors/favicon.svg")]
async fn favicon() -> Result<HttpResponse> {
    // Favorite Icon in SVG format
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("image/svg+xml; charset=utf-8")
        .body(include_str!("../vectors/cardano.svg")))
}

#[get("/styles/index.css")]
async fn index_css() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../styles/index.css")))
}

// Spawns a child process -- this function will never return None so it is safe to unwrap
fn execute_ada_calc_backend(
    json: String,
    verbose: bool,
    graph: bool,
    cli_path: String,
) -> Option<String> {
    let mut args = vec![format!("-p {}", json)];
    if verbose {
        args.push(String::from("-v"));
    }
    if graph {
        args.push(String::from("-G"));
    }
    if let Ok(output) = Command::new(cli_path).args(args).output() {
        Some(
            String::from_utf8(output.stdout.into_iter().collect::<Vec<u8>>())
                .unwrap_or(String::from("JSON Invalid!")),
        )
    } else {
        Some(String::from("JSON Invalid!"))
    }
}

async fn calculate(_request: HttpRequest, data: Data<String>, pool: Form<Pool>) -> impl Responder {

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!(
            include_str!("../static/index.html"),
            format!(
                "<b style='color:blue'>{}</b>",
                execute_ada_calc_backend(
                    pool.json.clone(),
                    pool.verbose,
                    pool.graph,
                    data.to_string()
                )
                .unwrap()
            )
        ))
}

#[derive(Debug, Serialize, Deserialize)]
struct WebAppConfig {
    host_or_ip: String,
    port: u16,
    ada_calc_backend_path: String,
}

impl Default for WebAppConfig {
    fn default() -> Self {
        WebAppConfig {
            host_or_ip: String::from("0.0.0.0"),
            port: 80,
            ada_calc_backend_path: String::default(),
        }
    }
}

fn parse_config(json_buffer: &String) -> WebAppConfig {
    serde_json::from_str::<WebAppConfig>(json_buffer).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = if let Ok(buffer) = &read_to_string("config.json") {
        parse_config(buffer)
    } else {
        let default_config = WebAppConfig::default();
        println!(
            "config.json is missing using default configuration {:?}",
            default_config
        );
        default_config
    };
    //TODO: Implement HTTPS with a signed certificate to ensure privacy of information being calculated
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.ada_calc_backend_path.clone())) // Pass backend path
            .service(index_js)
            .service(favicon)
            .service(index_css)
            .service(index)
            .service(web::resource("/calculate").route(web::post().to(calculate)))
    })
    .bind((config.host_or_ip, config.port))?
    .run()
    .await
}
