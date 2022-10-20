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
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum AuthorizationStateType {
    AuthorizationStateWaitTdlibParameters,
    AuthorizationStateWaitPhoneNumber,
    AuthorizationStateWaitCode,
    AuthorizationStateReady,
    AuthorizationStateClosing
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
pub struct TgEvent {
    #[serde(rename = "@extra")] pub extra: Option<String>,
    #[serde(flatten)] pub data: TgEventData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "@type", rename_all = "camelCase")]
pub enum TgEventData {
    // Util
    Error(Error),
    Ok,

    // Auth
    AuthorizationStateWaitTdlibParameters,
    AuthorizationStateWaitPhoneNumber,
    AuthorizationStateWaitCode,
    AuthorizationStateReady,

    // Data
    Chats(Chats),
    Chat(Chat),
    User(User),

    // Requests
    CheckAuthenticationCode(CheckAuthenticationCode),

    // Setters
    SetAuthenticationPhoneNumber(SetAuthenticationPhoneNumber),
    SetTdlibParameters(TdlibParameters),

    // Getters
    GetAllChats,
    GetAuthorizationState,
    GetChats(GetChats),
    GetChat(GetChat),
    GetUser(GetUser),
    LoadChats(LoadChats),

    // Updates
    UpdateActiveNotifications, 
    UpdateAnimatedEmojiMessageClicked, 
    UpdateAnimationSearchParameters, 
    UpdateAuthorizationState(UpdateAuthorizationState), 
    UpdateBasicGroup, 
    UpdateBasicGroupFullInfo, 
    UpdateCall, 
    UpdateChatAction, 
    UpdateChatActionBar, 
    UpdateChatDefaultDisableNotification, 
    UpdateChatDraftMessage, 
    UpdateChatFilters, 
    UpdateChatHasProtectedContent, 
    UpdateChatHasScheduledMessages, 
    UpdateChatIsBlocked, 
    UpdateChatIsMarkedAsUnread, 
    UpdateChatLastMessage, 
    UpdateChatMember, 
    UpdateChatMessageSender, 
    UpdateChatMessageTtl, 
    UpdateChatNotificationSettings, 
    UpdateChatOnlineMemberCount, 
    UpdateChatPendingJoinRequests, 
    UpdateChatPermissions, 
    UpdateChatPhoto, 
    UpdateChatPosition, 
    UpdateChatReadInbox, 
    UpdateChatReadOutbox, 
    UpdateChatReplyMarkup, 
    UpdateChatTheme, 
    UpdateChatThemes, 
    UpdateChatTitle, 
    UpdateChatUnreadMentionCount, 
    UpdateChatVideoChat, 
    UpdateConnectionState, 
    UpdateDeleteMessages, 
    UpdateDiceEmojis, 
    UpdateFavoriteStickers, 
    UpdateFile, 
    UpdateFileGenerationStart, 
    UpdateFileGenerationStop, 
    UpdateGroupCall, 
    UpdateGroupCallParticipant, 
    UpdateHavePendingNotifications, 
    UpdateInstalledStickerSets, 
    UpdateLanguagePackStrings, 
    UpdateMessageContent, 
    UpdateMessageContentOpened, 
    UpdateMessageEdited, 
    UpdateMessageInteractionInfo, 
    UpdateMessageIsPinned, 
    UpdateMessageLiveLocationViewed, 
    UpdateMessageMentionRead, 
    UpdateMessageSendAcknowledged, 
    UpdateMessageSendFailed, 
    UpdateMessageSendSucceeded, 
    UpdateNewCallbackQuery, 
    UpdateNewCallSignalingData, 
    UpdateNewChat, 
    UpdateNewChatJoinRequest, 
    UpdateNewChosenInlineResult, 
    UpdateNewCustomEvent, 
    UpdateNewCustomQuery, 
    UpdateNewInlineCallbackQuery, 
    UpdateNewInlineQuery, 
    UpdateNewMessage, 
    UpdateNewPreCheckoutQuery, 
    UpdateNewShippingQuery, 
    UpdateNotification, 
    UpdateNotificationGroup, 
    UpdateOption(UpdateOption), 
    UpdatePoll, 
    UpdatePollAnswer, 
    UpdateRecentStickers, 
    UpdateSavedAnimations, 
    UpdateScopeNotificationSettings, 
    UpdateSecretChat, 
    UpdateSelectedBackground, 
    UpdateServiceNotification, 
    UpdateStickerSet, 
    UpdateSuggestedActions, 
    UpdateSupergroup, 
    UpdateSupergroupFullInfo, 
    UpdateTermsOfService, 
    UpdateTrendingStickerSets, 
    UpdateUnreadChatCount, 
    UpdateUnreadMessageCount, 
    UpdateUser(UpdateUser),
    UpdateUserFullInfo, 
    UpdateUserPrivacySettingRules, 
    UpdateUsersNearby, 
    UpdateUserStatus(UpdateUserStatus),
    UpdateDefaultReactionType,
    UpdateAttachmentMenuBots,
    UpdateFileDownloads,
    UpdateActiveEmojiReactions,
    UpdateChatAvailableReactions,
    UpdateChatUnreadReactionCount
}