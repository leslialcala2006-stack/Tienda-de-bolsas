import * as web3 from "@solana/web3.js";
console.log("Dirección del cliente:", pg.wallet.publicKey.toString());
const balanceLamports = await pg.connection.getBalance(pg.wallet.publicKey);
const balanceSol = balanceLamports / web3.LAMPORTS_PER_SOL;
console.log(`Saldo disponible: ${balanceSol} SOL`);
