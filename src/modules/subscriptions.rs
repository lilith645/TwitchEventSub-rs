use crate::modules::generic_message::*;
use crate::TwitchKeys;

use crate::{Deserialise, Serialise};

macro_rules! from_string {
    ($enum_name:ident { $($variant:ident),* }) => {
        pub fn from_string(t: &str) -> Option<$enum_name> {
            $(
                if $enum_name::$variant.tag() == t {
                    return Some($enum_name::$variant);
                }
            )*
            None
        }
    };
}

#[derive(Clone, Debug)]
pub enum Subscription {
  UserUpdate,
  ChannelFollow,
  ChannelRaid,
  ChannelUpdate,
  ChannelSubscribe,
  ChannelSubscriptionEnd,
  ChannelSubscriptionGift,
  ChannelSubscriptionMessage,
  ChannelCheer,
  ChannelPointsCustomRewardRedeem,
  ChannelPointsAutoRewardRedeem,
  ChannelPollBegin,
  ChannelPollProgress,
  ChannelPollEnd,
  ChannelPredictionBegin,
  ChannelPredictionProgress,
  ChannelPredictionLock,
  ChannelPredictionEnd,
  ChannelGoalBegin,
  ChannelGoalProgress,
  ChannelGoalEnd,
  ChannelHypeTrainBegin,
  ChannelHypeTrainProgress,
  ChannelHypeTrainEnd,
  ChannelShoutoutCreate,
  ChannelShoutoutReceive,
  ChatMessage,
  BanTimeoutUser,
  DeleteMessage,
  AdBreakBegin,
  Custom((String, String, EventSubscription)),
}

#[derive(Serialise, Deserialise, Debug, Clone)]
pub struct EventSubscription {
  #[serde(rename = "type")]
  pub kind: String,
  pub version: String,
  pub condition: Condition,
  pub transport: Transport,
}

impl Subscription {
  from_string!(Subscription {
    UserUpdate,
    ChannelFollow,
    ChannelRaid,
    ChannelUpdate,
    ChannelSubscribe,
    ChannelSubscriptionEnd,
    ChannelSubscriptionGift,
    ChannelSubscriptionMessage,
    ChannelCheer,
    ChannelPointsCustomRewardRedeem,
    ChannelPointsAutoRewardRedeem,
    ChannelPollBegin,
    ChannelPollProgress,
    ChannelPollEnd,
    ChannelPredictionBegin,
    ChannelPredictionProgress,
    ChannelPredictionLock,
    ChannelPredictionEnd,
    ChannelGoalBegin,
    ChannelGoalProgress,
    ChannelGoalEnd,
    ChannelHypeTrainBegin,
    ChannelHypeTrainProgress,
    ChannelHypeTrainEnd,
    ChannelShoutoutCreate,
    ChannelShoutoutReceive,
    ChatMessage,
    BanTimeoutUser,
    DeleteMessage,
    AdBreakBegin
  });

  fn details(&self) -> (String, String, String) {
    let details = match self {
      Subscription::UserUpdate => ("user.update", "", "1"),
      Subscription::ChannelFollow => ("channel.follow", "moderator:read:followers", "2"),
      Subscription::ChannelRaid => ("channel.raid", "", "1"),
      Subscription::ChatMessage => (
        "channel.chat.message",
        "user:read:chat+user:write:chat",
        "1",
      ),
      Subscription::ChannelPointsCustomRewardRedeem => (
        "channel.channel_points_custom_reward_redemption.add",
        "channel:read:redemptions",
        "1",
      ),
      Subscription::AdBreakBegin => ("channel.ad_break.begin", "channel:read:ads", "1"),
      Subscription::ChannelUpdate => ("channel.update", "", "2"),
      Subscription::BanTimeoutUser => ("", "moderator:manage:banned_users", ""),
      Subscription::DeleteMessage => ("", "moderator:manage:chat_messages", ""),
      Subscription::ChannelSubscribe => ("channel.subscribe", "channel:read:subscriptions", "1"),
      Subscription::ChannelSubscriptionEnd => (
        "channel.subscription.end",
        "channel:read:subscriptions",
        "1",
      ),
      Subscription::ChannelSubscriptionGift => (
        "channel.subscription.gift",
        "channel:read:subscriptions",
        "1",
      ),
      Subscription::ChannelSubscriptionMessage => (
        "channel.subscription.message",
        "channel:read:subscriptions",
        "1",
      ),
      Subscription::ChannelCheer => ("channel.cheer", "bits:read", "1"),
      Subscription::ChannelPointsAutoRewardRedeem => (
        "channel.channel_points_automatic_reward_redemption.add",
        "channel:read:redemptions",
        "1",
      ),
      Subscription::ChannelPollBegin => (
        "channel.poll.begin",
        "channel:read:polls+channel:manage:polls",
        "1",
      ),
      Subscription::ChannelPollProgress => (
        "channel.poll.progress",
        "channel:read:polls+channel:manage:polls",
        "1",
      ),
      Subscription::ChannelPollEnd => (
        "channel.poll.end",
        "channel:read:polls+channel:manage:polls",
        "1",
      ),
      Subscription::ChannelPredictionBegin => (
        "channel.prediction.begin",
        "channel:read:predictions+channel:manage:predictions",
        "1",
      ),
      Subscription::ChannelPredictionProgress => (
        "channel.prediction.progress",
        "channel:read:predictions+channel:manage:predictions",
        "1",
      ),
      Subscription::ChannelPredictionLock => (
        "channel.prediction.lock",
        "channel:read:predictions+channel:manage:predictions",
        "1",
      ),
      Subscription::ChannelPredictionEnd => (
        "channel.prediction.end",
        "channel:read:predictions+channel:manage:predictions",
        "1",
      ),
      Subscription::ChannelGoalBegin => ("channel.goal.begin", "channel:read:goals", "1"),
      Subscription::ChannelGoalProgress => ("channel.goal.progress", "channel:read:goals", "1"),
      Subscription::ChannelGoalEnd => ("channel.goal.end", "channel:read:goals", "1"),
      Subscription::ChannelHypeTrainBegin => {
        ("channel.hype_train.begin", "channel:read:hype_train", "1")
      }
      Subscription::ChannelHypeTrainProgress => (
        "channel.hype_train.progress",
        "channel:read:hype_train",
        "1",
      ),
      Subscription::ChannelHypeTrainEnd => {
        ("channel.hype_train.end", "channel:read:hype_train", "1")
      }
      Subscription::ChannelShoutoutCreate => (
        "channel.shoutout.create",
        "moderator:read:shoutouts+moderator:manage:shoutouts",
        "1",
      ),
      Subscription::ChannelShoutoutReceive => (
        "channel.shoutout.receive",
        "moderator:read:shoutouts+moderator:manage:shoutouts",
        "1",
      ),
      Subscription::Custom((tag, scope, ..)) => (tag.as_str(), scope.as_str(), ""),
    };

    (
      details.0.to_owned(),
      details.1.to_owned(),
      details.2.to_owned(),
    )
  }

