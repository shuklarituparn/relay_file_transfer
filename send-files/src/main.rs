use anyhow::{Ok, Result};
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{
    net_protocol::Blobs,
    rpc::client::blobs::{ReadAtLen, WrapOption},
    ticket::BlobTicket,
    util::SetTagOption,
};
use std::{path::PathBuf, str::FromStr};
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;
    let blobs = Blobs::memory().build(&endpoint);
    let node = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .spawn()
        .await?;
    println!("Iroh nodes created");
    let blobs = blobs.client();
    let args = std::env::args().collect::<Vec<_>>();
    match &args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [_cmd, "send", path] => {
            let abs_path = PathBuf::from_str(path)?.canonicalize()?;
            println!("Analysing file.");

            let blob = blobs
                .add_from_path(abs_path, true, SetTagOption::Auto, WrapOption::NoWrap)
                .await?
                .finish()
                .await?;

            let node_id = node.endpoint().node_id();
            let ticket = BlobTicket::new(node_id.into(), blob.hash, blob.format)?;

            println!("File analysed. Fetch this by running:");
            println!("cargo run --example transfer --receive {ticket} {path}");
            tokio::signal::ctrl_c().await?;
        }
        [_cmd, "receive", ticket, path] => {
            let path_buf = PathBuf::from_str(path)?;
            let ticket = BlobTicket::from_str(ticket)?;

            println!("Starting download.");

            let download_future = blobs.download(ticket.hash(), ticket.node_addr().clone());
            let download_result = timeout(Duration::from_secs(30), download_future).await??;
            println!("Download started, waiting to finish.");
            download_result.finish().await?;
            println!("Finished download.");
            println!("Copying to destination");

            let mut file = tokio::fs::File::create(path_buf).await?;
            let mut reader = blobs.read_at(ticket.hash(), 0, ReadAtLen::All).await?;

            tokio::io::copy(&mut reader, &mut file).await?;

            println!("Finished Copying.")
        }
        _ => {
            println!("Couldn't parse command line arguments.");
            println!("Usage:");
            println!("    # to send:");
            println!("    cargo run --example transfer -- send [FILE]");
            println!("    # this will print a ticket.");
            println!();
            println!("    # to receive:");
            println!("    cargo run --example transfer -- receive [TICKET] [FILE]");
        }
    }

    println!("Shutting down.");
    node.shutdown().await?;
    Ok(())
}