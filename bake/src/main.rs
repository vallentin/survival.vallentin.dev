#![deny(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![allow(dead_code)]
#![cfg_attr(
    debug_assertions,
    allow(unreachable_code, unused_imports, unused_variables, unused_mut)
)]
#![warn(clippy::all)]

mod templates;

use std::env;
use std::error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

use crate::templates::prelude::*;

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

    render_index_page()?;

    Ok(())
}

fn render_index_page() -> Result<(), Box<dyn error::Error>> {
    render_template("www/index.html", IndexPage::new())?;

    Ok(())
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
