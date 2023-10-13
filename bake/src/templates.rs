pub mod prelude {
    pub use askama::Template;

    pub use super::IndexPage;
}

use askama::Template;

#[derive(Template, Clone, Debug)]
#[template(path = "index.html.jinja")]
pub struct IndexPage {
    title: Option<&'static str>,
}

impl IndexPage {
    pub fn new() -> Self {
        Self { title: None }
    }
}
