use super::addr;
use super::super::help;
use super::super::helper;
use  network::helper::help as helper_help;
use super::TrafficLight;
use super::TrafficLight::{Red, Yellow};

pub fn parse_protocol() {
    help();
    helper::help();
    helper_help();
    addr::get_addr_v4(String::from("127.0.0.1"));
    println!("communicator::network::utils::protocol::parse_protocol()");
    traffic_light();
}

fn traffic_light() {
    let _red = Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;
}