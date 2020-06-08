mod message;
pub use crate::message::Message;
mod acceptors;
pub use crate::acceptors::Acceptor;
pub use crate::acceptors::Status;
mod proposers;
pub use crate::proposers::Proposer;

fn main() {

    let val = Message::Propose(32,5);
    let mut acceptor = Acceptor::default();
    acceptor.set_max_known_id(10);
    let mut proposer = Proposer::default();
    proposer.set_id(10);
    println!("{}", val);
    println!("{}", acceptor);
    println!("{}", proposer);
}
