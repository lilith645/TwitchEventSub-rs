use crate::{EventSubError, SendMessage, Subscription, Token, TwitchEventSubApi, Validation};
use curl::easy::{Easy, List};

use log::{error, info};

use crate::modules::{
  consts::*,
  generic_message::{SendTimeoutRequest, TimeoutRequestData},
};

pub struct TwitchApi;

impl TwitchApi {
  /// Returns EventSubError::
  pub fn send_chat_message<S: Into<String>, T: Into<String>, V: Into<String>, X: Into<String>>(
    message: S,
    access_token: T,
    client_id: V,
    broadcaster_account_id: X,
    sender_account_id: Option<V>,
    is_reply_parent_message_id: Option<String>,
  ) -> Result<String, EventSubError> {
    let message = message.into();
    if message.len() > 500 {
      return Err(EventSubError::MessageTooLong);
    }

    let broadcaster_account_id = broadcaster_account_id.into();
    TwitchHttpRequest::new(SEND_MESSAGE_URL)
      .json_content()
      .full_auth(access_token, client_id)
      .is_post(
        serde_json::to_string(&SendMessage {
          broadcaster_id: broadcaster_account_id.to_owned(),
          sender_id: sender_account_id
            .ok_or(broadcaster_account_id)
            .map(|s| s.into())
            .unwrap(),
          message: message.into(),
          reply_parent_message_id: is_reply_parent_message_id,
        })
        .unwrap(),
      )
      .run()
  }

  pub fn generate_token_from_refresh_token<S: Into<String>, T: Into<String>, V: Into<String>>(
    client_id: S,
    client_secret: T,
    refresh_token: V,
  ) -> Result<Token, EventSubError> {
    let post_data = format!(
      "grant_type=refresh_token&refresh_token={}&client_id={}&client_secret={}",
      refresh_token.into(),
      client_id.into(),
      client_secret.into()
    );

    TwitchEventSubApi::process_token_query(post_data)
  }

  pub fn get_user_token_from_authorisation_code<
    S: Into<String>,
    T: Into<String>,
    V: Into<String>,
    W: Into<String>,
  >(
    client_id: S,
    client_secret: T,
    authorisation_code: V,
    redirect_url: W,
  ) -> Result<Token, EventSubError> {
    let post_data = format!(
      "client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}",
      client_id.into(),
      client_secret.into(),
      authorisation_code.into(),
      redirect_url.into()
    );

    TwitchEventSubApi::process_token_query(post_data)
  }

  pub fn get_authorisation_code<S: Into<String>, T: Into<String>>(
    client_id: S,
    redirect_url: T,
    scopes: &Vec<Subscription>,
  ) -> Result<String, EventSubError> {
    let redirect_url = redirect_url.into();

    let scope = &scopes
      .iter()
      .map(|s| s.required_scope())
      .filter(|s| !s.is_empty())
      .collect::<Vec<String>>()
      .join("+");

    let get_authorisation_code_request = format!(
      "{}authorize?response_type=code&client_id={}&redirect_uri={}&scope={}",
      TWITCH_AUTHORISE_URL,
      client_id.into(),
      redirect_url.to_owned(),
      scope
    );

    match TwitchEventSubApi::open_browser(get_authorisation_code_request, redirect_url) {
      Ok(http_response) => {
        if http_response.contains("error") {
          Err(EventSubError::UnhandledError(format!("{}", http_response)))
        } else {
          let auth_code = http_response.split('&').collect::<Vec<_>>()[0]
            .split('=')
            .collect::<Vec<_>>()[1];
          Ok(auth_code.to_string())
        }
      }
      e => e,
    }
  }

