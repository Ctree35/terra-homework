import { client, wallets } from '../library.js';

const contract = "terra1yvgrck4hh5vaz7frnhp3ncx0h0hafslkk0h3md";

const response = await client.wasm.contractQuery(contract, { query_token_address: {} });

console.log(response);