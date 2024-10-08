use std::{env, fs};

use dotenvy::dotenv;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use rand::{rngs::SmallRng, SeedableRng};
use wows_box::lootbox::LootBox;
use wows_box_rand::rand::rand_multi;
use wows_box_render::{
    html::render_html,
    process::{LootBoxListProp, LOOTBOX_TEMPLATE},
};

#[tokio::test]
async fn test_rand_multi_render() -> anyhow::Result<()> {
    dotenv().ok();
    let box_id = 4147303344_u64;
    let mut client_options = ClientOptions::parse(env::var("MONGODB_CONN")?).await?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    let col: Collection<LootBox> = client.database("wowslootbox-zh-sg").collection("list");
    let lootbox = col.find_one(doc! { "id": box_id as u32 }).await?.unwrap();

    let found = vec![];

    let mut rng = SmallRng::from_entropy();

    let resp = rand_multi(&mut rng, &lootbox, 25, &found, 0);

    let list_prop = LootBoxListProp::from_result("zh-sg", &client, box_id, resp, 25).await?;

    LOOTBOX_TEMPLATE.render_to_write(
        list_prop,
        fs::File::create(
            r"D:\WBH\rust\wows-rand-box\crates\wows-box-render\tests\lootbox.output.html",
        )?,
    )?;

    render_html(
        r"D:\WBH\rust\wows-rand-box\crates\wows-box-render\tests\lootbox.output.html",
        "./tests//test.html.output.png",
        "table#list",
        "div#loaded",
    )?;

    Ok(())
}
