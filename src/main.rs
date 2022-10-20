mod telegram;

#[macro_use]
extern crate dotenv_codegen;

use chrono::prelude::DateTime;
use chrono::Local;
use colored::Colorize;
use dirs;
use dotenv::dotenv;
use futures::stream::{self, StreamExt};
use std::time::{Duration, UNIX_EPOCH};
use telegram::{
    api::{Chat, ChatType, MessageSender, User, UserStatus},
    manager::*,
};

trait PrettyPrint {
    fn pretty_format(&self) -> String;
}

impl PrettyPrint for DateTime<Local> {
    fn pretty_format(&self) -> String {
        let now: DateTime<Local> = Local::now();

        if self.date() == now.date() {
            self.format("%H:%M ").to_string()
        } else {
            self.format("%m.%d ").to_string()
        }
    }
}

impl PrettyPrint for UserStatus {
    fn pretty_format(&self) -> String {
        match self {
            UserStatus::UserStatusEmpty | UserStatus::UserStatusOffline => "".to_owned(),
            UserStatus::UserStatusLastMonth => "(month) ".to_owned(),
            UserStatus::UserStatusLastWeek => "(week) ".to_owned(),
            UserStatus::UserStatusOnline => "(online) ".to_owned(),
            UserStatus::UserStatusRecently => "(recently) ".to_owned(),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let manager = Manager::new();
    manager
        .authorize(
            &format!(
                "{}/.config/rust-messenger",
                dirs::home_dir()
                    .expect("Can't get home directory")
                    .into_os_string()
                    .to_str()
                    .expect("Can't convert home directory to string")
            ),
            "Terminal",
        )
        .await;

    let chat_ids = manager.get_chats(40).await.chat_ids;

    let chats = stream::iter(chat_ids)
        .then(|id| manager.get_chat(id))
        .collect::<Vec<Chat>>()
        .await;

    let user_ids = chats
        .iter()
        .map(|chat| {
            if let telegram::api::MessageSender::MessageSenderUser(user) =
                &chat.last_message.sender_id
            {
                Some(user.user_id)
            } else {
                None
            }
        })
        .flat_map(|user| user)
        .collect::<Vec<i64>>();

    let users = stream::iter(user_ids)
        .then(|id| manager.get_user(id))
        .collect::<Vec<User>>()
        .await;

    // println!("{:?}", chats);
    // println!("{:?}", users);

    for chat in chats.iter().rev() {
        let title = format!("{} ", chat.title);

        let unread_count = if chat.unread_count > 0 {
            format!("({}+) ", chat.unread_count)
        } else {
            "".to_string()
        };

        let outgoing = if chat.last_message.is_outgoing {
            "🖊  "
        } else {
            "📖  "
        };

        let user = if let MessageSender::MessageSenderUser(id) = &chat.last_message.sender_id {
            match chat.chat_type {
                ChatType::ChatTypeBasicGroup | ChatType::ChatTypeSupergroup => {
                    let user = users.iter().find(|user| user.id == id.user_id).expect("Can't find user info");
                    format!(
                        "{} ",
                        format!("{} {}", user.first_name, user.last_name).trim()
                    )
                }

                _ => "".to_string(),
            }
        } else {
            "".to_string()
        };

        let user_status = if let MessageSender::MessageSenderUser(id) = &chat.last_message.sender_id
        {
            match chat.chat_type {
                ChatType::ChatTypePrivate => {
                    let user = users.iter().find(|user| user.id == id.user_id).expect("Can't find user info");
                    user.status.pretty_format()
                }

                _ => "".to_string(),
            }
        } else {
            "".to_string()
        };

        println!(
            "{}{}{}{}{}",
            title.bold().blue(),
            user.bright_black(),
            outgoing.bold().bright_black(),
            user_status.bold().blue(),
            unread_count.bold().magenta()
        );

        let body = match &chat.last_message.content {
            telegram::api::MessageContent::MessageText(body) => format!("{}", body.text.text),
            it @ _ => format!("{:?}", it).underline().to_string(),
        };

        let epoch = UNIX_EPOCH + Duration::from_secs(chat.last_message.date.try_into().expect("Can't convert unixtime to Duration"));
        let datetime = DateTime::<Local>::from(epoch).pretty_format();

        println!("{}{}\n", datetime.bold().bright_black(), body);
    }

    loop {}
}
