import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1wlkefxppr029xezhpeude9vz0vuwnuhkvk470z";

const response = await client.wasm.contractQuery(
  // Address of Oracle contract.
  cw20Contract,
  // QueryMsg payload.
  {
    query_price: {}
  }
);

console.log(response);

