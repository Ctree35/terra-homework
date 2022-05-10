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
        mint: {
            // Address of wallet or contract that is getting the tokens
            recipient: "terra18yfx9skmexwxvpxal6wtc5unjvcc68xtgl0l56",
            // Amount of tokens to transfer, in microunits
            amount: "1000000",
        },
    },
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);