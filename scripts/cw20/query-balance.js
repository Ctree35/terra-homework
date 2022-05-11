import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1g4h0t9qn3f7djcv9fv4tuzgr73rqvehvnlxqj0";
// const walletAddress = wallets.wallet1.key.accAddress;
const walletAddress = "terra1yvgrck4hh5vaz7frnhp3ncx0h0hafslkk0h3md"
const response = await client.wasm.contractQuery(
  // Address of CW20 contract.
  cw20Contract,
  // QueryMsg payload.
  {
    balance: {
      address: walletAddress
    }
  }
);

console.log(response);

