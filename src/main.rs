
use miden_assembly::Assembler;
use miden_prover::{prove, ProgramInputs, ProofOptions};

use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
pub use winterfell::StarkProof;
use std::io;
//use miden::ProofFile;
use super::data::{InputFile, OutputFile, ProgramFile, ProofFile};

use std::path::PathBuf;

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
        s: "Test file!".to_string(),
    };

    let serialized = serialize(&data)?.to_vec();

    //let mut file = File::create("data.bin")?;
    //file.write_all(&serialized)?;
    println!("Test file written");

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

    let serializedproof = proof.to_bytes();
    let mut filer = File::create("proofd2.bin");
    filer?.write_all(&serializedproof)?;
    println!("Proof file written");

    //println!("{}", proof);

    Ok(())
}

pub fn read() -> io::Result<()> {
    let mut f = File::open("proofd2.bin")?;
    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;
    //let mut filer = File::create("proofd2.bin");
    //filer?.write_all(&serializedproof)?;
    println!("Proof file read");
    Ok(())
}

pub fn process_proof(proof_path: &Option<PathBuf>){
    //let proof : StarkProof = buffer.from_bytes();
    //proof_path: &Option<PathBuf>
    //let proof : StarkProof = from_bytes(buffer);

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

    let pfile: ProofFile;
    //let serializedproof = proof.to_bytes();
    pfile.write(proof, "./prooftest.proof");
    //println!("{}", proof);


    // the output should be 8
    // assert_eq!(vec![8], outputs);
    //tester();
    println!("All done!");
}
