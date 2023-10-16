use mc_oxide::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    mc_oxide::logger::init().await?;

    let config = mc_oxide::Config::load()?;
    let servers = mc_oxide::Server::find(&config.server_dir).await?;

    for server in servers {
        println!("{:?}", server);
    }

    Ok(())
}
