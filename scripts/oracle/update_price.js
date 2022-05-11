import { client, wallets } from '../library.js';

import {
    MsgExecuteContract,
    MnemonicKey,
    Coins,
    LCDClient,
} from "@terra-money/terra.js";

const oracleContract = "terra1hh2qamsvhgjwpahfycawtzwttft8wxf0l4c926";
const wallet = wallets.wallet3;

const msg = new MsgExecuteContract(
    // Address of wallet that is signing the transaction
    wallet.key.accAddress,
    // Address of Oracle contract
    oracleContract,
    // ExecuteMsg payload
    {
        update_price: {
            // Price to be updated
            price: 102,
        },
    },
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);