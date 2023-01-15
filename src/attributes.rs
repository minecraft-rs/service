use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinecraftPrivilegeItem {
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftPrivileges {
    pub online_chat: MinecraftPrivilegeItem,
    pub multiplayer_server: MinecraftPrivilegeItem,
    pub multiplayer_realms: MinecraftPrivilegeItem,
    pub telemetry: MinecraftPrivilegeItem,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftProfanityFilter {
    pub profanity_filter_on: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftBanScopeItem {
    pub ban_id: String,
    pub expires: u64,
    pub reason: String,
    pub reason_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct MinecraftBanScopes {
    pub multiplayer: Option<MinecraftBanScopeItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftBanStatus {
    pub banned_scopes: MinecraftBanScopes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftPlayerAttributes {
    pub privileges: MinecraftPrivileges,
    pub profanity_filter_preferences: MinecraftProfanityFilter,
    pub ban_status: MinecraftBanStatus,
}
