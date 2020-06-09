use std::fmt;
use crate::message::Message;
use crate::acceptors::Acceptor;

pub struct Proposer{
    id: u32,
    val: u32,
    num_acceptors: u32,

}

impl Proposer{

    pub fn num_acceptors(&self) -> u32{
        self.num_acceptors
    }

    pub fn id(&self) -> u32{
        self.id
    }

    pub fn val(&self) -> u32{
        self.val
    }

    pub fn set_num_acceptors(&mut self, num: u32){
        self.num_acceptors = num;
    }

    pub fn set_id(&mut self, id: u32){
        self.id = id;
    }

    pub fn set_val(&mut self, val: u32){
        self.val = val;
    }

    pub fn phase_one(&mut self, list: &mut [Acceptor]){
        println!("Phase 1");
        for acc in list {
            println!("{}",acc);
        }

    }

    pub fn phase_two(&mut self, list: &mut [Acceptor]){
        println!("Phase 2");
        for acc in list {
            println!("{}",acc);
        }

    }
}

impl Default for Proposer{
    fn default() -> Proposer{
        Proposer{
            id: 0,
            val: 0,
            num_acceptors: 0,
        }
    }
}

impl fmt::Display for Proposer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Proposer => num_acceptors: {}, id: {}, val: {}", self.num_acceptors, self.id, self.val)
    }

}