use super::{api::*, client::Client};
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use uuid::Uuid;

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Can't read from stdout");
    return line;
}

struct TgEventFuture {
    shared_state: Arc<Mutex<TgEventSharedState>>,
    uuid: String,
}

struct TgEventSharedState {
    message: Option<TgEventData>,
    waker: Option<Waker>,
}

impl Future for TgEventFuture {
    type Output = TgEventData;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().expect("Can't lock mutex of shared state");

        if let Some(message) = &shared_state.message {
            Poll::Ready(message.clone())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TgEventFuture {
    pub fn new(rx: spmc::Receiver<TgEvent>) -> Self {
        let shared_state = Arc::new(Mutex::new(TgEventSharedState {
            message: None,
            waker: None,
        }));

        let uuid = format!("{}", Uuid::new_v4());

        let thread_shared_state = shared_state.clone();
        let thread_uuid = uuid.clone();

        thread::spawn(move || loop {
            let message = rx.recv().expect("Can't receive message through smpc channel");

            if let Some(extra) = message.clone().extra {
                if extra != thread_uuid {
                    continue;
                }
            } else {
                continue;
            }

            let mut shared_state = thread_shared_state.lock().expect("Can't lock mutex for shared state");
            shared_state.message = Some(message.data);

            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }

            break;
        });

        TgEventFuture { shared_state, uuid }
    }
}

pub struct Manager {
    client: Arc<Client>,
    receiver: spmc::Receiver<TgEvent>,
}

impl Manager {
    pub fn new() -> Manager {
        let (mut tx, rx) = spmc::channel();

        let manager = Manager {
            client: Arc::new(Client::new(10.0)),
            receiver: rx,
        };

        let client_in_thread = manager.client.clone();

        thread::spawn(move || loop {
            if let Some(r) = client_in_thread.receive() {
                if let Ok(message) = serde_json::from_str::<TgEvent>(&r) {
                    tx.send(message).unwrap_or_default();
                } else {
                    println!("Can't parse message: {}", r);
                }
            }
        });

        manager
    }

    pub async fn authorize(&self, dir: &str, device: &str) {
        loop {
            match self.auth().await {
                TgEventData::AuthorizationStateWaitTdlibParameters => {
                    self.set_tdlib_parameters(TdlibParameters {
                        use_test_dc: false,
                        database_directory: format!("{}/database", dir),
                        files_directory: format!("{}/files", dir),
                        use_file_database: false,
                        use_chat_info_database: false,
                        use_message_database: false,
                        use_secret_chats: true,
                        api_id: dotenv!("API_ID").parse::<i32>().expect("Can't parse i32"),
                        api_hash: dotenv!("API_HASH").to_owned(),
                        system_language_code: String::from("en"),
                        device_model: device.to_owned(),
                        system_version: String::from(""),
                        application_version: String::from(env!("CARGO_PKG_VERSION")),
                        enable_storage_optimizer: true,
                        ignore_file_names: true,
                    })
                    .await
                }
                TgEventData::AuthorizationStateWaitCode => {
                    self.set_auth_code(input("code >").trim_end().to_owned())
                        .await
                }
                TgEventData::AuthorizationStateWaitPhoneNumber => {
                    self.set_auth_phone(input("phone >").trim_end().to_owned())
                        .await
                }
                TgEventData::AuthorizationStateReady => break,

                _ => panic!("Invalid auth type"),
            };
        }
    }

    async fn auth(&self) -> TgEventData {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgEvent {
                extra: Some(future.uuid.clone()),
                data: TgEventData::GetAuthorizationState,
            })
            .expect("Can't serialize message"),
        );

        future.await
    }

    pub async fn set_tdlib_parameters(&self, data: TdlibParameters) -> TgEventData {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgEvent {
                extra: Some(future.uuid.clone()),
                data: TgEventData::SetTdlibParameters(data),
            })
            .expect("Can't serialize message"),
        );

        future.await
    }

    pub async fn set_auth_phone(&self, data: String) -> TgEventData {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgEvent {
                extra: Some(future.uuid.clone()),
                data: TgEventData::SetAuthenticationPhoneNumber(SetAuthenticationPhoneNumber {
                    phone_number: data,
                }),
            })
            .expect("Can't serialize message"),
        );

        future.await
    }

    pub async fn set_auth_code(&self, data: String) -> TgEventData {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgEvent {
                extra: Some(future.uuid.clone()),
                data: TgEventData::CheckAuthenticationCode(CheckAuthenticationCode { code: data }),
            })
            .expect("Can't serialize message"),
        );

        future.await
    }

    pub async fn get_chats(&self, limit: i32) -> Chats {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgEvent {
                extra: Some(future.uuid.clone()),
                data: TgEventData::GetChats(GetChats { limit: limit }),
            })
            .expect("Can't serialize message"),
        );

        if let TgEventData::Chats(result) = future.await {
            result
        } else {
            panic!()
        }
    }

    pub async fn get_chat(&self, id: i64) -> Chat {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgEvent {
                extra: Some(future.uuid.clone()),
                data: TgEventData::GetChat(GetChat { chat_id: id }),
            })
            .expect("Can't serialize message"),
        );

        if let TgEventData::Chat(result) = future.await {
            result
        } else {
            panic!()
        }
    }

    pub async fn get_user(&self, id: i64) -> User {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgEvent {
                extra: Some(future.uuid.clone()),
                data: TgEventData::GetUser(GetUser { user_id: id }),
            })
            .expect("Can't serialize message"),
        );

        if let TgEventData::User(result) = future.await {
            result
        } else {
            panic!()
        }
    }
}
