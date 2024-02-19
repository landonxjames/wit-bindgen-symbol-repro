mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        let headers = wasi::http::types::Headers::new();
        let foo = wasi::http::types::OutgoingResponse::new(headers);

        "Hello, World!".to_string()
    }
}
