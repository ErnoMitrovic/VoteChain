// Start the API connection to the local running RPC node:
import { GearApi } from '@gear-js/api';
import { getWasmMetadata } from '@gear-js/api';
import { GearKeyring } from '@gear-js/api';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { decodeAddress } from '@gear-js/api';


const provider = new WsProvider('wss://rpc-node.gear-tech.io');

const api = await ApiPromise.create({ provider });


const gearApi = await GearApi.create({
  providerAddress: 'wss://rpc-node.gear-tech.io',
});

import * as fs from 'fs';

const code = fs.readFileSync('dao.opt.wasm');
const fileBuffer = fs.readFileSync('dao.meta.wasm');
const meta = await getWasmMetadata(fileBuffer);

console.log('[*] PARTE 1: ')
const gas = await gearApi.program.calculateGas.initUpload(
  '0x5f994e98c1bbec1055cdb7867e89074a4388dc27a9eda8500a9cb87c88acc75f', // source id
  code,
  '0x00', // payload
  0, //value
  true, // allow other panics
);

//console.log(gas.toJSON());
console.log('Gas calculated!');

// Keyring
const keyring = await GearKeyring.fromSuri('//Alice');
//const seed = '0x3c660c5fc8363fa76692b63ee7d7e88e76a0ec4647733d49ef245971ea41d366';
//const keyring = await GearKeyring.fromSeed(seed, 'MoisesAdameWallet');
//const { keyring, json } = await GearKeyring.create('test3', 'poet confirm lake pond catch math setup umbrella man spring razor table');

//const seed = GearKeyring.generateSeed('poet confirm lake pond catch math setup umbrella man spring razor table')
/*
console.log('[*] PARTE 2: ')
console.log('[*] Seed: ' + seed.seed)
let b = await gearApi.balance.findOut(seed.seed);
console.log('[*] Balance: ' + b.toString());
*/

try {
  const message = {
    destination: '0x5f994e98c1bbec1055cdb7867e89074a4388dc27a9eda8500a9cb87c88acc75f', // programId
    payload: {
      "submitVote": {
          "origin": '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d',
          "proposalId": 0,
          "vote": "Yes"
      }
  },
    gasLimit: 10000000,
    value: 1000,
  };
  // In that case payload will be encoded using meta.handle_input type
  let extrinsic = gearApi.message.send(message, meta);
  // So if you want to use another type you can specify it
  extrinsic = gearApi.message.send(message, meta, meta.async_handle_input);
  await extrinsic.signAndSend(keyring, (event) => {
    console.log(event.toHuman());
    console.log('Message sent!');
  });
} catch (error) {
  console.error(`${error.name}: ${error.message}`);
}