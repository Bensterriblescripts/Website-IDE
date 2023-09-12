use axum::{
    extract::Json,
    response::{Html, Response, IntoResponse},
    routing::post,
    routing::get,
    Router,
};
use serde::Deserialize;
use regex::Regex;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct EditorContent {
    content: String
}


async fn root() -> Html<&'static str> {
    Html(include_str!("static/index.html"))
}
async fn lib() -> Html<&'static str> {
    Html(include_str!("lib/htmx.min.js"))
}
async fn styles() -> Html<&'static str> {
    Html(include_str!("static/styles.css"))
}
async fn navbar() -> Html<&'static str> {
    Html(include_str!("components/navbar.html"))
}

async fn lsp(payload: Json<EditorContent>) -> Html<String> {
    let total_time = Instant::now();
    
    let text = payload.content.to_string();

    let parsed_time = Instant::now();
    let parsed_text = html_parser(text.as_str());
    let total_end_time = Instant::now();
    let elapsed_time = total_end_time.duration_since(parsed_time);
    let parsed_timer = elapsed_time.as_secs_f64();
    println!(":: HTML Parsed Time: {}", parsed_timer);

    let var_parsed_time = Instant::now();
    let new_text = string_variable_parser(parsed_text);
    let total_end_time = Instant::now();
    let elapsed_time = total_end_time.duration_since(var_parsed_time);
    let var_parsed_timer = elapsed_time.as_secs_f64();
    println!(":: String/Variable Parsed Time: {}", var_parsed_timer);
    
    let total_end_time = Instant::now();
    let elapsed_time = total_end_time.duration_since(total_time);
    let the_end_timer = elapsed_time.as_secs_f64();
    println!(":: Total time taken: {}", the_end_timer);

    Html(new_text)
}

#[tokio::main]
async fn main() {
    // Router
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/lib", get(lib))
        .route("/lsp", post(lsp))
        .route("/styles", get(styles))
        .route("/navbar", get(navbar));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/* ********************/
/*   HTML Inserters   */
/* ********************/

fn html_parser(text: &str) -> String {

    let html_class_var_err = Regex::new(r#"<span class="variable_error" contenteditable="(false|true)">"#).unwrap();
    let html_reg = Regex::new(r#"(?s)<[^>]*>"#).unwrap(); // Regex: Find all elements <*>

    let rawtext = html_class_var_err.replace_all(text, "");
    let rawtext = html_reg.replace_all(&rawtext, "");

    return rawtext.to_string();
}
fn string_variable_parser(mut text: String) -> String {

    let mut updates: Vec<Vec<usize>> = Vec::new();

    let html_var = r#"<span class="variable" contenteditable="false">"#;
    let html_var_err = r#"<span class="variable_error" contenteditable="false">"#;
    let html_string = r#"<span class="string">"#;
    let html_end = r#"</span>"#;

    // Strings
    let start_time = Instant::now();
    updates.append(&mut single_string_check(&text));
    updates.append(&mut double_string_check(&text));
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let string_timer = elapsed_time.as_secs_f64();
    println!("String time elapsed: {}", string_timer);

    // Variables
    let start_time = Instant::now();
    updates.append(&mut variable_check(&text));
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let variable_timer = elapsed_time.as_secs_f64();
    println!("Variables time elapsed: {}", variable_timer);

    // Create the new text string
    updates.sort_by(|a, b| b.cmp(a));
    for update in updates.iter() {
        // Variable highlight
        if update[2] == 0 {
            text.insert_str(update[1], html_end);
            text.insert_str(update[0], html_var);
        }
        // Variable highlight + error
        else if update[2] == 1 {
            text.insert_str(update[1], html_end);
            text.insert_str(update[0], html_var_err);
        }
        // String highlight ''
        else if update[2] == 2 {
            text.insert_str(update[1], html_end);
            text.insert_str(update[0], html_string);
        }
        // String highlight ""
        else if update[2] == 3 {
            text.insert_str(update[1], html_end);
            text.insert_str(update[0], html_string);
        }
        else {
            continue;
        }
    }

    return text;
}

/* ********************/
/*      Regex         */
/* ********************/

fn variable_check(text: &str) -> Vec<Vec<usize>> {

    let mut variables = Vec::new();
    let variable_highlight_code: usize = 0;
    let variable_error_code: usize = 1;

    let var_reg = Regex::new(r"\$[a-zA-Z_][a-zA-Z0-9_]*").unwrap(); // Regex: Any word starting with $

    for variable in var_reg.find_iter(&text) {

        let call_reg: String = format!(r"{}\s?=[^=].*;", regex::escape(variable.as_str())); // Regex: Find the existing variable in a place that has a single =
        let call = Regex::new(&call_reg).unwrap();
        if let Some(_capture) = call.find(&text) {
            let variable_vec = vec![variable.start(), variable.end(), variable_highlight_code];
            variables.push(variable_vec);
            continue
        }

        let variable_vec = vec![variable.start(), variable.end(), variable_error_code];
        variables.push(variable_vec);
    }

    return variables;

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
fn single_string_check(text: &str) -> Vec<Vec<usize>> {

    let mut strings = Vec::new();
    let single_strings_code: usize = 2;

    let small_string_reg = Regex::new(r"(?s)'.*?'").unwrap(); // Regex: Anything in between ''

    for string in small_string_reg.find_iter(&text) {
        let small_string_vec = vec![string.start(), string.end(), single_strings_code];
        strings.push(small_string_vec);
    }

    return strings
}
fn double_string_check(text: &str) -> Vec<Vec<usize>> {

    let mut strings = Vec::new();
    let double_strings_code: usize = 3;

    let big_string_reg = Regex::new(r#"(?s)".*?""#).unwrap(); // Regex: Anything inbetween "" 

    for string in big_string_reg.find_iter(&text) {
        let small_string_vec = vec![string.start(), string.end(), double_strings_code];
        strings.push(small_string_vec);
    }

    return strings
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