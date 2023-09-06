use axum::{
    extract::Json,
    response::Html,
    routing::post,
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct GetText {
    line: String
}

use regex::Regex;
use std::time::Instant;


async fn home() -> Html<&'static str> {
    Html("<html><body>Hello <b>Woooooooorld!</b></body></html>")
}

async fn get_text(Json(GetText { line }): Json<GetText>) -> Html<&'static str> {
    Html("<html><body>POST request received!</body></html>")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/testing", post(get_text));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn send_to_purgatory(text: &str) {

    println!("Issues:");
    let total_time = Instant::now();
    
    let start_time = Instant::now();
    let var = variable_check(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let variable_timer = elapsed_time.as_secs_f64();

    let start_time = Instant::now();
    let delimiter = delimiter_check(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let delimiter_timer = elapsed_time.as_secs_f64();

    let start_time = Instant::now();
    let semicolon = semicolon_check(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let semicolon_timer = elapsed_time.as_secs_f64();

    let start_time = Instant::now();
    let condition = condition_check(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let condition_timer = elapsed_time.as_secs_f64();

    let start_time = Instant::now();
    let string = string_highlight(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let string_timer = elapsed_time.as_secs_f64();

    let start_time = Instant::now();
    let function = function_check(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let function_timer = elapsed_time.as_secs_f64();

    println!("\nElapsed times (Milliseconds):\nVariables: {}\nDelimiter: {}\nSemicolon: {}\nConditions: {}\nStrings: {}\nFunctions: {}", variable_timer, delimiter_timer, semicolon_timer, condition_timer, string_timer, function_timer);

    let total_end_time = Instant::now();
    let elapsed_time = total_end_time.duration_since(total_time);
    let the_end_timer = elapsed_time.as_secs_f64();
    println!("Total time taken: {}", the_end_timer);

}

/* ********************/
/* Checks start here! */
/* ********************/

fn variable_check(text: &str) {
    // If it needs to be declared
    let var_reg = Regex::new(r"\$[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    for variable in var_reg.find_iter(text) {
        println!("Found a variable: {}", variable.as_str()); // Highlighting
        let call_reg = format!(r"{}\s?=[^=].*;", regex::escape(variable.as_str()));
        let call = Regex::new(&call_reg).unwrap();
        if let Some(_capture) = call.find(text) {
            continue;
        }
        println!("{} is not set as a variable, declare it somewhere", variable.as_str())
    }
}

fn delimiter_check(text: &str) {
    // Opened and not closed
    // let delo_reg = Regex::new(r"\{((.|\n)*)}$").unwrap();
    // for delimiter_open in delo_reg.find_iter(text) {
    //     println!("Delimiter needs to be closed: {}", delimiter_open.as_str());
    // }
    // // Closed but not open
    // let delc_reg = Regex::new(r"[^{].*}").unwrap();
    // for delimiter_closed in delc_reg.find_iter(text) {
    //     println!("Delimiter has not been opened: {}", delimiter_closed.as_str());
    // }

}

fn condition_check(text: &str) {
    let cond_reg = Regex::new(r"(if|else if|elseif).*\(.*=.*\)").unwrap(); // if statement
    let eq_reg = Regex::new(r"[^=!]=[^=]").unwrap(); // =

    for if_state in cond_reg.find_iter(text) {
        for eq in eq_reg.find_iter(if_state.as_str()) {
            println!("'{}' needs another operand (try != or ==)", eq.as_str());
        }
    }
}

fn semicolon_check(text: &str) {
    // let semivar_reg = Regex::new(r"(?s)\$.*=.*;").unwrap();

    // for semicolon in semivar_reg.find_iter(text) {
    //     println!("Needs a semi-colon: {}", semicolon.as_str());
    // }
}

fn string_highlight(text: &str) {
    // let ministring_reg = Regex::new(r"(?s)'.*?'").unwrap();
    // let biggistring_reg = Regex::new(r#"(?s)".*?""#).unwrap();

    // for stringsmol in ministring_reg.find_iter(text) {
    //     println!("Found a string: {}", stringsmol.as_str());
    // }
    // for stringbig in biggistring_reg.find_iter(text) {
    //     println!("Found a string: {}", stringbig.as_str());
    // }
}

fn function_check(text: &str) {
    let function_reg = Regex::new(r"(?s)[^\$][a-zA-Z_][a-zA-Z0-9_]*\(.*?\)").unwrap();

    for function in function_reg.find_iter(text) {
        println!("Found function: {}", function.as_str()); // Highlighting
    }
}