mod message;
pub use crate::message::Message;
mod acceptors;
pub use crate::acceptors::Acceptor;
pub use crate::acceptors::Status;
mod proposers;
pub use crate::proposers::Proposer;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let f: u32 = *(&args[1].to_string().parse::<u32>().unwrap());
    let accs:u32 = (3*f)+1;
    let props:u32 = (3*f)+1;

    println!("Num of Acceptors: {}, Num of Proposers: {}", accs, props);

    let mut buffer: HashMap<u32, Vec<Message>> = HashMap::new();
    let mut acceptors: Vec<Acceptor> = Vec::new();
    let mut proposers: Vec<Proposer> = Vec::new();

    for i in 0..accs {
        let mut acctr = Acceptor::default();
        acctr.set_id(i);
        acceptors.push(acctr);
        buffer.insert(i, Vec::new());
    }

    for j in accs..accs+props {
        let mut ppr = Proposer::default();
        ppr.set_id(j);
        proposers.push(ppr);
        buffer.insert(j, Vec::new());
    }
    // let mut acceptor = Acceptor::default();
    // acceptor.set_max_known_id(10);
    // let mut proposer = Proposer::default();
    // proposer.set_id(10);
    // acceptor.publish_message(Message::Prepare(32));
    // acceptor.publish_message(Message::Propose(32, 10));
    // println!("{}", acceptor);
    // println!("{}", proposer);
}
