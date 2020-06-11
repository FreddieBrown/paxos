use super::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn proposer_receives_val_and_sends_messages() {
    let mut buffer: HashMap<u32, Vec<Message>> = HashMap::new();
    let mut acceptors: Vec<Acceptor> = Vec::new();
    let mut proposers: Vec<Proposer> = Vec::new();
    let range = 100;
    let prob = 0.2;
    let f = 1;
    let accs:u32 = (3*f)+1;
    let props:u32 = (3*f)+1;
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
    let number = 10;
    proposers[0].set_val(number);
    proposers[0].run(&acceptors, &mut buffer);
    for i in 0..accs {
        let mess = buffer.get_mut(&i).unwrap().pop().unwrap();
        assert_eq!(mess, Message::Prepare(proposers[0].id(), proposers[0].id()));
    }
}

#[test]
fn proposer_receives_promise() {
    let mut buffer: HashMap<u32, Vec<Message>> = HashMap::new();
    let mut acceptors: Vec<Acceptor> = Vec::new();
    let mut proposers: Vec<Proposer> = Vec::new();
    let range = 100;
    let prob = 0.2;
    let f = 1;
    let accs:u32 = (3*f)+1;
    let props:u32 = (3*f)+1;
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
    let number = 10;
    proposers[0].set_val(number);
    proposers[0].run(&acceptors, &mut buffer);
    for acc in acceptors.iter_mut() {
        acc.check_buffer(&mut buffer);
        acc.send_buffer(&mut buffer);
    }

    let bucket = buffer.get_mut(&proposers[0].id()).unwrap();
    while bucket.len() > 0 {
        let mess = bucket.pop().unwrap();
        match mess {
            Message::Promise(id,_) => assert_eq!(id, proposers[0].id()),
            _ => assert!(false)
        };
    }
}