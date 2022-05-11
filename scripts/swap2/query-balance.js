import { client, wallets } from '../library.js';

const contract = "terra1ee8c8xfrwtj60c720l6hvzqguatjklu5qxgd2u";

const response = await client.wasm.contractQuery(contract, { query_balance: {} });

console.log(response);