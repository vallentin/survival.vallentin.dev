use std::collections::VecDeque;
use std::iter::{self, FusedIterator};

use itertools::Itertools;
use matter::matter;
use pulldown_cmark::{escape, html, CowStr, Event, Options, Parser, Tag};

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
    let parser = p_add_has_img(parser);
    let parser = img_to_video(parser);

    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}

// Needed until the `:has()` selector is better supported
fn p_add_has_img<'a, I>(events: I) -> impl Iterator<Item = Event<'a>> + FusedIterator
where
    I: Iterator<Item = Event<'a>> + FusedIterator,
{
    let mut events = events.peekable();
    let mut paragraph = VecDeque::new();

    iter::from_fn(move || {
        if let Some(evt) = paragraph.pop_front() {
            return Some(evt);
        }

        if let Some(evt) = events.next_if(|evt| !matches!(evt, Event::Start(Tag::Paragraph))) {
            return Some(evt);
        }
        paragraph.push_back(events.next()?);
        paragraph
            .extend(events.take_while_inclusive(|evt| !matches!(evt, Event::End(Tag::Paragraph))));

        assert_eq!(paragraph.front(), Some(&Event::Start(Tag::Paragraph)));
        assert_eq!(paragraph.back(), Some(&Event::End(Tag::Paragraph)));

        if paragraph
            .iter()
            .any(|evt| matches!(evt, Event::Start(Tag::Image(..))))
        {
            paragraph.pop_front();
            paragraph.push_front(Event::Html(CowStr::Borrowed("<p class=\"has-img\">")))
        }

        paragraph.pop_front()
    })
    .fuse()
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