  pub fn tag(&self) -> String {
    self.details().0
  }

  pub fn required_scope(&self) -> String {
    self.details().1
  }

  pub fn version(&self) -> String {
    self.details().2
  }

  pub fn construct_data(&self, session_id: &str, twitch_keys: &TwitchKeys) -> EventSubscription {
    let transport = Transport::new(session_id);

    let event_subscription = EventSubscription::new(self, transport);
    let condition =
      Condition::new().broadcaster_user_id(twitch_keys.broadcaster_account_id.to_owned());

    match self {
      Subscription::UserUpdate => event_subscription
        .condition(Condition::new().user_id(twitch_keys.broadcaster_account_id.to_owned())),
      Subscription::ChannelFollow => event_subscription.condition(
        condition
          .moderator_user_id(twitch_keys.broadcaster_account_id.to_owned())
          .user_id(twitch_keys.broadcaster_account_id.to_owned()),
      ),
      Subscription::ChatMessage => event_subscription
        .condition(condition.user_id(twitch_keys.broadcaster_account_id.to_owned())),
      Subscription::ChannelPointsCustomRewardRedeem => event_subscription.condition(condition),
      Subscription::AdBreakBegin => event_subscription.condition(condition),
      Subscription::ChannelRaid => event_subscription.condition(condition),
      Subscription::ChannelUpdate => event_subscription.condition(condition),
      Subscription::ChannelSubscribe => event_subscription.condition(condition),
      Subscription::ChannelSubscriptionEnd => event_subscription.condition(condition),
      Subscription::ChannelSubscriptionGift => event_subscription.condition(condition),
      Subscription::ChannelSubscriptionMessage => event_subscription.condition(condition),
      Subscription::Custom((_, _, event)) => {
        let mut event = event.to_owned();
        event = event.transport(Transport::new(session_id));
        event.to_owned()
      }
      _ => event_subscription,
    }
  }
}

impl EventSubscription {
  pub fn new(event: &Subscription, transport: Transport) -> EventSubscription {
    EventSubscription {
      kind: event.tag(),
      version: event.version(),
      condition: Condition::new(),
      transport,
    }
  }

  pub fn transport(mut self, transport: Transport) -> EventSubscription {
    self.transport = transport;
    self
  }

  pub fn condition(mut self, condition: Condition) -> EventSubscription {
    self.condition = condition;
    self
  }
}

#[derive(Serialise, Deserialise, Debug, Clone, Default)]
pub struct Condition {
  pub user_id: Option<String>,
  pub moderator_user_id: Option<String>,
  pub broadcaster_user_id: Option<String>,
  pub reward_id: Option<String>,
  pub from_broadcaster_user_id: Option<String>,
  pub to_broadcaster_user_id: Option<String>,
  #[serde(rename = "organisation_id")]
  pub organisation_id: Option<String>,
  pub category_id: Option<String>,
  pub campaign_id: Option<String>,
  pub extension_client_id: Option<String>,
}

impl Condition {
  pub fn new() -> Condition {
    Condition {
      ..Default::default()
    }
  }

  pub fn user_id<S: Into<String>>(mut self, user_id: S) -> Condition {
    self.user_id = Some(user_id.into());
    self
  }

  pub fn moderator_user_id<S: Into<String>>(mut self, moderator_user_id: S) -> Condition {
    self.moderator_user_id = Some(moderator_user_id.into());
    self
  }

  pub fn broadcaster_user_id<S: Into<String>>(mut self, broadcaster_user_id: S) -> Condition {
    self.broadcaster_user_id = Some(broadcaster_user_id.into());
    self
  }

  pub fn reward_id<S: Into<String>>(mut self, reward_id: S) -> Condition {
    self.reward_id = Some(reward_id.into());
    self
  }

  pub fn from_broadcaster_user_id<S: Into<String>>(
    mut self,
    from_broadcaster_user_id: S,
  ) -> Condition {
    self.from_broadcaster_user_id = Some(from_broadcaster_user_id.into());
    self
  }

  pub fn to_broadcaster_user_id<S: Into<String>>(mut self, to_broadcaster_user_id: S) -> Condition {
    self.to_broadcaster_user_id = Some(to_broadcaster_user_id.into());
    self
  }
}
