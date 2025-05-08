use async_openai::Client;
use orphic::cli;
use orphic::runner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (opts, task) = cli::parse();
    let client = Client::new();
    runner::execute(&client, opts, task).await
}
