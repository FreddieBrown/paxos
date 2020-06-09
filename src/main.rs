mod messages;
pub use crate::messages::Message;
pub use crate::messages::Status;
mod acceptors;
pub use crate::acceptors::Acceptor;
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
        println!("Acceptor: {}", acctr);
        acceptors.push(acctr);
        buffer.insert(i, Vec::new());
    }

    for j in accs..accs+props {
        let mut ppr = Proposer::default();
        ppr.set_id(j);
        ppr.set_num_acceptors(accs);
        println!("Proposer: {}", ppr);
        proposers.push(ppr);
        buffer.insert(j, Vec::new());
    }
}
