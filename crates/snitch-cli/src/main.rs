#[tokio::main]
async fn main() {
    println!("snitch {}", env!("CARGO_PKG_VERSION"));
}
