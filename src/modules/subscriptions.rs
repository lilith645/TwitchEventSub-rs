use crate::modules::generic_message::*;
use crate::TwitchKeys;

#[derive(Clone)]
pub enum SubscriptionPermission {
  UserUpdate,
  ChannelFollow,
  ChatMessage,
  CustomRedeem,
  BanTimeoutUser,
  DeleteMessage,
  AdBreakBegin,
}

impl SubscriptionPermission {
  pub fn tag(&self) -> String {
    match self {
      SubscriptionPermission::UserUpdate => "user.update",
      SubscriptionPermission::ChannelFollow => "channel.follow",
      SubscriptionPermission::ChatMessage => "channel.chat.message",
      SubscriptionPermission::CustomRedeem => "channel.channel_points_custom_reward_redemption.add",
      SubscriptionPermission::AdBreakBegin => "channel.ad_break.begin",
      _ => "",
    }
    .to_string()
  }

  pub fn required_scope(&self) -> String {
    match self {
      SubscriptionPermission::ChannelFollow => "moderator:read:followers",
      SubscriptionPermission::ChatMessage => "user:read:chat+user:write:chat",
      SubscriptionPermission::CustomRedeem => "channel:read:redemptions",
      SubscriptionPermission::BanTimeoutUser => "moderator:manage:banned_users",
      SubscriptionPermission::DeleteMessage => "moderator:manage:chat_messages",
      SubscriptionPermission::AdBreakBegin => "channel:read:ads",
      _ => "",
    }
    .to_string()
  }

  pub fn construct_data(&self, session_id: &str, twitch_keys: &TwitchKeys) -> EventSubscription {
    let transport = Transport::new(session_id);
    match self {
      SubscriptionPermission::UserUpdate => EventSubscription {
        kind: self.tag(),
        version: "1".to_string(),
        condition: Condition {
          user_id: Some(twitch_keys.broadcaster_account_id.to_owned()),
          moderator_user_id: None,
          broadcaster_user_id: None,
          reward_id: None,
        },
        transport,
      },
      SubscriptionPermission::ChannelFollow => EventSubscription {
        kind: self.tag(),
        version: "2".to_string(),
        condition: Condition {
          broadcaster_user_id: Some(twitch_keys.broadcaster_account_id.to_owned()),
          moderator_user_id: Some(twitch_keys.broadcaster_account_id.to_owned()),
          user_id: Some(twitch_keys.broadcaster_account_id.to_owned()),
          reward_id: None,
        },
        transport,
      },
      SubscriptionPermission::ChatMessage => EventSubscription {
        kind: self.tag(),
        version: "1".to_string(),
        condition: Condition {
          broadcaster_user_id: Some(twitch_keys.broadcaster_account_id.to_owned()),
          moderator_user_id: None,
          user_id: Some(twitch_keys.broadcaster_account_id.to_owned()),
          reward_id: None,
        },
        transport,
      },
      SubscriptionPermission::CustomRedeem => EventSubscription {
        kind: self.tag(),
        version: "1".to_string(),
        condition: Condition {
          user_id: None,
          moderator_user_id: None,
          broadcaster_user_id: Some(twitch_keys.broadcaster_account_id.to_owned()),
          reward_id: None,
        },
        transport,
      },
      SubscriptionPermission::AdBreakBegin => EventSubscription {
        kind: self.tag(),
        version: "1".to_owned(),
        condition: Condition {
          user_id: None,
          moderator_user_id: None,
          broadcaster_user_id: Some(twitch_keys.broadcaster_account_id.to_owned()),
          reward_id: None,
        },
        transport,
      },
      _ => EventSubscription {
        kind: "".to_owned(),
        version: "1".to_owned(),
        condition: Condition {
          user_id: None,
          moderator_user_id: None,
          broadcaster_user_id: None,
          reward_id: None,
        },
        transport,
      },
    }
  }
}