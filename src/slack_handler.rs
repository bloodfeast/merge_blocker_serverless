use reqwest;
use warp::Rejection;
use serde_json::{json, Value};
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

pub struct Config {
  pub url: String,
  pub token: String,
}

impl Config {
  pub fn new(url: &str, token: &str) -> Self {
      Self {
          url: url.to_string(),
          token: token.to_string(),
      }
  }
}

pub async fn handle_slack_command(body: Value) -> Result<impl warp::Reply, Rejection> {
    // Extract the action type and user ID from the Slack command payload
    let action = body.get("text").and_then(Value::as_str);
    let user_id = body.get("user_id").and_then(Value::as_str);

    let config = Config::new(
      "https://api.github.com/repos/OWNER/REPO/branches/BRANCH/protection/required_pull_request_reviews",
      "GITHUB_TOKEN",
    );

    match (action, user_id) {
        (Some("block"), Some(user_id)) => {
            // If the action is "block", block all merge requests except for the user who invoked the command

            let client = reqwest::Client::builder()
                .user_agent("Reqwest")
                .default_headers(headers(&config.token).unwrap())
                .build()
                .unwrap();

            let body = json!({
                "dismissal_restrictions": {
                    "users": [user_id],
                    "teams": []
                },
                "dismiss_stale_reviews": false
            });

            let response = client.patch(&config.url).json(&body).send().await.unwrap();
            
            if response.status().is_success() {
                Ok(format!("Blocked all merge requests except for user {}", user_id))
            } else {
                Ok(format!("Error blocking merge requests: {:?}", response.text().await.unwrap()))
            }
        }
        (Some("unblock"), _) => {
            // If the action is "unblock", unblock all merge requests
            
            let client = reqwest::Client::builder()
                .user_agent("Reqwest")
                .default_headers(headers(&config.token).unwrap())
                .build()
                .unwrap();

            let body = json!({
                "dismissal_restrictions": {},
                "dismiss_stale_reviews": false
            });

            let response = client.patch(&config.url).json(&body).send().await.unwrap();

            if response.status().is_success() {
                Ok(format!("Unblocked all merge requests"))
            } else {
                Ok(format!("Error unblocking merge requests: {:?}", response.text().await.unwrap()))
            }
        }
        _ => Ok(format!("Invalid command")),
    }
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