use std::sync::Mutex;
use serde::{Serialize};
use std::fs;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Serialize)]
struct PageData<T> {
    props: T,
    component: String,
}

static APP_HTML: Lazy<Mutex<String>> = Lazy::new(|| {
    let content = fs::read_to_string(String::from("./src/index.html")).expect("Could not find index.html");
    return Mutex::new(content);
});

pub fn render_with_props<T>(component: String, props: T) -> String where T: Serialize {
    let re = Regex::new("<!-- *?@app\\(\\) *?-->").unwrap();
    let app_html = APP_HTML.lock().unwrap().to_string();
    let page_data = PageData {
        component,
        props
    };
    let serialized_page_data: String = serde_json::to_string(&page_data).unwrap();
    let app_html_with_page_data = re.replace(&app_html, format!(r#"<div id="root" data-page='{}'></div>"#, serialized_page_data));

    return app_html_with_page_data.to_string();
}
