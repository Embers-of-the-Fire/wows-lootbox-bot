use std::{collections::HashMap, env};

use dotenvy::dotenv;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use rand::{rngs::SmallRng, SeedableRng};
use wows_box::lootbox::LootBox;
use wows_box_rand::rand::rand_single;

#[tokio::test]
async fn test_rand_single() -> anyhow::Result<()> {
    dotenv().ok();
    let mut client_options = ClientOptions::parse(env::var("MONGODB_CONN")?).await?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    let col: Collection<LootBox> = client.database("wowslootbox-zh-sg").collection("list");
    let lootbox = col.find_one(doc! { "id": 4288861104_u32 }).await?.unwrap();

    let found = vec![];

    let mut map = HashMap::new();

    let mut rng = SmallRng::from_entropy();

    for _ in 0..200 {
        let resp = rand_single(&mut rng, &lootbox, &found[..]);
        for r in resp {
            *map.entry(r.reward_type).or_insert(0) += r.amount;
        }
    }

    println!("{:#?}", map);

    Ok(())
}
