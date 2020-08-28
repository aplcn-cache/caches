#![feature(box_syntax)]

use anyhow::Result;
use dialoguer::Input;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use serde_json::json;
use structopt::StructOpt as _;
use structopt_derive::StructOpt;

fn percent_encode_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or_else(|| RenderError::new("Parameter required"))?;
    out.write(
        percent_encode(param.value().render().as_bytes(), NON_ALPHANUMERIC)
            .to_string()
            .as_ref(),
    )?;
    Ok(())
}

#[structopt(name = "aplcn")]
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    name: Option<String>,
    #[structopt(short, long)]
    appstore: Option<String>,
    #[structopt(short, long)]
    tags: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();
    let mut reg = Handlebars::new();
    reg.register_helper("percent-encode", box percent_encode_helper);
    let template = include_str!("template.hbs");

    let name = cli
        .name
        .unwrap_or_else(|| Input::new().with_prompt("Release name").interact().unwrap());
    let appstore = cli.appstore.unwrap_or_else(|| {
        Input::new()
            .with_prompt("App Store URL")
            .interact()
            .unwrap()
    });
    let tags = cli.tags.unwrap_or_else(|| {
        Input::<String>::new()
            .with_prompt("Tags")
            .interact()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|i| i.strip_prefix('#').unwrap_or(i).to_owned())
            .collect()
    });

    let tag_name = name
        .to_lowercase()
        .replace(char::is_whitespace, "-")
        .replace(|c: char| c != '-' && !c.is_alphanumeric(), "");
    println!(
        "{}",
        reg.render_template(
            template,
            &json!({
                "file_url": format!(
                "https://github.com/aplcn-cache/caches/releases/download/{release}/{file}.imazingapp",
                    file=percent_encode(name.as_bytes(), NON_ALPHANUMERIC),
                    release=percent_encode(tag_name.as_bytes(), NON_ALPHANUMERIC)),
                "tags": tags,
                "appstore": appstore,
                "release_name": tag_name,
                "filename": format!("{}.imazingapp", name),
            }),
        )?
    );
    Ok(())
}
