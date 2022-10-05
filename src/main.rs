#[tokio::main]
async fn main() {
    docker_manager::run().await;
}
