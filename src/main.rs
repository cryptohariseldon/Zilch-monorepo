
use miden_assembly::Assembler;
use miden_prover::{prove, ProgramInputs, ProofOptions};

use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    x: u32,
    y: f32,
    s: String,
}

fn tester() -> Result<(), bincode::Error> {
    let data = Data {
        x: 42,
        y: 3.14,
        s: "Hello, World!".to_string(),
    };

    let serialized = serialize(&data)?.to_vec();

    let mut file = File::create("data.bin")?;
    file.write_all(&serialized)?;

    Ok(())
}

fn main() {

    // instantiate the assembler
    let assembler = Assembler::default();

    // this is our program, we compile it from assembly code
    let program = assembler.compile("begin push.3 push.5 add end").unwrap();

    // let's execute it and generate a STARK proof
    let (outputs, proof) = prove(
        &program,
        &ProgramInputs::none(),   // we won't provide any inputs
        &ProofOptions::default(), // we'll be using default options
    )
    .unwrap();



    // the output should be 8
    // assert_eq!(vec![8], outputs);
    tester();
    println!("Hello, world!");
}
