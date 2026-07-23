use shared::Processes;
use shared::packet::{Packet, PacketKind};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};
enum State {
    Regular,
    WaitingForRetransmit,
}

#[derive(Debug)]
pub enum Action {
    RequestRetransmit(SocketAddr),
    Received(Processes),
}

struct Connection {
    pub last_received: usize,
    pub curr_timer: Instant,
    pub state: State,
}

pub struct Receiver {
    address: SocketAddr,
    socket: UdpSocket,
    connections: HashMap<SocketAddr, Connection>,
    state: State,
    delay: usize,
}

impl Receiver {
    pub fn new(address: SocketAddr, delay: usize) -> Self {
        let socket = UdpSocket::bind(&address).expect("Unable to create socket!");
        socket
            .set_read_timeout(Some(Duration::from_millis(10)))
            .expect("Unable to setup socket!");
        Self {
            address,
            socket,
            connections: HashMap::new(),
            state: State::Regular,
            delay,
        }
    }

    pub fn tick(&mut self) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut buf = [0u8; 65536];
        while let Ok(r) = self.socket.recv(&mut buf) {
            if let Ok(p) = bincode::deserialize::<Packet>(&buf[..r]) {
                let sender = p.sender_address();
                let id = p.packet_id();
                match p.get_ownership_of_packet_kind() {
                    PacketKind::Data(processes) => {
                        let connection = self.connections.get_mut(&sender);
                        if let Some(c) = connection {
                            if id > c.last_received {
                                c.state = State::Regular;
                                c.curr_timer = Instant::now();
                                c.last_received = id;
                                actions.push(Action::Received(processes));
                            }
                        } else {
                            let new_connection = Connection {
                                last_received: id,
                                curr_timer: Instant::now(),
                                state: State::Regular,
                            };
                            self.connections.insert(sender, new_connection);
                            actions.push(Action::Received(processes));
                        }
                    }
                    _ => (),
                }
            }
        }
        
        for pair in self.connections.iter_mut() {
            let addr = pair.0;
            let connection = pair.1;
            match connection.state {
                State::Regular => {
                    //e.g., been waiting for 6 seconds, instead of usual 5
                    if connection.curr_timer.elapsed().as_secs() as usize >= self.delay + 1 {
                        connection.curr_timer = Instant::now();
                        connection.state = State::WaitingForRetransmit;
                        actions.push(Action::RequestRetransmit(*addr));
                    }
                }
                State::WaitingForRetransmit => {
                    //every 1 second
                    if connection.curr_timer.elapsed().as_secs() >= 1 {
                        connection.curr_timer = Instant::now();
                        actions.push(Action::RequestRetransmit(*addr));
                    }
                }
            }
        }

        actions
    }

    pub fn send_retransmit_request(&mut self, addr_to: SocketAddr) -> Result<usize, Box<dyn Error>> {
        let connection = self.connections.get(&addr_to);
        if let None = connection {
            return Ok(0);
        }
        let packet = Packet::new(
            self.address,
            addr_to,
            connection.unwrap().last_received,
            PacketKind::RetransmitRequest,
        );
        let json = serde_json::to_string(&packet)?;
        let bytes = self.socket.send_to(json.as_bytes(), addr_to)?;

        Ok(bytes)
    }
}
