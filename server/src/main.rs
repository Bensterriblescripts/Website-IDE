// use axum::{
//     routing::get,
//     Router,
// };
use regex::Regex;
use std::time::Instant;

#[tokio::main]
async fn main() {

    let text = "
    $test = \"Hello\";
    if ($text == \"Hello\") {
        echo 'Hi!!';
    }
    ";

    println!("Issues:");

    let start_time = Instant::now();
    variable_check(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let variable_timer = elapsed_time.as_secs_f64();

    let start_time = Instant::now();
    delimiter_check(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let delimiter_timer = elapsed_time.as_secs_f64();

    let start_time = Instant::now();
    semicolon_check(text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let semicolon_timer = elapsed_time.as_secs_f64();


    println!("\nElapsed times (Milliseconds):\nVariables: {}\nDelimiter: {}\nSemicolon: {}", variable_timer, delimiter_timer, semicolon_timer);

    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    
}
fn variable_check(text: &str) {
    // If it needs to be declared
    let var_reg = Regex::new(r"\$[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    for variable in var_reg.find_iter(text) {
        let call_reg = format!(r"{}\s?=[^=].*", regex::escape(variable.as_str()));
        let call = Regex::new(&call_reg).unwrap();
        if let Some(_capture) = call.find(text) {
            continue;
        }
        println!("{} needs to be declared", variable.as_str())
    }
}

fn delimiter_check(text: &str) {
    // Opened and not closed
    // let delo_reg = Regex::new(r"\{((?s).*?)\}").unwrap();
    // for delimiter_open in delo_reg.find_iter(text) {
    //     println!("Delimiter needs to be closed: {}", delimiter_open.as_str());
    // }
    // // Closed but not open
    // let delc_reg = Regex::new(r"[^{].*}").unwrap();
    // for delimiter_closed in delc_reg.find_iter(text) {
    //     println!("Delimiter has not been opened: {}", delimiter_closed.as_str());
    // }

}

fn semicolon_check(text: &str) {
    let semi_reg = Regex::new(r"=?(?s)[^=;]").unwrap();
    for semicolon in semi_reg.find_iter(text) {
        println!("Needs a semi-colon: {}", semicolon.as_str());
    }
}