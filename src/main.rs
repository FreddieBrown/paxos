mod messages;
pub use crate::messages::Message;
pub use crate::messages::Status;
mod acceptors;
pub use crate::acceptors::Acceptor;
mod proposers;
pub use crate::proposers::Proposer;
use std::collections::HashMap;
use std::env;
use rand::Rng;

fn main() {
    // Command line arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let f: u32 = *(&args[1].to_string().parse::<u32>().unwrap());
    let prob: f32 = *(&args[2].to_string().parse::<f32>().unwrap());
    let accs:u32 = (3*f)+1;
    let props:u32 = (3*f)+1;
    let mut declared_val = 0;
    let mut rng = rand::thread_rng();
    let threshold: u32 = (100.0*prob) as u32;
    let fail_val = rng.gen_range(1, 101);
    println!("Num of Acceptors: {}, Num of Proposers: {}", accs, props);

    // Setting up data structures to hold information
    let mut buffer: HashMap<u32, Vec<Message>> = HashMap::new();
    let mut acceptors: Vec<Acceptor> = Vec::new();
    let mut proposers: Vec<Proposer> = Vec::new();

    // Instantiating the Acceptors
    for i in 0..accs {
        let mut acctr = Acceptor::default();
        acctr.set_id(i);
        println!("Acceptor: {}", acctr);
        acceptors.push(acctr);
        buffer.insert(i, Vec::new());
    }

    // Instantiating the Proposers
    for j in accs..accs+props {
        let mut ppr = Proposer::default();
        ppr.set_id(j);
        ppr.set_num_acceptors(accs);
        println!("Proposer: {}", ppr);
        proposers.push(ppr);
        buffer.insert(j, Vec::new());
    }

    // Start main loop
    loop {
        // Add in section about client selecting a value for a proposer
        let number = rng.gen_range(1, 101);
        let id = rng.gen_range(accs, accs+props);
        let fail_id = rng.gen_range(0, accs+props);
        // Add in section about making nodes fail

        for acc in acceptors.iter_mut(){
            println!("Acceptor {}", acc.id());
            if number == fail_val && fail_id == acc.id(){
                acc.set_status(Status::Failed);
                println!("Acceptor {} has failed", acc.id());
            }
            acc.check_buffer(&mut buffer);
            acc.send_buffer(&mut buffer);
        }

        for prop in proposers.iter_mut(){
            println!("Proposer {}", prop.id());
            if number == fail_val && fail_id == prop.id(){
                prop.set_status(Status::Failed);
                println!("Proposer {} has failed", prop.id());
            }
            
            if prop.status != Status::Failed && number <= threshold && prop.id() == id{
                prop.set_val(number);
            }
            prop.run(&acceptors, &mut buffer);
            if prop.status == Status::Accepted{
                declared_val = prop.val();
            }

            match prop.status {
                Status::Accepted => {
                    println!("Declaring value for {}", &prop.id());
                    declared_val = prop.val();
                }
                _ => ()
            }
        }
        if declared_val > 0 {
            break;
        }
    }
    println!("Ending Program. Value is {}", declared_val);
    // End program
}
