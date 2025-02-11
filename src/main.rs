use crate::core::entity::player::class::{AliveClass, Class, DeadClass};

mod core;
mod error;

#[tokio::main]
async fn main() -> error::Result<()> {
	Ok(())
}