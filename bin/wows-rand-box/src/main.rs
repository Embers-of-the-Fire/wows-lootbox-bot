use std::process::exit;
use std::sync::Arc;
use std::{env, panic};
use std::{net::SocketAddr, panic::PanicInfo};

use async_once::AsyncOnce;
use axum::routing::{get, post};
use axum::Router;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use log::error;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use tokio::{
    // io::AsyncWriteExt,
    net::TcpListener,
};

use rand_handler::rand_handler;
use search_handler::search_handler;

mod rand_handler;
mod search_handler;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum AppResponse<T> {
    Ok { data: T },
    Error { brief: String, full: String },
}

impl<T> From<anyhow::Result<T>> for AppResponse<T> {
    fn from(value: anyhow::Result<T>) -> Self {
        match value {
            Ok(t) => AppResponse::Ok { data: t },
            Err(e) => AppResponse::Error {
                brief: format!("{e}"),
                full: format!("{e:?}"),
            },
        }
    }
}

lazy_static! {
    pub static ref APP_STATE: AsyncOnce<AppState> = AsyncOnce::new(async {
        AppState {
            conn: Arc::new({
                let mut client_options = ClientOptions::parse(env::var("MONGODB_CONN").unwrap())
                    .await
                    .unwrap();
                let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
                client_options.server_api = Some(server_api);
                let client = Client::with_options(client_options).unwrap();
                client
            }),
        }
    });
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: Arc<Client>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    panic::set_hook(Box::new(panic_handler));
    log4rs::init_file(
        concat!(env!("CARGO_MANIFEST_DIR"), "/log4rs.yaml"),
        Default::default(),
    )?;

    let lootbox = Router::new()
        .route("/rand", post(rand_handler))
        .route("/search", get(search_handler));
    let app = Router::new().nest("/lootbox", lootbox);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

fn panic_handler(panic_info: &PanicInfo) {
    error!("{}", panic_info);
    exit(1);
}
