use super::{api::*, client::Client};
use std::env::var;
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
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

struct TgEventFuture {
    shared_state: Arc<Mutex<TgEventSharedState>>,
    uuid: String,
}

struct TgEventSharedState {
    message: Option<Tg>,
    waker: Option<Waker>,
}

impl Future for TgEventFuture {
    type Output = Tg;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if let Some(message) = &shared_state.message {
            Poll::Ready(message.clone())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TgEventFuture {
    pub fn new(rx: spmc::Receiver<TgExtra>) -> Self {
        let shared_state = Arc::new(Mutex::new(TgEventSharedState {
            message: None,
            waker: None,
        }));

        let uuid = format!("{}", Uuid::new_v4());

        let thread_shared_state = shared_state.clone();
        let thread_uuid = uuid.clone();

        thread::spawn(move || loop {
            let message = rx.recv().unwrap();

            if let Some(extra) = message.clone().extra {
                if extra != thread_uuid {
                    continue;
                }
            } else {
                continue;
            }

            let mut shared_state = thread_shared_state.lock().unwrap();
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
    receiver: spmc::Receiver<TgExtra>,
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
                /*if let Ok(message) = serde_json::from_str::<TgExtra>(&r) {
                    tx.send(message).unwrap_or_default();
                } else {
                    println!("Can't parse message: {}", r);
                }*/

                tx.send(serde_json::from_str::<TgExtra>(&r).unwrap()).unwrap();
            }
        });

        manager
    }

    pub async fn authorize(&self, dir: &str, device: &str) {
        loop {
            match self.auth().await {
                Tg::AuthorizationStateWaitTdlibParameters => {
                    self.set_tdlib_parameters(TdlibParameters {
                        use_test_dc: false,
                        database_directory: format!("{}/database", dir),
                        files_directory: format!("{}/files", dir),
                        use_file_database: false,
                        use_chat_info_database: false,
                        use_message_database: false,
                        use_secret_chats: true,
                        api_id: var("API_ID").unwrap().parse::<i32>().unwrap(),
                        api_hash: var("API_HASH").unwrap(),
                        system_language_code: String::from("en"),
                        device_model: device.to_owned(),
                        system_version: String::from(""),
                        application_version: String::from(env!("CARGO_PKG_VERSION")),
                        enable_storage_optimizer: true,
                        ignore_file_names: true,
                    })
                    .await
                }
                Tg::AuthorizationStateWaitCode => {
                    self.set_auth_code(input("code >").trim_end().to_owned())
                        .await
                }
                Tg::AuthorizationStateWaitPhoneNumber => {
                    self.set_auth_phone(input("phone >").trim_end().to_owned())
                        .await
                }
                Tg::AuthorizationStateReady => break,

                _ => panic!("Invalid auth type"),
            };
        }
    }

    async fn auth(&self) -> Tg {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgExtra {
                extra: Some(future.uuid.clone()),
                data: Tg::GetAuthorizationState,
            })
            .unwrap(),
        );

        future.await
    }

    pub async fn set_tdlib_parameters(&self, data: TdlibParameters) -> Tg {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgExtra {
                extra: Some(future.uuid.clone()),
                data: Tg::SetTdlibParameters(data),
            })
            .unwrap(),
        );

        future.await
    }

    pub async fn set_auth_phone(&self, data: String) -> Tg {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgExtra {
                extra: Some(future.uuid.clone()),
                data: Tg::SetAuthenticationPhoneNumber(SetAuthenticationPhoneNumber {
                    phone_number: data,
                }),
            })
            .unwrap(),
        );

        future.await
    }

    pub async fn set_auth_code(&self, data: String) -> Tg {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgExtra {
                extra: Some(future.uuid.clone()),
                data: Tg::CheckAuthenticationCode(CheckAuthenticationCode { code: data }),
            })
            .unwrap(),
        );

        future.await
    }

    pub async fn get_chats(&self, limit: i32) -> Chats {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgExtra {
                extra: Some(future.uuid.clone()),
                data: Tg::GetChats(GetChats { limit: limit }),
            })
            .unwrap(),
        );

        if let Tg::Chats(result) = future.await {
            result
        } else {
            panic!()
        }
    }

    pub async fn get_chat(&self, id: i64) -> Chat {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgExtra {
                extra: Some(future.uuid.clone()),
                data: Tg::GetChat(GetChat { chat_id: id }),
            })
            .unwrap(),
        );

        if let Tg::Chat(result) = future.await {
            result
        } else {
            panic!()
        }
    }

    pub async fn get_user(&self, id: i64) -> User {
        let future = TgEventFuture::new(self.receiver.clone());

        self.client.send(
            &serde_json::to_string(&TgExtra {
                extra: Some(future.uuid.clone()),
                data: Tg::GetUser(GetUser { user_id: id }),
            })
            .unwrap(),
        );

        if let Tg::User(result) = future.await {
            result
        } else {
            panic!()
        }
    }
}
