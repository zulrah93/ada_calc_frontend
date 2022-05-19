use actix_web::http::StatusCode;
use actix_web::web::{Data, Form};
use actix_web::Result;
use actix_web::{web, get, App, HttpRequest,  HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::process::Command;

#[derive(Debug,Deserialize)]
struct Pool {
    json: String,
    verbose : String,
    graph : String,
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!(
            include_str!("../static/index.html"),
            format!("<b>{}</b>", "Nothing Calculated Yet")
        )))
}

#[get("/scripts/index.js")]
async fn js() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/javascript; charset=utf-8")
        .body(include_str!("../scripts/index.js")
        ))
}

// Spawns a child process -- this function will never return None so it is safe to unwrap
// TODO: Implement verbose and graph flags
fn execute_ada_calc_backend(json: String, _verbose : bool, _graph : bool, cli_path: String) -> Option<String> {
    if let Ok(output) = Command::new(cli_path)
        .args(["-p", json.as_str()])
        .args(["-v"]) // Verbose output
        .args(["-G"]) // Generate CSV
        .output()
    {
        Some(
            String::from_utf8(output.stdout.into_iter().collect::<Vec<u8>>())
                .unwrap_or(String::from("JSON Invalid!")),
        )
    } else {
        Some(String::from("JSON Invalid!"))
    }
}

async fn calculate(_request: HttpRequest, data: Data<String>, form: Form<Pool>) -> impl Responder {
    println!("Graph On: {} Verbose On: {}", form.graph, form.verbose);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(format!(
            include_str!("../static/index.html"),
            format!(
                "<b>{}</b>",
                execute_ada_calc_backend(form.json.clone(), true, true, data.to_string()).unwrap()
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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.ada_calc_backend_path.clone())) // Pass backend path
            .service(js)
            .service(index)
            .service(web::resource("/calculate").route(web::post().to(calculate)))
            
    })
    .bind((config.host_or_ip, config.port))?
    .run()
    .await
}
