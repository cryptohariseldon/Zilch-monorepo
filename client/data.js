const web3 = require('@solana/web3.js');

const accountAddress = 'GyS3HSjCQwU2sGsySvhL9Fa2bC8VufakoW4TpB5Bdfao';

async function getAccountBytecode() {
  const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed");

  const accountInfo = await connection.getAccountInfo(new web3.PublicKey(accountAddress), "processed");
  //const accountInfo = await connection.getAccountInfo(accountAddress, "processed");

  //const bytecodeData = accountInfo.data[0];

  //const bytecode = Buffer.from(bytecodeData, 'base64');

  console.log(accountInfo);
}

getAccountBytecode();