  pub fn generate_user_token<S: Into<String>, T: Into<String>, V: Into<String>>(
    client_id: S,
    client_secret: T,
    redirect_url: V,
    subscriptions: &Vec<Subscription>,
  ) -> Result<Token, EventSubError> {
    let client_id = client_id.into();
    let client_secret = client_secret.into();
    let redirect_url = redirect_url.into();

    TwitchApi::get_authorisation_code(
      client_id.to_owned(),
      redirect_url.to_owned(),
      &subscriptions,
    )
    .and_then(|authorisation_code| {
      TwitchApi::get_user_token_from_authorisation_code(
        client_id.to_owned(),
        client_secret.to_owned(),
        authorisation_code.to_owned(),
        redirect_url.to_owned(),
      )
    })
  }

  pub fn delete_message<
    U: Into<String>,
    S: Into<String>,
    X: Into<String>,
    Z: Into<String>,
    F: Into<String>,
  >(
    broadcaster_id: X,
    moderator_id: Z,
    message_id: S,
    access_token: U,
    client_id: F,
  ) -> Result<String, EventSubError> {
    let url = RequestBuilder::new()
      .add_key_value("broadcaster_id", broadcaster_id.into())
      .add_key_value("moderator_id", moderator_id.into())
      .add_key_value("message_id", message_id.into())
      .build(TWITCH_DELETE_MESSAGE_URL);

    TwitchHttpRequest::new(url)
      .header_authorisation(access_token.into(), AuthType::Bearer)
      .header_client_id(client_id.into())
      .is_delete()
      .run()
  }

  pub fn timeout_user<
    T: Into<String>,
    S: Into<String>,
    V: Into<String>,
    X: Into<String>,
    Z: Into<String>,
    O: Into<String>,
  >(
    access_token: T,
    client_id: S,
    broadcaster_id: X,
    moderator_id: Z,
    user_id: V,
    duration_secs: u32,
    reason: O,
  ) -> Result<String, EventSubError> {
    let broadcaster_id = broadcaster_id.into();
    let url = RequestBuilder::new()
      .add_key_value("broadcaster_id", broadcaster_id.to_owned())
      .add_key_value("moderator_id", moderator_id.into())
      .build(TWITCH_BAN_URL);

    let post_data = SendTimeoutRequest {
      data: TimeoutRequestData {
        user_id: user_id.into(),
        duration: duration_secs,
        reason: reason.into(),
      },
    };

    let post_data = serde_json::to_string(&post_data).unwrap();

    TwitchHttpRequest::new(url)
      .header_authorisation(access_token.into(), AuthType::Bearer)
      .header_client_id(client_id.into())
      .json_content()
      .is_post(post_data)
      .run()
  }
}

#[derive(PartialEq, Clone, Debug)]
pub enum RequestType {
  Post(String),
  Delete,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AuthType {
  Bearer,
  OAuth,
}

impl AuthType {
  pub fn to_string(&self) -> String {
    match self {
      AuthType::Bearer => "Bearer",
      AuthType::OAuth => "OAuth",
    }
    .into()
  }
}

impl RequestType {
  pub fn apply(&self, handle: &mut Easy) {
    match self {
      RequestType::Post(data) => {
        handle.post(true).unwrap();
        handle.post_fields_copy(data.as_bytes()).unwrap();
      }
      RequestType::Delete => {
        let _ = handle.custom_request("DELETE");
      }
    }
  }
}

pub struct RequestBuilder {
  data: Vec<(String, String)>,
}

impl RequestBuilder {
  fn new() -> RequestBuilder {
    RequestBuilder { data: Vec::new() }
  }

  fn add_key_value<S: Into<String>, T: Into<String>>(mut self, key: S, value: T) -> RequestBuilder {
    self.data.push((key.into(), value.into()));
    self
  }

