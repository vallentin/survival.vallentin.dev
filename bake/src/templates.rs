pub mod prelude {
    pub use askama::Template;

    pub use super::{BlogPage, IndexPage, PostPage};
}

use std::borrow::Cow;

use askama::Template;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::post::Post;

#[derive(Template, Clone, Debug)]
#[template(path = "index.html.jinja")]
pub struct IndexPage<'a> {
    title: Option<&'static str>,
    posts: &'a [Post],
    has_see_all_posts: bool,
}

impl<'a> IndexPage<'a> {
    const SHOW_MORE_POSTS_LIMIT: usize = 5;

    pub fn with_posts(posts: &'a [Post]) -> Self {
        let (has_see_all_posts, posts) = if posts.len() > Self::SHOW_MORE_POSTS_LIMIT {
            (true, &posts[..Self::SHOW_MORE_POSTS_LIMIT])
        } else {
            (false, posts)
        };

        Self {
            title: None,
            posts,
            has_see_all_posts,
        }
    }
}

#[derive(Template, Clone, Debug)]
#[template(path = "blog.html.jinja")]
pub struct BlogPage<'a> {
    title: Option<&'static str>,
    posts: &'a [Post],
    has_see_all_posts: bool,
}

impl<'a> BlogPage<'a> {
    pub fn with_posts(posts: &'a [Post]) -> Self {
        Self {
            title: Some("Blog"),
            posts,
            has_see_all_posts: false,
        }
    }
}

#[derive(Template, Clone, Debug)]
#[template(path = "post.html.jinja")]
pub struct PostPage<'a> {
    title: Option<&'a str>,
    post: &'a Post,
    links: Vec<SocialLink<'a>>,
}

impl<'a> PostPage<'a> {
    pub fn with_post(post: &'a Post) -> Self {
        Self {
            title: Some(&post.meta.title),
            post,
            links: post
                .meta
                .links
                .iter()
                .map(|link| SocialLink::from_url(link))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
struct SocialLink<'a> {
    url: &'a str,
    title: Cow<'static, str>,
}

impl<'a> SocialLink<'a> {
    fn from_url(url: &'a str) -> Self {
        let title = match () {
            _ if url.contains("reddit.com") => {
                static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"/r/(.*?)/").unwrap());

                let title = match RE.captures(url) {
                    Some(caps) => {
                        let subreddit_name = caps.get(1).unwrap().as_str();
                        Cow::Owned(format!("Reddit (/r/{})", subreddit_name))
                    }
                    None => Cow::Borrowed("Reddit"),
                };

                title
            }
            _ if url.contains("twitter.com") || url.contains("x.com") => Cow::Borrowed("Twitter/X"),
            _ if url.contains("mastodon") => Cow::Borrowed("Mastodon"),
            _ if url.contains("discord.com") || url.contains("discord.gg") => {
                Cow::Borrowed("Discord")
            }
            _ => unimplemented!("{:?} is not supported", url),
        };

        Self { url, title }
    }
}
