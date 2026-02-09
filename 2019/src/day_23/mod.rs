use crate::intcode::{IntcodeComputer, RunStatus};
use anyhow::{Context, Result};
use std::sync::{LazyLock, Mutex, mpsc};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_23/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let y_to_255 = part_one(input.as_str())?;
    println!("Day 23, Part 1: First Y value sent to 255: {}", y_to_255);

    let y_to_0 = part_two(input.as_str())?;
    println!("Day 23, Part 2: First Y value sent to 0 twice: {}", y_to_0);

    Ok(())
}

fn part_one(input: &str) -> Result<isize> {
    let (res_tx, res_rx) = mpsc::channel::<isize>();
    let mut network = Network::new(move |packet: &Packet, idle_check: bool| -> bool {
        if packet.dest == 255 && !idle_check {
            res_tx.send(packet.y).unwrap();
            return true;
        }
        false
    });

    for addr in 0..50 {
        let nic = NetworkInterfaceController::new(input, addr)?;
        network.add_nic(nic);
    }

    network.run()?;

    let y_to_255 = res_rx.recv()?;
    Ok(y_to_255)
}

fn part_two(input: &str) -> Result<isize> {
    let (res_tx, res_rx) = mpsc::channel::<isize>();
    let mut network = Network::new(move |packet: &Packet, idle_check: bool| -> bool {
        static LAST_NAT: LazyLock<Mutex<Option<Packet>>> = LazyLock::new(|| Mutex::new(None));
        if packet.dest == Network::NAT_ADDR && idle_check {
            let mut last_nat = LAST_NAT.lock().unwrap();
            if let Some(last_nat) = last_nat.take()
                && last_nat.y == packet.y
            {
                res_tx.send(packet.y).unwrap();
                return true;
            }
            *last_nat = Some(packet.clone());
        }
        false
    });

    for addr in 0..50 {
        let nic = NetworkInterfaceController::new(input, addr)?;
        network.add_nic(nic);
    }

    network.run()?;

    let y_to_0 = res_rx.recv()?;
    Ok(y_to_0)
}

type PacketAnalyzer = Box<dyn Fn(&Packet, bool) -> bool>;

struct Network {
    nics: Vec<NetworkInterfaceController>,

    observer_fn: PacketAnalyzer,
}

impl Network {
    const NAT_ADDR: usize = 255;

    fn new<F: Fn(&Packet, bool) -> bool + 'static>(observer_fn: F) -> Self {
        Self {
            nics: Vec::new(),
            observer_fn: Box::new(observer_fn),
        }
    }

    fn add_nic(&mut self, nic: NetworkInterfaceController) {
        self.nics.push(nic);
    }

    fn run(&mut self) -> Result<()> {
        let mut nat_packet = None;

        loop {
            let mut idle_state = true;
            for addr in 0..self.nics.len() {
                if self.nics[addr].packet_queue_len() != 0 {
                    idle_state = false;
                } else {
                    self.nics[addr].enqueue_empty_wait_list_signal();
                }

                match self.nics[addr].run()? {
                    NicState::Done => anyhow::bail!("Nic finished early"),
                    NicState::WaitForInput => (),
                    NicState::Send(packet) => {
                        if (self.observer_fn)(&packet, false) {
                            return Ok(());
                        }

                        if packet.dest < self.nics.len() {
                            self.nics[packet.dest].enqueue_packet(packet);
                        } else if packet.dest == Self::NAT_ADDR {
                            nat_packet = Some(packet);
                        }
                    }
                }

                if idle_state && let Some(nat_packet) = nat_packet.take() {
                    if (self.observer_fn)(&nat_packet, true) {
                        return Ok(());
                    }
                    self.nics[0].enqueue_packet(nat_packet.clone());
                }
            }
        }
    }
}

struct NetworkInterfaceController {
    addr: usize,
    inner: IntcodeComputer,
}

impl NetworkInterfaceController {
    fn new(program: &str, addr: usize) -> Result<Self> {
        Ok(Self {
            addr,
            inner: IntcodeComputer::new(program, [addr.try_into()?])?,
        })
    }

    fn packet_queue_len(&self) -> usize {
        self.inner.buffered_input()
    }

    fn enqueue_packet(&mut self, packet: Packet) {
        self.inner.push_input(packet.x);
        self.inner.push_input(packet.y);
    }

    fn enqueue_empty_wait_list_signal(&mut self) {
        self.inner.push_input(-1);
    }

    fn run(&mut self) -> Result<NicState> {
        loop {
            match self.inner.run()? {
                RunStatus::Halt => return Ok(NicState::Done),
                RunStatus::WaitForInput => {
                    // If the NIC computer issues an input instruction it wants to read the next
                    // packet from the receive buffer but the buffer is empty
                    // self.inner.push_input(-1);
                    return Ok(NicState::WaitForInput);
                }
                RunStatus::OutputValue if self.inner.buffered_output() == 3 => {
                    // If the NIC computer issues an output instruction it wants to transmit a
                    // packet to another NIC on the network
                    // It will issue 3 consecutive output instruction to send one packet
                    let packet = Packet {
                        source: self.addr,
                        dest: self
                            .inner
                            .next_output()
                            .context("No output for destionation")?
                            .try_into()?,
                        x: self.inner.next_output().context("No output for X")?,
                        y: self.inner.next_output().context("No output for Y")?,
                    };
                    return Ok(NicState::Send(packet));
                }
                RunStatus::OutputValue => (),
            }
        }
    }
}

enum NicState {
    Done,
    WaitForInput,
    Send(Packet),
}

#[derive(Debug, Clone, PartialEq)]
struct Packet {
    source: usize,
    dest: usize,
    x: isize,
    y: isize,
}