  fn build<S: Into<String>>(self, url: S) -> String {
    let mut request = url.into();

    if !self.data.is_empty() {
      request = format!("{}?", request);
    }

    for (key, value) in self.data {
      request = format!("{}&{}={}", request, key, value);
    }

    request
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Header {
  Auth((AuthType, String)),
  ClientId(String),
  ContentJson,
  ContentUrlEncoded,
}

impl Header {
  pub fn generate(&self) -> String {
    match self {
      Header::Auth((auth_type, token)) => {
        format!("Authorization: {} {}", auth_type.to_string(), token)
      }
      Header::ClientId(id) => {
        format!("Client-Id: {}", id)
      }
      Header::ContentJson => {
        format!("Content-Type: application/json")
      }
      Header::ContentUrlEncoded => {
        format!("Content-Type: application/x-www-form-urlencoded")
      }
    }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TwitchHttpRequest {
  url: String,
  headers: Vec<Header>,
  request_type: Option<RequestType>,
}

impl TwitchHttpRequest {
  pub fn new<S: Into<String>>(url: S) -> TwitchHttpRequest {
    TwitchHttpRequest {
      url: url.into(),
      headers: Vec::new(),
      request_type: None,
    }
  }

  #[must_use]
  pub fn full_auth<S: Into<String>, T: Into<String>>(
    self,
    access_token: S,
    client_id: T,
  ) -> TwitchHttpRequest {
    self
      .header_authorisation(access_token, AuthType::Bearer)
      .header_client_id(client_id)
  }

  #[must_use]
  pub fn add_header(mut self, header: Header) -> TwitchHttpRequest {
    self.headers.push(header);
    self
  }

  #[must_use]
  pub fn header_authorisation<S: Into<String>>(
    mut self,
    token: S,
    auth_type: AuthType,
  ) -> TwitchHttpRequest {
    self.headers.push(Header::Auth((auth_type, token.into())));
    self
  }

  #[must_use]
  pub fn header_client_id<S: Into<String>>(mut self, client_id: S) -> TwitchHttpRequest {
    self.headers.push(Header::ClientId(client_id.into()));
    self
  }

  #[must_use]
  pub fn json_content(mut self) -> TwitchHttpRequest {
    self.headers.push(Header::ContentJson);
    self
  }

  #[must_use]
  pub fn url_encoded_content(mut self) -> TwitchHttpRequest {
    self.headers.push(Header::ContentUrlEncoded);
    self
  }

  #[must_use]
  pub fn is_delete(mut self) -> TwitchHttpRequest {
    self.request_type = Some(RequestType::Delete);
    self
  }

  #[must_use]
  pub fn is_post<S: Into<String>>(mut self, data: S) -> TwitchHttpRequest {
    self.request_type = Some(RequestType::Post(data.into()));
    self
  }

  pub fn update_token<S: Into<String>>(&mut self, new_token: S) {
    for header in &mut self.headers {
      if let Header::Auth((_, ref mut token)) = header {
        *token = new_token.into();
        break;
      }
    }
  }

  pub fn run(&self) -> Result<String, EventSubError> {
    let mut data = Vec::new();

    info!("Running curl command with:");
    info!("    url: {}", self.url);
    let mut handle = Easy::new();
    {
      handle.url(&self.url).unwrap();
      if let Some(request) = &self.request_type {
        request.apply(&mut handle);
      }

      let mut headers = List::new();
      for header in &self.headers {
        headers.append(&header.generate()).unwrap();
      }

      handle.http_headers(headers).unwrap();

      let mut handle = handle.transfer();
      // getting data back
      // idk why its called write function
      // that silly
      // we are reading whats coming back
      let _ = handle.write_function(|new_data| {
        data.extend_from_slice(new_data);
        Ok(new_data.len())
      });

      if let Err(e) = handle.perform() {
        if let Ok(error) = serde_json::from_str::<Validation>(&e.to_string()) {
          if error.is_error() {
            if error.status.unwrap() == 401 {
              // Regen access token
              // Re run the query
              return Err(EventSubError::TokenRequiresRefreshing(self.to_owned()));
            }
            error!("Converting result from curl request to validation failed!");
            return Err(EventSubError::InvalidOauthToken(error.error_msg()));
          }
        }
        error!("Curl error: {}", e);
        return Err(EventSubError::CurlFailed(e));
      }
    }

    Ok(String::from_utf8_lossy(&data).to_string())
  }
}
