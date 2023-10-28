#![deny(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![allow(dead_code)]
#![cfg_attr(
    debug_assertions,
    allow(unreachable_code, unused_imports, unused_variables, unused_mut)
)]
#![warn(clippy::all)]

mod md;
mod post;
mod templates;

use std::env;
use std::error;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

use crate::post::prelude::*;
use crate::templates::prelude::*;

pub const DOMAIN: &str = "https://survival.vallentin.dev";

fn main() {
    exit({
        let code = match try_main() {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("error: {err}");
                1
            }
        };
        let _ = io::stdout().flush();
        let _ = io::stderr().flush();
        code
    });
}

fn try_main() -> Result<(), Box<dyn error::Error>> {
    let cwd = env::current_dir().unwrap();
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    if cwd != workspace_dir {
        return Err("bake must only be executed within the workspace directory".into());
    }

    let mut posts = read_posts()?;
    for post in &posts {
        render_post_page(&post)?;
    }

    posts.retain(Post::is_visible);

    render_index_page(&posts)?;
    render_blog_page(&posts)?;

    Ok(())
}

fn read_posts() -> Result<Vec<Post>, Box<dyn error::Error>> {
    println!("Reading blog posts...");

    let mut posts = Vec::new();
    for entry in fs::read_dir("posts")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }

        let path = entry.path();
        let has_md_ext = path
            .extension()
            .and_then(OsStr::to_str)
            .map_or(false, |ext| ext.eq_ignore_ascii_case("md"));
        if has_md_ext {
            let post = Post::read(path)?;
            posts.push(post);
        }
    }

    Post::sort_posts(&mut posts);

    Ok(posts)
}

fn render_index_page(posts: &[Post]) -> Result<(), Box<dyn error::Error>> {
    render_template("www/index.html", IndexPage::with_posts(posts))
}

fn render_blog_page(posts: &[Post]) -> Result<(), Box<dyn error::Error>> {
    render_template("www/blog/index.html", BlogPage::with_posts(posts))
}

fn render_post_page(post: &Post) -> Result<(), Box<dyn error::Error>> {
    let out_path = Path::new("www/blog/post")
        .join(&post.name)
        .with_extension("html");
    render_template(out_path, PostPage::with_post(post))
}

fn render_template<T>(out_path: impl AsRef<Path>, template: T) -> Result<(), Box<dyn error::Error>>
where
    T: Template,
{
    let out_path = out_path.as_ref();
    println!("Rendering `{}`", out_path.display());

    if let Some(parent_dir) = out_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let html = template.render()?;
    fs::write(out_path, html)?;

    Ok(())
}
