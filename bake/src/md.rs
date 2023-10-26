use std::iter::FusedIterator;

use matter::matter;
use pulldown_cmark::{escape, html, Event, Options, Parser, Tag};

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
    let parser = img_to_video(parser);

    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}

fn img_to_video<'a, I>(events: I) -> impl Iterator<Item = Event<'a>> + FusedIterator
where
    I: Iterator<Item = Event<'a>> + FusedIterator,
{
    let mut in_video = false;
    events.filter_map(move |evt| match evt {
        Event::Start(Tag::Image(_, url, _)) if url.ends_with(".mp4") => {
            assert!(!in_video);
            in_video = true;
            None
        }
        Event::End(Tag::Image(_, url, title)) if in_video => {
            assert!(title.is_empty());
            in_video = false;
            let html = video(url);
            Some(Event::Html(html.into()))
        }
        _ if in_video => {
            unimplemented!("{:?}", evt);
        }
        _ => Some(evt),
    })
}

fn video(url: impl AsRef<str>) -> String {
    let mut html = String::new();
    html.push_str("<video src=\"");
    escape::escape_href(&mut html, url.as_ref()).unwrap();
    html.push_str(r#"" autoplay="" loop="" muted="" playsinline="">"#);
    html.push_str("Your browser does not support embedded videos.");
    html.push_str("</video>");
    html
}
