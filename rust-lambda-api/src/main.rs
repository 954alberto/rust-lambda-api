use env_logger;
use json::{self, JsonValue};
use lambda_web::actix_web::{self, middleware, web, App, Error, HttpResponse, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use regex::RegexBuilder;
use serde_json;
use std::collections::HashMap;

pub fn matching_regex(acronyms_map: (&str, &str), message: String) -> Vec<String> {
    let acronym = acronyms_map.0;
    let long_name = acronyms_map.1;
    let message: &str = &message;

    let acronym_regex= RegexBuilder::new(acronym)
        .case_insensitive(true)
        .build()
        .expect("Invalid Regex");

    let mut matches = Vec::new();
    for i in acronym_regex.find_iter(&message) {
        let test = i.as_str();
        let uno = message.replace(test, long_name).clone();
        matches.push(uno)
    }
    matches
}

async fn index_acronyms(body: web::Bytes) -> Result<HttpResponse, Error> {
    let acronyms_map = HashMap::from([
        ("ESA", "European Space Agency"),
        ("NASA", "National Aeronautics and Space Administration"),
        ("AIAA", "American Institute of Aeronautics and Astronautics"),
        ("CSA", "Canadian Space Agency"),
        ("ASI", "Agenzia Spaziale Italiana"),
        ("RSA", "Russian Space Agency"),
    ]);

    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result

    let resut_json: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };

    log::debug!("Received list: {}", resut_json);
    let key = "err";

    if resut_json.has_key(key) {
        log::error!("{}", resut_json);
    }

    let mut response_vec = Vec::new();
    for items in resut_json.members() {
        let message = items.to_string();
        for bank in acronyms_map.to_owned() {
            let veky = matching_regex(bank, message.to_owned());
            let mut s = Some(String::from_iter(veky));
            if let Some(s) = s.take() {
                if s.len() > 0 {
                    response_vec.push(s);
                }
            }
        }
    }
    let response_json = serde_json::to_string(&response_vec)?;
    let response = HttpResponse::Ok()
        .content_type("application/json")
        .body(response_json);

    Ok(response)
}

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let factory = move || {
        App::new()
            .wrap(middleware::Logger::default()) // enable logger
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/acronyms").route(web::post().to(index_acronyms)))
    };
    if is_running_on_lambda() {
        log::info!("Detected AWS Lambda environment");
        log::info!("Running Actix on Lambda");
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        log::info!("Not in AWS Lambda environment");
        log::info!("Starting HTTP server at http://0.0.0.0:8080");

        // Local server
        HttpServer::new(factory)
            .workers(1)
            .bind("0.0.0.0:8080")?
            .run()
            .await?;
    }
    Ok(())
}
