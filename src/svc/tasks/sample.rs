use crate::SharedState;

pub async fn start_worker(state: SharedState) -> crate::Result {
    // Background worker code goes here
    // Example: Simulate a long-running task

    log::info!("🔄 Starting sample worker");

    loop {
        tokio::select! {
            biased;
            _ = state.on_shutdown() => {
                log::warn!("Background worker shutting down...");
                break;
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                // Simulate a long-running task
                // log::info!("Performing a long-running task...");
            },
        }
    }
    log::info!("❎ Background worker stopped. Exiting.");
    Ok(())
}
