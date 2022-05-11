import { client, wallets } from '../library.js';

import {
    MsgExecuteContract,
    Coin, Coins,
} from "@terra-money/terra.js";

// Address of the Lemon Swap contract.
const contract = "terra1ee8c8xfrwtj60c720l6hvzqguatjklu5qxgd2u";
// Wallet to use. Make sure to use the right wallet from library.js.
const wallet = wallets.wallet3;

const amount = (0.1 * 1e6).toFixed(0); // 0.5 Luna

const msg = new MsgExecuteContract(
  // Address of person who's signing the transaction.
  wallet.key.accAddress,
  // Address of contract to execute.
  contract,
  // ExecuteMsg payload
  {
    buy: {},
  },
  // Send Luna with this execute message.
  new Coins([new Coin("uluna", amount)]),
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);