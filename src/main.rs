mod config;
mod routes;
pub mod webhooks;

use self::config::{Options};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts = Options::new();
    return opts.run().await;
}
