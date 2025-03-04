use bevy::app::App;

mod error;
mod core;
#[tokio::main]
async fn main() -> error::Result<()> {
    App::new().run().await;
    Ok(())
}

