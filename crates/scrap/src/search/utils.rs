use scraper::{Html, Selector};

pub fn trim_head_read_days_ago(head_read: String) -> String {
    let trimmed_head_read = trim_to(head_read, "天之前");
    let trimmed_head_read = trim_to(trimmed_head_read, "天前");
    let trimmed_head_read = trim_to(trimmed_head_read, "days ago");
    let trimmed_head_read = trim_to(trimmed_head_read, "hours ago");
    let trimmed_head_read = trim_to(trimmed_head_read, "day ago");
    let trimmed_head_read = trim_to(trimmed_head_read, "hours ago");
    trim_to(trimmed_head_read, "days ago · ")
}

pub fn trim_html_with_script_and_style(html_text: &str) -> String {
    let document = Html::parse_document(html_text);
    let root_selector = Selector::parse("body > *").unwrap();

    let mut new_html_content = String::from("<html><body>");
    for element in document.select(&root_selector) {
        if !element.value().name().eq_ignore_ascii_case("script")
            && !element.value().name().eq_ignore_ascii_case("style")
            && !element.value().name().eq_ignore_ascii_case("meta")
            && !element.value().name().eq_ignore_ascii_case("link")
            && !element.value().name().eq_ignore_ascii_case("iframe")
            && !element.value().name().eq_ignore_ascii_case("noscript") {
            new_html_content.push_str(&element.html());
        }
    }
    new_html_content.push_str("</body></html>");

    new_html_content.into()
}

fn trim_to(text: String, redundant_prefix: &str) -> String {
    let offset = redundant_prefix.len();
    match text.find(redundant_prefix) {
        None => text,
        Some(index) => text.get(index + offset..).unwrap_or_else(|| panic!("text = {}, redundant_prefix = {}", text, redundant_prefix)).to_string()
    }
}