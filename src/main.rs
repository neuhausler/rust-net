extern crate pnet;

use std::env;

use pnet::packet::{Packet};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};

use pnet::datalink::{datalink_channel};
use pnet::datalink::DataLinkChannelType::{Layer2};

use pnet::util::{NetworkInterface, get_network_interfaces};

fn handle_arp_packet(interface_name: &str, ethernet: &EthernetPacket) {
    if ethernet.packet()[21] == 1 { return }

    println!("[{}]: ARP packet: {} > {}; length: {}; data: {:?}",
            interface_name,
            ethernet.get_source(),
            ethernet.get_destination(),
            ethernet.packet().len(),
            ethernet.packet());

    println!("{}.{}.{}.{}",
            ethernet.packet()[38],
            ethernet.packet()[39],
            ethernet.packet()[40],
            ethernet.packet()[41])
}

fn handle_packet(interface_name: &str, ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Arp  => handle_arp_packet(interface_name, ethernet),
        _ => return,
    }
}

fn main() {
    let iface_name = env::args().nth(1).unwrap();
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;

    // Find the network interface with the provided name
    let interfaces = get_network_interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    // Create a channel to receive on
    let (_, mut rx) = match datalink_channel(&interface, 0, 4096, Layer2) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("packetdump: unable to create channel: {}", e)
    };

    let mut iter = rx.iter();
    loop {
        match iter.next() {
            Ok(packet) => handle_packet(&interface.name[..], &packet),
            Err(e) => panic!("packetdump: unable to receive packet: {}", e)
        }
    }
}
