use std::{error::Error, future::pending};
use zbus::{connection, interface};

struct Greeter {
    count: u64,
}

#[interface(name = "org.zbus.cantor")]
impl Greeter {
    async fn hola(&mut self, name: &str) -> String {
        self.count += 1;
        format!("yo me {}, called: {}", name, self.count);
    }
}
