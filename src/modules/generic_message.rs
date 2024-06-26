use crate::{
  modules::messages::{MessageData, RaidData},
  Condition, Deserialise, EventSubError, Serialise, Subscription, Token,
};

use super::messages::*;

#[derive(Deserialise)]
pub struct NewAccessTokenResponse {
  pub access_token: String,
  pub expires_in: u32,
  token_type: String,
  pub refresh_token: Option<String>,
  scope: Option<Vec<String>>,
}

impl NewAccessTokenResponse {
  pub fn _get_token_from_data(raw_data: &str) -> Result<Token, EventSubError> {
    serde_json::from_str::<NewAccessTokenResponse>(raw_data)
      .map(|validation| {
        Token::new_user_token(
          validation.access_token,
          validation.refresh_token.unwrap(),
          validation.expires_in as f32,
        )
      })
      .map_err(|e| EventSubError::AuthorisationError(e.to_string()))
  }
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Validation {
  client_id: Option<String>,
  login: Option<String>,
  pub scopes: Option<Vec<String>>,
  user_id: Option<String>,
  pub expires_in: Option<u32>,
  pub status: Option<u32>,
  message: Option<String>,
}

impl Validation {
  pub fn is_error(&self) -> bool {
    self.status.is_some()
  }

  pub fn error_msg(&self) -> String {
    if self.is_error() {
      format!(
        "status: {}, message: {}",
        self.status.unwrap(),
        self.message.clone().unwrap()
      )
    } else {
      panic!("Validation Error message requested, when it isnt a error!");
    }
  }
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct TimeoutRequestData {
  pub user_id: String,
  pub duration: u32,
  pub reason: String,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct SendTimeoutRequest {
  pub data: TimeoutRequestData,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct SendMessage {
  pub broadcaster_id: String,
  pub sender_id: String,
  pub message: String,
  pub reply_parent_message_id: Option<String>,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Transport {
  pub method: String,
  pub session_id: String,
}

impl Transport {
  pub fn new<S: Into<String>>(session_id: S) -> Transport {
    Transport {
      method: "websocket".to_string(),
      session_id: session_id.into(),
    }
  }
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Session {
  pub id: String,
  pub status: String,
  pub connected_at: String,
  pub keepalive_timeout_seconds: Option<u32>,
  pub reconnect_url: Option<String>, // is null
  pub recovery_url: Option<String>,  // is null
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct GMSubscription {
  pub id: String,
  pub status: Option<String>,
  #[serde(rename = "type")]
  pub kind: String,
  pub version: String,
  pub cost: i32,
  pub condition: Option<Condition>,
  pub transport: Transport,
  pub created_at: String,
  pub event: Option<Event>,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Mention {
  user_id: String,
  user_login: String,
  user_name: String,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Emote {
  id: String,
  emote_set_id: String,
  owner_id: String,
  format: Vec<String>,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct CheerMote {
  prefix: String,
  bits: u32,
  tier: u32,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Fragments {
  #[serde(rename = "type")]
  kind: String,
  text: String,
  cheermote: Option<CheerMote>,
  emote: Option<Emote>,
  mention: Option<Mention>,
}

impl Fragments {
  pub fn _is_text(&self) -> bool {
    self.kind == "text"
  }

  pub fn is_mention(&self) -> bool {
    self.kind == "mention"
  }

  pub fn text(&self) -> String {
    self.text.to_string()
  }
}

impl Message {
  pub fn get_written_message(&self) -> Option<String> {
    let mut text = None;
    for fragment in &self.fragments {
      if !fragment.is_mention() {
        if let Some(ref mut text) = text {
          *text = format!("{} {}", text, fragment.text());
        } else {
          text = Some(fragment.text().to_string().trim().to_string());
        }
      }
    }
    text
  }
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Message {
  pub text: String,
  pub fragments: Vec<Fragments>,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Badge {
  set_id: String,
  id: String,
  info: String,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Reply {
  #[serde(flatten)]
  thread: ThreadUser,
  #[serde(flatten)]
  parent_user: ParentUser,
  parent_message_id: String,
  parent_message_body: String,
  thread_message_id: String,
}

#[derive(Serialise, Deserialise, Debug, Clone, PartialEq)]
pub struct Reward {
  pub id: String,
  pub title: String,
  pub prompt: String,
  pub cost: u32,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Cheer {
  bits: u32,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
#[serde(untagged)]
pub enum Event {
  ChatMessage(MessageData),
  Raid(RaidData),
  PointsCustomRewardRedeem(CustomPointsRewardRedeemData),
  AdBreakBegin(AdBreakBeginData),
  Subscribe(SubscribeData),
  SubscriptionGift(GiftData),
  SubscriptionMessage(SubscribeMessageData),
  Cheer(CheerData),
  ChannelPointsAutoRewardRedeem(AutoRewardData),
  PollBegin(PollBeginData),
  PollProgress(PollProgressData),
  PollEnd(PollEndData),
  PredictionBegin(PredictionBeginData),
  PredictionProgress(PredicitonProgressData),
  PredictionLock(PredictionLockData),
  PredictionEnd(PredicitionEndData),
  HypeTrainBegin(HypeTrainBeginData),
  HypeTrainProgress(HypeTrainProgressData),
  HypeTrainEnd(HypeTrainEndData),
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct Payload {
  pub session: Option<Session>,
  pub subscription: Option<GMSubscription>,
  pub event: Option<Event>,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct MetaData {
  pub message_id: String,
  pub message_type: String,
  pub message_timestamp: String,
  pub subscription_type: Option<String>,
  pub subscription_version: Option<String>,
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct GenericMessage {
  pub metadata: MetaData,
  pub payload: Option<Payload>,
  pub subscription_type: Option<String>,
  pub subscription_version: Option<String>,
}

pub enum EventMessageType {
  Welcome,
  KeepAlive,
  Notification,
  //  Reconnect,
  Unknown,
}

impl EventMessageType {
  pub fn from_string(t: &str) -> EventMessageType {
    match t {
      "session_welcome" => EventMessageType::Welcome,
      "session_keepalive" => EventMessageType::KeepAlive,
      "notification" => EventMessageType::Notification,
      _ => EventMessageType::Unknown,
    }
  }
}

impl GenericMessage {
  pub fn event_type(&self) -> EventMessageType {
    EventMessageType::from_string(&self.metadata.message_type)
  }

  pub fn subscription_type(&self) -> Subscription {
    Subscription::from_string(&self.metadata.subscription_type.clone().unwrap()).unwrap()
  }
}
