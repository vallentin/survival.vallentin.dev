use matter::matter;
use pulldown_cmark::{html, Options, Parser};

pub fn split_front_matter<'a>(contents: &'a str) -> (Option<&'a str>, &str) {
    match matter(contents) {
        Some((yaml, md)) => (Some(yaml), md),
        None => (None, contents),
    }
}

pub fn render(md: &str) -> String {
    let mut opt = Options::empty();
    opt.insert(Options::ENABLE_TABLES);
    opt.insert(Options::ENABLE_FOOTNOTES);
    opt.insert(Options::ENABLE_STRIKETHROUGH);
    opt.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, opt);

    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}
