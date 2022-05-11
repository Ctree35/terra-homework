import { MsgSend } from "@terra-money/terra.js";
import { client, wallets } from '../library.js';

const send = new MsgSend(
  wallets.wallet3.key.accAddress, // from
  "terra1yvgrck4hh5vaz7frnhp3ncx0h0hafslkk0h3md", // to
  { uusd: "6000000" }
);

const tx = await wallets.wallet3.createAndSignTx({ msgs: [send] });
const result = await client.tx.broadcast(tx);

console.log(result);


