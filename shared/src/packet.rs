use crate::Processes;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize, Serialize)]
pub enum PacketKind {
    Data(Processes),
    RetransmitRequest,
}

//rr retransmit request
#[derive(Deserialize, Serialize)]
pub struct Packet {
    sender_address: SocketAddr,
    receiver_address: SocketAddr,
    packet_id: usize,
    packet_kind: PacketKind,
}

impl Packet {
    pub fn new(
        sender_address: SocketAddr,
        receiver_address: SocketAddr,
        packet_id: usize,
        packet_kind: PacketKind,
    ) -> Self {
        Self {
            sender_address,
            receiver_address,
            packet_id,
            packet_kind,
        }
    }

    pub fn sender_address(&self) -> SocketAddr {
        self.sender_address
    }

    pub fn receiver_address(&self) -> SocketAddr {
        self.receiver_address
    }

    pub fn packet_id(&self) -> usize {
        self.packet_id
    }

    pub fn packet_kind(&self) -> &PacketKind {
        &self.packet_kind
    }

    pub fn get_ownership_of_packet_kind(self) -> PacketKind {
        self.packet_kind
    }
}
