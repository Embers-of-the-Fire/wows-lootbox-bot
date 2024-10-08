use std::{
    env,
    panic::{self, PanicInfo},
    time::Instant,
};

use dotenvy::dotenv;
use log::{error, info};
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
// use human_panic::setup_panic;

mod update_boxlist;
mod update_currency;
mod update_items;
mod update_lang_list;

const LANGUAGE_LIST: &'static [&'static str] = &["zh-sg", "en"];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    panic::set_hook(Box::new(panic_handler));
    // setup_panic!();
    dotenv().ok();

    info!("Starting update wows data...");
    let time_c = Instant::now();

    info!("Started connecting to database...");
    let time = Instant::now();
    let mut client_options = ClientOptions::parse(env::var("MONGODB_CONN")?).await?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    info!(
        "Successfully build connection to database in {:.2}s.",
        time.elapsed().as_secs_f64()
    );

    // Send a ping to confirm a successful connection
    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;
    info!("Pinged database, connection verified.");

    update_lang_list::update_lang_list(LANGUAGE_LIST, &client).await?;

    for lang in LANGUAGE_LIST {
        update_boxlist::update_boxlist(lang, &client).await?;
        update_items::update_items(lang, &client).await?;
        update_currency::update_currency(lang, &client).await?;
    }

    info!(
        "Successfully finished update in {:.2}s.",
        time_c.elapsed().as_secs_f64()
    );
    Ok(())
}

fn panic_handler(panic_info: &PanicInfo) {
    error!("Panic occurred: {}", panic_info);
    std::process::exit(1);
}
