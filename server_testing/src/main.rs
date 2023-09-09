// use axum::{
//     extract::Json,
//     response::Html,
//     routing::post,
//     routing::get,
//     Router,
// };
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct GetText {
//     line: String
// }

use regex::Regex;
use std::time::Instant;


// async fn home() -> Html<&'static str> {
//     Html("<html><body>Hello <b>Woooooooorld!</b></body></html>")
// }

// async fn get_text(Json(GetText { line }): Json<GetText>) -> Html<&'static str> {
//     let output = send_to_purgatory(&line);
//     Html(format!("Output: {}", output.as_str()))
// }

#[tokio::main]
async fn main() {

    let html_var = r#"<span class="variable">"#;
    let html_var_err = r#"<span class="variable_error">"#;
    let html_string = r#"<span class="string">"#;

    let html_end = r#"</span>"#;

    let mut text = r#"
    $strng = "Hello";
    if ($string == "Hello") {
        echo 'Hi!!';
    }
    if ($confirm = "Test") {
        echo 'Confirmed';
    }

    test();
    "#.to_string();

    println!("Issues:");
    let total_time = Instant::now();

    // Variables
    let start_time = Instant::now();
    let variables = variable_check(&text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let variable_timer = elapsed_time.as_secs_f64();
    let mut text = String::from(text);
    for variable in variables.0.iter().rev() {
        if !variables.1.contains(variable) {
            text.insert_str(variable[1], &html_end);
            text.insert_str(variable[0], &html_var);
        }
        else {
            text.insert_str(variable[1], &html_end);
            text.insert_str(variable[0], &html_var_err);
        }
    }
    println!("Variables time elapsed: {}", variable_timer);

    // let start_time = Instant::now();
    // let delimiters = delimiter_check(text);
    // let end_time = Instant::now();
    // let elapsed_time = end_time.duration_since(start_time);
    // let delimiter_timer = elapsed_time.as_secs_f64();

    // let start_time = Instant::now();
    // let semicolons = semicolon_check(text);
    // let end_time = Instant::now();
    // let elapsed_time = end_time.duration_since(start_time);
    // let semicolon_timer = elapsed_time.as_secs_f64();

    // let start_time = Instant::now();
    // let conditions = condition_check(text);
    // let end_time = Instant::now();
    // let elapsed_time = end_time.duration_since(start_time);
    // let condition_timer = elapsed_time.as_secs_f64();

    // Strings
    let start_time = Instant::now();
    let strings = double_string_highlight(&text);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let string_timer = elapsed_time.as_secs_f64();
    let mut text = String::from(text);
    for string in strings.iter().rev() {
        text.insert_str(string[1], &html_end);
        text.insert_str(string[0], &html_string);
    }
    println!("String time elapsed: {}", string_timer);

    // let start_time = Instant::now();
    // let functions = function_check(&text);
    // let end_time = Instant::now();
    // let elapsed_time = end_time.duration_since(start_time);
    // let function_timer = elapsed_time.as_secs_f64();

    let total_end_time = Instant::now();
    let elapsed_time = total_end_time.duration_since(total_time);
    let the_end_timer = elapsed_time.as_secs_f64();
    println!("Total time taken: {}", the_end_timer);

    println!("New text: {}", text);



    // let app = Router::new()
    //     .route("/", get(home))
    //     .route("/testing", post(get_text));

    // axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

/* ********************/
/* Checks start here! */
/* ********************/

fn variable_check(text: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {

    let mut highlights = Vec::new();
    let mut errors = Vec::new();

    let var_reg = Regex::new(r"\$[a-zA-Z_][a-zA-Z0-9_]*").unwrap(); // Regex: Any word starting with $

    for variable in var_reg.find_iter(&text) {

        let variable_vec = vec![variable.start(), variable.end()];
        highlights.push(variable_vec);

        println!("Variable starts at: {}", variable.as_str());

        let call_reg: String = format!(r"{}\s?=[^=].*;", regex::escape(variable.as_str())); // Regex: Find the existing variable in a place that has a single =
        let call = Regex::new(&call_reg).unwrap();
        if let Some(_capture) = call.find(&text) {
            continue;
        }

        let variable_vec = vec![variable.start(), variable.end()];
        errors.push(variable_vec);

        println!("Variable needs to be declared: {}", variable.as_str());
    }

    return (highlights, errors);

}

// fn delimiter_check(text: &str) {
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

// }

// fn condition_check(text: &str) -> Vec<String> {

    // let mut conditions = Vec::new();

    // let cond_reg = Regex::new(r"(if|else if|elseif).*\(.*=.*\)").unwrap(); // Regex: If statement
    // let eq_reg = Regex::new(r"[^=!]=[^=]").unwrap(); // Regex: Missing operand

    // for if_state in cond_reg.find_iter(text) {
    //     for eq in eq_reg.find_iter(if_state.as_str()) {

    //         let condition_string = eq.as_str().to_string(); // Error: Missing operand
    //         conditions.push(condition_string);

    //         println!("'{}' needs another operand (try != or ==)", eq.as_str());
    //     }
    // }

    // return conditions;
// }

// fn semicolon_check(text: &str) {
    // let semivar_reg = Regex::new(r"(?s)\$.*=.*;").unwrap();

    // for semicolon in semivar_reg.find_iter(text) {
    //     println!("Needs a semi-colon: {}", semicolon.as_str());
    // }
// }

fn double_string_highlight(text: &str) -> Vec<Vec<usize>> {

    let mut highlights = Vec::new();

    let biggistring_reg = Regex::new(r#"(?s)".*?""#).unwrap(); // Regex: Anything inbetween "" 

    for big_string in biggistring_reg.find_iter(text) {

        let small_string_vec = vec![big_string.start(), big_string.end()];
        highlights.push(small_string_vec);

        println!("Found a string: {}", big_string.as_str());
    }

    return highlights
}
fn single_string_highlight(text: &str) -> Vec<Vec<usize>> {

    let mut highlights = Vec::new();

    let ministring_reg = Regex::new(r"(?s)'.*?'").unwrap(); // Regex: Anything in between ''

    for small_string in ministring_reg.find_iter(text) {

        let small_string_vec = vec![small_string.start(), small_string.end()];
        highlights.push(small_string_vec);

        println!("Found a string: {}", small_string.as_str());
    }

    return highlights
}

// fn function_check(text: &str) -> Vec<String> {

    // let mut functions = Vec::new();

    // let function_reg = Regex::new(r"(?s)[^\$][a-zA-Z_][a-zA-Z0-9_]*\(.*?\)").unwrap(); // Regex: Any word ending in ()

    // for function in function_reg.find_iter(text) {
    //     let function_string = function.as_str().to_string(); // Highlighting: Functions
    //     functions.push(function_string);

    //     println!("Found function: {}", function.as_str());
    // }

    // return functions;
// }