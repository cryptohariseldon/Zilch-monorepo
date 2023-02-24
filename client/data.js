const web3 = require('@solana/web3.js');
const fs = require('fs');

//Location to write on-chain proof obtained from Bob.
const output_file = '../examples/Alice/data/fib_trace.proof'
//const accountAddress = 'GyS3HSjCQwU2sGsySvhL9Fa2bC8VufakoW4TpB5Bdfao';
const accountAddress = 'B6pwmZ5FrTkz2yWMWMkDcduzhojLggboDuNA7T4ReuvP';

async function getAccountBytecode() {
  // const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed"); //local
  const connection = new web3.Connection("https://api.devnet.solana.com", "confirmed") //devnet
  const accountInfo = await connection.getAccountInfo(new web3.PublicKey(accountAddress), "processed");
  //const accountInfo = await connection.getAccountInfo(accountAddress, "processed");
  const bytecodeData = accountInfo.data;
  const bytecode = Buffer.from(bytecodeData, 'base64');
  const bytecodeWithoutPadding = bytecode.slice(37);


  //const bytecodeData = accountInfo.data[0];

  //const bytecode = Buffer.from(bytecodeData, 'base64');
  fs.writeFileSync(output_file, bytecodeWithoutPadding);

  console.log(`Bytecode written to ${output_file}`);

  console.log(accountInfo.data);
}

getAccountBytecode();
