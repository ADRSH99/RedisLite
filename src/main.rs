use self::server::create_server;

mod command;
mod server;
mod database;

#[tokio::main]
async fn main() {
	let add = "127.0.0.1:3002";
    eprintln!("Server starts at {:?}",  add);
	create_server(add).await;
}
