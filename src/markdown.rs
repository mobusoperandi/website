use pulldown_cmark::{html, Options, Parser};

pub fn to_html(markdown: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(markdown, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}
