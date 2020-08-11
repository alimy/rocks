use super::parse_addr_v4;
use super::TrafficLight::*;

pub fn get_addr_v4(addr: String) -> u32 {
    traffic_light();
    println!("communicator::network::utils::get_addr_v4()");
    parse_addr_v4(addr)
}

fn traffic_light() {
    let _red = Red;
    let _yellow = Yellow;
    let _green = Green;
}