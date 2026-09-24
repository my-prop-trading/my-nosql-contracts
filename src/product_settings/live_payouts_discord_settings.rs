use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

// Live Payouts Discord publishing credentials (PROP25-2472). Editable from the back
// office (Live Payouts → Settings → Publishing) and read live by the services that
// publish the feed: discord-bridge-flows-grpc uses the bot token + guild id, and
// live-payouts-flows-grpc uses the channel id. Kept in its OWN "live-payouts"
// partition of the shared product-settings table so it does not mix with the
// affiliate/integration settings.
#[enum_model(partition_key: "live-payouts", row_key: "discord")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LivePayoutsDiscordSettingsModel {
    /// Raw bot token of the OneFunded Discord app (bot-token transport). On alpha this
    /// is a test bot; on prod the real token. Read by discord-bridge-flows-grpc.
    pub bot_token: Option<String>,
    /// Guild (server) id, used only to assemble the message permalink
    /// (https://discord.com/channels/{guild}/{channel}/{message}). Read by
    /// discord-bridge-flows-grpc.
    pub guild_id: Option<String>,
    /// Channel id the live-payouts feed posts to. Read by live-payouts-flows-grpc.
    pub channel_id: Option<String>,
}
