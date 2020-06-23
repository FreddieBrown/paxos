mod messages;
pub use crate::messages::Message;
pub use crate::messages::Status;
mod acceptors;
pub use crate::acceptors::Acceptor;
mod proposers;
pub use crate::proposers::Proposer;
use std::collections::HashMap;
use rand::Rng;
use clap::{Arg, App};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Command line arguments
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // let f: u32 = *(&args[1].to_string().parse::<u32>().unwrap());
    // let prob: f32 = *(&args[2].to_string().parse::<f32>().unwrap());

    let matches = App::new("Synchronous PAXOS")
        .version("0.0.1")
        .author("Freddie Brown")
        .about("Synchronous PAXOS for deciding a value")
        .arg(Arg::with_name("ftolerance")
                .short("f")
                .long("ftolerance")
                .takes_value(true)
                .help("Number of nodes that can fail"))
        .arg(Arg::with_name("probability")
                .short("p")
                .long("probability")
                .takes_value(true)
                .help("Probability with which a random value is sent to proposer"))
        .arg(Arg::with_name("range")
                .short("r")
                .long("range")
                .takes_value(true)
                .help("Range of value to use"))
        .get_matches();

    let f: u32 = match matches.value_of("ftolerance"){
        Some(v) => match v.parse::<u32>(){
            Ok(t) => t,
            Err(_) => 2
        },
        None => 2
    };
    let prob: f32 = match matches.value_of("probability"){
        Some(v) => match v.parse::<f32>(){
            Ok(t) => t,
            Err(_) => 0.2
        },
        None => 0.2
    };
    let range: Arc<u32> = Arc::new(match matches.value_of("range"){
        Some(v) => match v.parse::<u32>(){
            Ok(t) => t,
            Err(_) => 100
        },
        None => 100
    });

    let accs: Arc<u32> = Arc::new((3*f)+1);
    let props: Arc<u32> = Arc::new((3*f)+1);
    let declared_val = Arc::new(Mutex::new(0));
    let mut rng = rand::thread_rng();
    let threshold: Arc<u32> = Arc::new(((*range as f32)*prob) as u32);
    let fail_val = Arc::new(rng.gen_range(1, *range+1));
    println!("Num of Acceptors: {}, Num of Proposers: {}", accs, props);

    // Setting up data structures to hold information
    let mut handles = vec![];
    let buffer: Arc<HashMap<u32, Mutex<Vec<Message>>>>;
    let acceptors: Arc<Vec<Mutex<Acceptor>>>;
    let proposers: Arc<Vec<Mutex<Proposer>>>;

    // Create acceptors and proposers
        // Make these accessible using Arc

    // Instantiating the Acceptors
    let mut acc_temp: Vec<Mutex<Acceptor>> = Vec::new();
    let mut buff_temp = HashMap::new();
    for i in 0..*accs {
        let mut acctr = Acceptor::default();
        acctr.set_id(i);
        println!("Acceptor: {}", acctr);
        acc_temp.push(Mutex::new(acctr));
        buff_temp.insert(i, Mutex::new(Vec::new()));
    }


    // Instantiating the Proposers
    let mut prop_temp: Vec<Mutex<Proposer>> = Vec::new();
    for j in *accs..*accs+*props {
        let mut ppr = Proposer::default();
        ppr.set_id(j);
        ppr.set_num_acceptors(*accs);
        println!("Proposer: {}", ppr);
        prop_temp.push(Mutex::new(ppr));
        buff_temp.insert(j, Mutex::new(Vec::new()));
    }
    println!("Number of Acceptors needed for majority: {}", (((*accs)/2)+1));

    acceptors = Arc::from(acc_temp);
    proposers = Arc::from(prop_temp);
    buffer = Arc::from(buff_temp);

    // Generate a number of threads (each thread should have an acceptor and a proposer)
    for t in 0..*accs {
        let declared_val_th = Arc::clone(&declared_val);
        let threshold_th = Arc::clone(&threshold);
        let buffer_th = Arc::clone(&buffer);
        let props_th = Arc::clone(&proposers);
        let accs_th = Arc::clone(&acceptors);
        let acc_num = Arc::clone(&accs);
        let prop_num = Arc::clone(&props);
        let range_num = Arc::clone(&range);
        let fail_val_th = Arc::clone(&fail_val);
        let handle = thread::spawn(move || {
            let th_id = t;
            let mut rng_th = rand::thread_rng();
            let prop_id = *acc_num + th_id;

            loop {
                let number = rng_th.gen_range(1, *range_num+1);
                let id = rng_th.gen_range(*acc_num, *acc_num+*prop_num);
                let fail_id = rng_th.gen_range(0, *acc_num+*prop_num);
                let mut proposer = (*props_th)[th_id as usize].lock().unwrap();
                // println!("Thread {}: {}, {}, {}", t, number, id, fail_id);

                // Acceptors
                {
                    let mut acceptor = (*accs_th)[th_id as usize].lock().unwrap();
                    if number == *fail_val_th && fail_id == th_id {
                        acceptor.set_status(Status::Failed);
                        println!("Acceptor {} has failed", th_id);
                    }
                    // println!("Thread {}", th_id);
                    acceptor.check_buffer(&buffer_th);
                    acceptor.send_buffer(&buffer_th);
                    if proposer.status == Status::Failed && acceptor.status == Status::Failed{
                        return
                    }
                }

                // Proposers
                
                if proposer.status != Status::Failed {
                    if number == *fail_val_th && fail_id == prop_id { 
                        proposer.set_status(Status::Failed);
                        println!("Proposer {} has failed", prop_id);

                    }

                    

                    if number <= *threshold_th && prop_id == id {
                        proposer.set_val(number);
                    }

                    proposer.run(&accs_th, &buffer_th);
                }
                 
                if proposer.status == Status::Accepted {
                    *(declared_val_th).lock().unwrap() = proposer.val();
                }
                
                if *(declared_val_th).lock().unwrap() > 0 {
                    return
                }
            }
        });
        handles.push(handle);
        
    }

    
    // End program
    for handle in handles {
        handle.join().unwrap();
    }
    num_of_fails(&acceptors, &proposers);
    println!("DECLARED VALUE: {}", *(declared_val).lock().unwrap());
}

fn num_of_fails(acceptors: &Arc<Vec<Mutex<Acceptor>>>, proposers: &Arc<Vec<Mutex<Proposer>>>) {
    let mut acc_fails = 0;
    let mut prop_fails = 0;
    for acc_mut in (*acceptors).iter(){
        let acc = acc_mut.lock().unwrap();
        match acc.status {
            Status::Failed => acc_fails += 1,
            _ => ()
        };
    }

    for prop_mut in (*proposers).iter(){
        let prop = prop_mut.lock().unwrap();
        match prop.status {
            Status::Failed => prop_fails += 1,
            _ => ()
        };
    }

    println!("Number of Acceptors Failed {}/{}", acc_fails, (*acceptors).len());
    println!("Number of Proposers Failed {}/{}", prop_fails, (*proposers).len());
}

#[cfg(test)]
mod tests;