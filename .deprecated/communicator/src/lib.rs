pub use self::network::server;

pub mod client;
pub mod network;
mod outermost;

pub fn run() {
    println!("communicator::run()");
}

#[cfg(test)]
mod tests {
    use super::network::server;

    #[test]
    fn it_works() {
        ::network::server::connect();
        server::connect();
        super::client::connect();
        super::outermost::try_me()
    }
}
