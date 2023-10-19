pub mod prelude {
    pub use super::{Post, PostMeta};
}

use std::cmp::Reverse;
use std::error;
use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use serde::Deserialize;

use crate::md;

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PostMeta {
    pub title: String,
    pub published: NaiveDate,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub pinned: bool,
}

#[derive(Clone, Debug)]
pub struct Post {
    pub meta: PostMeta,
    pub md: String,
    pub name: String,
}

impl Post {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, Box<dyn error::Error>> {
        let path = path.as_ref();
        println!("Reading `{}`", path.display());

        let contents = fs::read_to_string(&path)?;
        let (yaml, md) = md::split_front_matter(&contents);

        let yaml = yaml.ok_or_else::<Box<dyn error::Error>, _>(|| {
            format!("missing meta in `{}`", path.display()).into()
        })?;
        let meta: PostMeta = serde_yaml::from_str(yaml)?;

        let name = path
            .file_stem()
            // Safe to unwrap as `path` is a file
            .unwrap()
            .to_str()
            // Safe to unwrap as `path` is guaranteed to be valid UTF-8
            .unwrap()
            .to_owned();

        Ok(Self {
            meta,
            md: md.to_owned(),
            name,
        })
    }

    #[inline]
    pub fn render(&self) -> String {
        md::render(&self.md)
    }

    pub fn sort_posts(posts: &mut [Self]) {
        posts.sort_by_key(|post| Reverse(post.meta.published));
        posts.sort_by_key(|post| Reverse(post.meta.pinned));
    }
}
