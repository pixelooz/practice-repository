#![allow(unused, dead_code)]

use anyhow::Result;
use tokio::runtime;

fn main() -> Result<()> {
    let rt = runtime::Runtime::new()?;
    let response = rt.block_on(make_request())?;
    println!("{}", response);
    Ok(())
}

async fn make_request() -> Result<String> {
    let url = "https://rust-lang.org";
    let res = reqwest::get(url).await?.text().await?;
    Ok(res)
}

fn never_type(strs: String) -> Result<Vec<char>, !> {
    let chars =
        strs.chars().map(|ch| ch.to_ascii_lowercase()).filter(|ch| ch.is_alphabetic()).collect();
    Ok(chars)
}
