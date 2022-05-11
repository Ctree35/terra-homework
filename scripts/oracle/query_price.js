import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1hh2qamsvhgjwpahfycawtzwttft8wxf0l4c926";

const response = await client.wasm.contractQuery(
  // Address of Oracle contract.
  cw20Contract,
  // QueryMsg payload.
  {
    query_price: {}
  }
);

console.log(response);

