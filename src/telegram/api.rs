use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    code: i32,
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueBoolean {
    value: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueInteger {
    value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionValueString {
    value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum OptionValue {
    OptionValueBoolean(OptionValueBoolean),
    OptionValueEmpty,
    OptionValueInteger(OptionValueInteger),
    OptionValueString(OptionValueString)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOption {
    name: String,
    value: OptionValue
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "@type")]
pub enum AuthorizationStateType {
    #[serde(rename = "authorizationStateWaitTdlibParameters")]
    WithTdLibParameters,
    #[serde(rename = "authorizationStateWaitPhoneNumber")]
    WaitPhoneNumber,
    #[serde(rename = "authorizationStateWaitCode")]
    WaitCode,
    #[serde(rename = "authorizationStateReady")]
    Ready,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateAuthorizationState {
    pub authorization_state: AuthorizationStateType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetAuthenticationPhoneNumber {
    pub phone_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckAuthenticationCode {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub id: i64,
    pub status: UserStatus
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum UserStatus {
    UserStatusEmpty,
    UserStatusOnline,
    UserStatusOffline,
    UserStatusRecently,
    UserStatusLastWeek,
    UserStatusLastMonth
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserStatus {
    pub user_id: i64,
    pub status: UserStatus
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetChats {
    pub limit: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadChats {
    pub limit: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetChat {
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUser {
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chats {
    pub total_count: i32,
    pub chat_ids: Vec<i64>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormattedText {
    pub text: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageText {
    pub text: FormattedText
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum MessageContent {
    MessageAnimatedEmoji, 
    MessageAnimation, 
    MessageAudio, 
    MessageBasicGroupChatCreate, 
    MessageCall, 
    MessageChatAddMembers, 
    MessageChatChangePhoto, 
    MessageChatChangeTitle, 
    MessageChatDeleteMember, 
    MessageChatDeletePhoto, 
    MessageChatJoinByLink, 
    MessageChatJoinByRequest, 
    MessageChatSetTheme, 
    MessageChatSetTtl, 
    MessageChatUpgradeFrom, 
    MessageChatUpgradeTo, 
    MessageContact, 
    MessageContactRegistered, 
    MessageCustomServiceAction, 
    MessageDice, 
    MessageDocument, 
    MessageExpiredPhoto, 
    MessageExpiredVideo, 
    MessageGame, 
    MessageGameScore, 
    MessageInviteVideoChatParticipants, 
    MessageInvoice, 
    MessageLocation, 
    MessagePassportDataReceived, 
    MessagePassportDataSent, 
    MessagePaymentSuccessful, 
    MessagePaymentSuccessfulBot, 
    MessagePhoto, 
    MessagePinMessage, 
    MessagePoll, 
    MessageProximityAlertTriggered, 
    MessageScreenshotTaken, 
    MessageSticker, 
    MessageSupergroupChatCreate, 
    MessageText(MessageText),
    MessageUnsupported, 
    MessageVenue, 
    MessageVideo, 
    MessageVideoChatEnded, 
    MessageVideoChatScheduled, 
    MessageVideoChatStarted, 
    MessageVideoNote, 
    MessageVoiceNote, 
    MessageWebsiteConnected
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSenderUser {
    pub user_id: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageSenderChat {
    pub chat_id: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum MessageSender {
    MessageSenderUser(MessageSenderUser),
    MessageSenderChat(MessageSenderChat)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub is_outgoing: bool,
    pub date: i32,
    pub content: MessageContent,
    pub sender_id: MessageSender
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum ChatType {
    ChatTypeBasicGroup,
    ChatTypePrivate,
    ChatTypeSecret,
    ChatTypeSupergroup
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: i64,
    pub title: String,
    pub last_message: Message,
    pub is_marked_as_unread: bool,
    pub unread_count: i32,
    #[serde(rename = "type")] pub chat_type: ChatType
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TdlibParameters {
    pub use_test_dc: bool,
    pub database_directory: String,
    pub files_directory: String,
    pub use_file_database: bool,
    pub use_chat_info_database: bool,
    pub use_message_database: bool,
    pub use_secret_chats: bool,
    pub api_id: i32,
    pub api_hash: String,
    pub system_language_code: String,
    pub device_model: String,
    pub system_version: String,
    pub application_version: String,
    pub enable_storage_optimizer: bool,
    pub ignore_file_names: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TgExtra {
    #[serde(rename = "@extra")] pub extra: Option<String>,
    #[serde(flatten)] pub data: Tg
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum Tg {
    Error(Error),
    UpdateOption(UpdateOption),
    UpdateAuthorizationState(UpdateAuthorizationState),
    SetTdlibParameters(TdlibParameters),
    UpdateAnimationSearchParameters,
    Ok,
    UpdateSelectedBackground,
    UpdateFileDownloads,
    UpdateConnectionState,
    GetAuthorizationState,
    AuthorizationStateWaitTdlibParameters,
    AuthorizationStateWaitPhoneNumber,
    SetAuthenticationPhoneNumber(SetAuthenticationPhoneNumber),
    AuthorizationStateWaitCode,
    CheckAuthenticationCode(CheckAuthenticationCode),
    UpdateUser(UpdateUser),
    UpdateDefaultReactionType,
    UpdateAttachmentMenuBots,
    UpdateDiceEmojis,
    UpdateActiveEmojiReactions,
    UpdateChatThemes,
    UpdateScopeNotificationSettings,
    UpdateChatFilters,
    UpdateUnreadMessageCount,
    UpdateUnreadChatCount,
    UpdateHavePendingNotifications,
    UpdateUserStatus(UpdateUserStatus),
    UpdateSupergroup,
    UpdateNewChat,
    UpdateChatLastMessage,
    UpdateSupergroupFullInfo,
    UpdateNewMessage,
    GetChats(GetChats),
    LoadChats(LoadChats),
    GetAllChats,
    UpdateBasicGroup,
    UpdateChatPosition,
    Chats(Chats),
    UpdateUserFullInfo,
    UpdateChatActionBar,
    UpdateChatAvailableReactions,
    GetChat(GetChat),
    Chat(Chat),
    UpdateChatReadInbox,
    GetUser(GetUser),
    User(User),
    UpdateChatReadOutbox,
    UpdateGroupCall,
    UpdateMessageInteractionInfo,
    UpdateMessageContent,
    UpdateMessageEdited,
    UpdateBasicGroupFullInfo,
    UpdateChatVideoChat,
    UpdateDeleteMessages,
    AuthorizationStateReady,
    UpdateChatNotificationSettings,
    UpdateChatUnreadReactionCount,
    UpdateChatMessageTtl,
    UpdateChatAction
}