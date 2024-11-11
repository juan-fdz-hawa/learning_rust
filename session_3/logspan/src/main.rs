use tracing_subscriber::fmt::format::FmtSpan;

// Issue an span events around this func
#[tracing::instrument]
async fn hello_world() {
    println!("Hello")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Tracing works as an event based system
    // let subscriber = tracing_subscriber::FmtSubscriber::new();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Emit an info trace
    tracing::info!("Starting up");
    tracing::error!("Error");
    tracing::warn!("Warning");

    hello_world().await;

    Ok(())
}
