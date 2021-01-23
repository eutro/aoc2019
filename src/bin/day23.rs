use aoc::intcode::{Program, Int, VM, State};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::HashMap;

const COMPUTERS: Int = 50;
const NAT_ADDRESS: Int = 255;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Packet {
    x: Int,
    y: Int,
}

struct NetworkComputer {
    chan: Receiver<Packet>,
    vm: VM
}

impl NetworkComputer {
    fn new(address: Int, nic: &Program, chan: Receiver<Packet>) -> Self {
        let mut vm = VM::of(nic);
        vm.input(address);
        NetworkComputer { chan, vm }
    }

    fn advance(&mut self, addresses: &HashMap<Int, Sender<Packet>>) -> bool {
        match self.vm.next_state().unwrap() {
            State::AwaitingInput => {
                match self.chan.try_recv() {
                    Ok(packet) => {
                        self.vm.input(packet.x);
                        self.vm.input(packet.y);
                        true
                    }
                    Err(_) => {
                        self.vm.input(-1);
                        false
                    }
                }
            }
            State::Outputting(dest) => {
                if let State::Outputting(x) = self.vm.next_state().unwrap() {
                    if let State::Outputting(y) = self.vm.next_state().unwrap() {
                        match addresses.get(&dest) {
                            None => {}
                            Some(dest) => {
                                dest.send(Packet { x, y }).unwrap();
                            }
                        }
                        true
                    } else { panic!() }
                } else { panic!() }
            }
            State::Finished => false,
        }
    }
}

fn main() {
    let nic = Program::from_stdin().unwrap();
    let mut addresses = HashMap::new();

    let (sender, nat) = channel::<Packet>();
    addresses.insert(NAT_ADDRESS, sender);

    let mut computers = Vec::new();
    for address in 0..COMPUTERS {
        let (sender, receiver) = channel();
        addresses.insert(address, sender);
        computers.push(NetworkComputer::new(address, &nic, receiver));
    }

    run_until_idle(&mut computers, &addresses);
    let mut last_packet = nat.try_recv().unwrap();
    println!("Packet: Y={}", last_packet.y);

    if let Some(p) = nat.try_iter().last() {
        last_packet = p;
    }
    let mut last_sent;
    let nat_dest = addresses.get(&0).unwrap();

    loop {
        last_sent = last_packet;
        nat_dest.send(last_packet).unwrap();
        run_until_idle(&mut computers, &addresses);
        if let Some(p) = nat.try_iter().last() {
            last_packet = p;
        }
        if last_packet == last_sent {
            break;
        }
    }

    println!("Duplicate: Y={}", last_packet.y);
}

const MAX_IDLES: u32 = 2;

fn run_until_idle(computers: &mut Vec<NetworkComputer>, addresses: &HashMap<Int, Sender<Packet>>) {
    let mut idle_counter = 0;
    while idle_counter < MAX_IDLES {
        let mut idle = true;
        for computer in &mut *computers {
            while computer.advance(addresses) {
                idle = false;
            }
        }
        if idle {
            idle_counter += 1;
        } else {
            idle_counter = 0;
        }
    }
}
