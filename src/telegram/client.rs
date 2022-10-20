use std::ffi;

#[link(name = "tdjson")]
extern "C" {
    fn td_create_client_id() -> i32;
    fn td_receive(timeout: f64) -> *const i8;
    fn td_send(client_id: i32, request: *const u8);
    fn td_execute(request: *const u8) -> *const i8;
}

pub struct Client {
    id: i32,
    timeout: f64,
}

impl Client {
    pub fn new(timeout: f64) -> Client {
        Client::execute(r#"{ "@type": "setLogVerbosityLevel", "new_verbosity_level": 0 }"#);
        Client {
            id: unsafe { td_create_client_id() },
            timeout: timeout,
        }
    }

    pub fn execute(data: &str) {
        unsafe { td_execute(format!("{}\0", data).as_ptr()) };
    }

    pub fn receive(&self) -> Option<String> {
        unsafe {
            match td_receive(self.timeout)
                .as_ref()
                .map(|it| ffi::CStr::from_ptr(it).to_string_lossy().into_owned())
            {
                None => None,
                Some(data) => Some(data),
            }
        }
    }

    pub fn send(&self, data: &str) {
        unsafe { td_send(self.id, format!("{}\0", data).as_ptr()) }
    }
}
