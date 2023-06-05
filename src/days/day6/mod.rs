use crate::communication_system::{start_of_packet_marker, start_of_message_marker};

pub fn day6a_solution(input_content: &str) {
    let start_position = start_of_packet_marker(input_content.to_string());
    println!("Packer start position: {start_position}");
}

pub fn day6b_solution(input_content: &str) {
    let start_position = start_of_message_marker(input_content.to_string());
    println!("Message start position: {start_position}");
}
