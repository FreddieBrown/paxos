mod message;
pub use crate::message::Message;
mod acceptors;
pub use crate::acceptors::Acceptor;
pub use crate::acceptors::Status;
mod proposers;
pub use crate::proposers::Proposer;

fn main() {
    let val = Message::Prepare(32);
    let val1 = Message::Propose(32, 10);
    let mut acceptor = Acceptor::default();
    acceptor.set_max_known_id(10);
    let mut proposer = Proposer::default();
    proposer.set_id(10);
    acceptor.publish_message(val);
    acceptor.check_messages();
    acceptor.publish_message(val1);
    acceptor.check_messages();
    println!("{}", acceptor);
    println!("{}", proposer);
}
