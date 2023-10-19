pub mod prelude {
    pub use askama::Template;

    pub use super::{BlogPage, IndexPage, PostPage};
}

use askama::Template;

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
}

impl<'a> PostPage<'a> {
    pub fn with_post(post: &'a Post) -> Self {
        Self {
            title: Some(&post.meta.title),
            post,
        }
    }
}
