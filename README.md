# PAXOS

This repo is a synchronous Rust implementation of the PAXOS consensus protocol.

This is a single thread simulation desgined to show how PAXOS works for a certain f-tolerance.

To run the program, use `cargo run`. This will run the program with its basic options.

The program has a command line interface to tweak parameters. These can be accessed by using `cargo run -- [OPTIONS]`. These options are:

- `-f`: Level of fault tolerance of the program. Number of nodes that can fail for before the program doesn't work anymore. Standard value is 2.
- `-p`: Probability that a value is sent to a proposer. Increasing this will mean values will be proposed to nodes with increased frequency. The standard value for this is 0.2.
- `-r`: This specifies the range of values to be used and decided on. The standard value for this is 100.
