
import {
    Keypair,
    PublicKey,
    Connection,
    SystemProgram,
    Account

} from '@solana/web3.js';

import { readFile } from 'mz/fs';
import { Buffer } from 'buffer';
import BN from 'bn.js';
// import BufferLayout from 'buffer-layout';
var pathUtil = require('path');
export const PROGRAM_PATH = pathUtil.resolve(__dirname);
export const STREAM_SIZE = 232;
export const PROGRAM_ID = '2HEkjrj21DX2ecNQjAEUPKwr2pEnwSBjgi9GUHWtKnhH';

export const enum PROGRAM_ACTIONS {
    createStream = 1,
    addFunds = 2,
    withdraw = 3,
    proposeUpdate = 4,
    answerUpdate = 5,
    closeStream = 6,
    closeTreasury = 7,
    listStreams = 8,
    getStream = 9
}

export const AVAILABLE_PROGRAM_ACTIONS = [
    { id: PROGRAM_ACTIONS.createStream, name: "Create Stream" },
    { id: PROGRAM_ACTIONS.closeStream, name: "Close Stream" },
]

export type StreamTerms = {

}

export async function createConnection(url = "https://devnet.solana.com") {
    return new Connection(url);
}

export async function createAccountInstruction(
    publicKey: PublicKey,
    lamports: number,
    space: number) {

    const program = await getProgramAccount();
    const payer = await getPayerAccount();
    // const newAccount = Keypair.generate();

    return SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: publicKey,
        lamports,
        space,
        programId: program.publicKey
    });
}

async function getAccount(path: string) {
    const programFilePath = pathUtil.join(PROGRAM_PATH, path);
    const programKeyPair = await readFile(programFilePath, { encoding: 'utf8' });
    const programKeyPairBuffer = Buffer.from(JSON.parse(programKeyPair));
    const program = new Account(programKeyPairBuffer);

    return program;
}

export async function getProgramAccount() {
    return await getAccount('../../program/dist/money_streaming-keypair.json');
}

export async function getPayerAccount() {
    return await getAccount('../../program/keys/payer-keypair.json');
}

export function toBuffer(value: number): Buffer {
    let bn = new BN(value);
    const a = bn.toArray().reverse();
    const b = Buffer.from(a);
    const zeroPad = Buffer.alloc(8);
    b.copy(zeroPad);

    return zeroPad;
}