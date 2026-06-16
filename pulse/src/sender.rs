use shared::Processes;
use shared::packet::{Packet, PacketKind};
use std::error::Error;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

pub enum Action {
    Retransmit,
    Send,
    Nothing,
}

pub struct Sender {
    address: SocketAddr,
    socket: UdpSocket,
    last_sent: usize,
    curr_timer: Instant,
}

impl Sender {
    pub fn new(address: SocketAddr) -> Self {
        let socket = UdpSocket::bind(&address).expect("Unable to bind socket!");
        socket
            .set_read_timeout(Some(Duration::from_millis(10)))
            .expect("Unable to setup socket!");
        Self {
            address,
            socket,
            last_sent: 0,
            curr_timer: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> Action {
        if self.curr_timer.elapsed() >= Duration::from_secs(5) {
            return Action::Send;
        }

        //keep as is for now -> future fragment???
        let mut buf = [0u8; 1500];
        if let Ok(r) = self.socket.recv(&mut buf) {
            if let Ok(p) = serde_json::from_slice::<Packet>(&buf[..r]) {
                match p.packet_kind() {
                    PacketKind::Data(_) => (),
                    PacketKind::RetransmitRequest => {
                        if p.packet_id() < self.last_sent {
                            return Action::Retransmit;
                        }
                    }
                }
            }
        }

        Action::Nothing
    }

    pub fn send(
        &mut self,
        processes: Processes,
        addr_to: SocketAddr,
    ) -> Result<usize, Box<dyn Error>> {
        self.last_sent = self.last_sent.wrapping_add(1);
        let packet = Packet::new(
            self.address,
            addr_to,
            self.last_sent,
            PacketKind::Data(processes),
        );
        let json = serde_json::to_string(&packet)?;
        let r = self.socket.send_to(json.as_bytes(), addr_to)?;
        self.curr_timer = Instant::now();
        Ok(r)
    }
}
