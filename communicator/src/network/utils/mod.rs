pub mod addr;
mod protocol;

pub use self::protocol::parse_protocol;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn parse_addr_v4(addr: String) -> u32 { ;
    println!("comunicator::network::utils::parse_addr_v4()");
    let _addr = addr.split(':');
    0
}