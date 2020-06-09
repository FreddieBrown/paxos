mod message;
pub use crate::message::Message;
mod acceptors;
pub use crate::acceptors::Acceptor;
pub use crate::acceptors::Status;
mod proposers;
pub use crate::proposers::Proposer;

fn main() {
    let mut acceptor = Acceptor::default();
    acceptor.set_max_known_id(10);
    let mut proposer = Proposer::default();
    proposer.set_id(10);
    acceptor.publish_message(Message::Prepare(32, &mut proposer));
    acceptor.publish_message(Message::Propose(32, 10, &mut proposer));
    println!("{}", acceptor);
    println!("{}", proposer);
}
