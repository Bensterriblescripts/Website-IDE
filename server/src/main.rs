// use axum::{
//     routing::get,
//     Router,
// };
use regex::Regex;
use std::time::Instant;

#[tokio::main]
async fn main() {

    // Benchmark
    let start_time = Instant::now();

    let text = "
    $test = \"Hello\";
    if ($text == \"Hello\") {
        echo 'Hi!!';
    }
    ";
    check_variable_exists(text);

    // Benchmark
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let elapsed_milliseconds = elapsed_time.as_secs_f64();
    println!("Elapsed time: {} seconds", elapsed_milliseconds);

    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    
}
fn check_variable_exists(text: &str) {
    let var_reg = Regex::new(r"\$[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    for variable in var_reg.find_iter(text) {

        println!("Found: {}", variable.as_str());

        let call_reg = format!(r"{}\s?=[^=].*", regex::escape(variable.as_str()));
        let call = Regex::new(&call_reg).unwrap();

        if let Some(_capture) = call.find(text) {
            continue;
        }

        println!("{} needs to be declared", variable.as_str())
    }
}