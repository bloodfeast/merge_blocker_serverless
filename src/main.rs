
use warp::Filter;

mod slack_handler; // assuming slack_handler.rs is in the same directory as main.rs

use slack_handler::handle_slack_command;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a route for the Slack command
    let route = warp::path("command")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_slack_command);

    warp::serve(route).run(([0, 0, 0, 0], 8080)).await;

    Ok(())
}

