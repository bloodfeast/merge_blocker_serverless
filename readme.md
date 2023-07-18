# GitHub Deployment Blocking Tool
This tool is a serverless function written in Rust, which automatically blocks and unblocks merge requests to a GitHub repository during deployment based on specific Slack commands. The function is designed to be triggered by Slack events.

## Setup
To set up the function, you will need to:

1. Install the Rust programming language and the AWS CLI tool.
2. Clone this repository and navigate to its directory.
3. Build the function using cargo build --release --target x86_64-unknown-linux-musl.
4. Deploy the function to AWS Lambda.

## Configuration
This function uses two main configuration parameters: the GitHub API URL and the token for accessing the GitHub API. These parameters are stored in a Config struct, which is passed around in the function.

`pub struct Config {
    pub url: String,
    pub token: String,
}
`
#### You will also need to replace "https://api.github.com/repos/OWNER/REPO/branches/BRANCH/protection/required_status_checks" and "GITHUB_TOKEN" in the Config::new function call with your actual values.

#### Additionally, you need to set up a webhook in your Slack workspace to enable the function to receive Slack events and execute the block and unblock operations.

## Usage
The function listens for specific Slack commands. When a deployment starts, the appropriate Slack command triggers the function to block all merge requests to the specified branch. When the deployment ends, a different Slack command triggers the function to unblock all merge requests.

## Testing
You can run tests for this function using cargo test. The tests ensure that the function correctly sets up the headers for the GitHub API request.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

Please replace OWNER/REPO/BRANCH and GITHUB_TOKEN with actual values. Also, please adjust any other sections to fit your actual project requirements.