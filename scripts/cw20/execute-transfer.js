import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1g4h0t9qn3f7djcv9fv4tuzgr73rqvehvnlxqj0";
const wallet = wallets.wallet3;

const msg = new MsgExecuteContract(
    // Address of wallet that is signing the transaction
    wallet.key.accAddress,
    // Address of CW20 contract
    cw20Contract,
    // ExecuteMsg payload
    {
        transfer: {
          // Address of wallet or contract that is getting the tokens
          recipient: "terra1yvgrck4hh5vaz7frnhp3ncx0h0hafslkk0h3md",
          // Amount of tokens to transfer, in microunits
          amount: "100000000",
        },
    },
  );

  const tx = await wallet.createAndSignTx({ msgs: [msg] });
  const result = await client.tx.broadcast(tx);

console.log(result);