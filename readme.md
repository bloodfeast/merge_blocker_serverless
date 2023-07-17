# GitHub Deployment Blocking Tool
This tool is a serverless function written in Rust, designed to be deployed to AWS Lambda, that automatically blocks and unblocks merge requests to a GitHub repository during deployment.

## Setup
To set up the function, you will need to:

1. Install the Rust programming language and the AWS CLI tool.
2. Clone this repository and navigate to its directory.
3. Build the function using cargo build --release --target x86_64-unknown-linux-musl.
4. Deploy the function to AWS Lambda.

## Configuration
To configure the function, you will need to provide your GitHub token, which the function uses to authenticate with the GitHub API. You will need to replace "GITHUB_TOKEN" with your actual GitHub token in the main function.

### You will also need to replace "https://api.github.com/repos/OWNER/REPO/branches/BRANCH/protection/required_status_checks" with the URL of the status checks for the branch you want to protect.

## Usage
The function automatically runs whenever a deployment starts or ends. When a deployment starts, it blocks all merge requests to the specified branch. When the deployment ends, it unblocks all merge requests.

## Testing
You can run tests for this function using cargo test.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

Please replace OWNER/REPO/BRANCH and GITHUB_TOKEN with actual values. Also, please adjust any other sections to fit your actual project requirements.