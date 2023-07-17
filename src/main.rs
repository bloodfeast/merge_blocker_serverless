use reqwest;
use serde_json::json;
use std::error::Error;
use std::fmt;

// Define a custom error type to encapsulate the possible errors
#[derive(Debug)]
enum RequestError {
    RequestError(reqwest::Error),
    HeaderParseError(reqwest::header::InvalidHeaderValue),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::RequestError(err) => write!(f, "Request Error: {}", err),
            RequestError::HeaderParseError(err) => write!(f, "Header Parse Error: {}", err),
        }
    }
}

impl Error for RequestError {}

impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> RequestError {
        RequestError::RequestError(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for RequestError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> RequestError {
        RequestError::HeaderParseError(err)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url: &str =
        "https://api.github.com/repos/OWNER/REPO/branches/BRANCH/protection/required_status_checks"; /////////////////////////////////////////////// replace with real path
    let token: &str = "GITHUB_TOKEN"; ////////////////////////////////////////////////////////////////////////////////////////////////////////////// replace with GitHub token

    let client = reqwest::Client::builder()
        // Set a user agent.
        .user_agent("Reqwest")
        .default_headers(headers(token)?)
        .build()?;

    // If deployment is starting, create the payload for starting deployment
    let start_body: serde_json::Value = json!({
        "strict": true,
        "contexts": ["deploying"]
    });

    // Send the POST request to start deployment and handle the response
    let response_start = client.post(url).json(&start_body).send().await?;
    if response_start.status().is_success() {
        println!("{:#?}", response_start.text().await?);
    } else {
        println!(
            "Error starting deployment: {:?}",
            response_start.text().await?
        );
    }

    // If deployment is ending, create the payload for ending deployment
    let end_body: serde_json::Value = json!({
        "strict": false,
        "contexts": []
    });

    // Send the POST request to end deployment and handle the response
    let response_end = client.post(url).json(&end_body).send().await?;
    if response_end.status().is_success() {
        println!("{:#?}", response_end.text().await?);
    } else {
        println!("Error ending deployment: {:?}", response_end.text().await?);
    }

    Ok(())
}

// Function to create default headers for API requests
fn headers(token: &str) -> Result<reqwest::header::HeaderMap, RequestError> {
    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    let auth_value: String = format!("token {}", token);

    headers.insert(
        reqwest::header::AUTHORIZATION,
        auth_value
            .parse()
            .map_err(RequestError::from)?,
    );
    headers.insert(
        reqwest::header::ACCEPT,
        "application/vnd.github.v3+json"
            .parse()
            .map_err(RequestError::from)?,
    );

    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT};

    #[test]
    fn test_headers() {
        let token: &str = "test_token";
        let expected_auth_value: String = format!("token {}", token);
        let expected_auth_header: HeaderValue = HeaderValue::from_str(&expected_auth_value).unwrap();
        let expected_accept_header: HeaderValue = HeaderValue::from_str("application/vnd.github.v3+json").unwrap();

        let headers: HeaderMap = headers(token).unwrap();

        assert_eq!(headers.get(AUTHORIZATION).unwrap(), &expected_auth_header);
        assert_eq!(headers.get(ACCEPT).unwrap(), &expected_accept_header);
    }
}