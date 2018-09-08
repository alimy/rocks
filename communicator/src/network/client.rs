use super::utils;

pub fn connect() {
    utils::parse_protocol();
    let _addr = utils::addr::get_addr_v4(String::from("127.0.0.1"));
    println!("communicator::network::client::connect()");
}