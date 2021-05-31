
import {
    Keypair,
    PublicKey,
    Connection,
    SystemProgram,
    Account

} from '@solana/web3.js';

import { readFile } from 'mz/fs';
import { Buffer } from 'buffer';
import * as BufferLayout from 'buffer-layout';
import * as BN from "bn.js";
var pathUtil = require('path');
export const PROGRAM_PATH = pathUtil.resolve(__dirname);
export const STREAM_SIZE = 232;

export const enum PROGRAM_ACTIONS {
    createStream = 1,
    addFunds = 2,
    withdraw = 3,
    proposeUpdate = 4,
    answerUpdate = 5,
    closeStream = 6,
    closeTreasury = 7
}

export const AVAILABLE_PROGRAM_ACTIONS = [
    { id: PROGRAM_ACTIONS.createStream, name: "Create Stream" },
    { id: PROGRAM_ACTIONS.closeStream, name: "Close Stream" },
]

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
    return await getAccount('dist/money_streaming-keypair.json');
}

export async function getPayerAccount() {
    return await getAccount('keys/payer-keypair.json');
}

export const publicKey = (property: string = 'publicKey'): Object => {
    return BufferLayout.blob(32, property);
};

export const string = (property: string = 'string'): Object => {
    const layout = BufferLayout.blob(32, property);

    layout.decode = (buffer: Buffer) => {
        return String.fromCharCode.apply(null, new Uint16Array(buffer));
    };

    layout.encode = (str: String) => {
        var buf = new ArrayBuffer(str.length * 2); // 2 bytes for each char
        var bufView = new Uint16Array(buf);
        for (var i = 0, strLen = str.length; i < strLen; i++) {
            bufView[i] = str.charCodeAt(i);
        }
        return buf;
    };

    return layout;
};

export const uint64 = (property = "uint64"): unknown => {
    const layout = BufferLayout.blob(8, property);

    const _decode = layout.decode.bind(layout);
    const _encode = layout.encode.bind(layout);

    layout.decode = (buffer: Buffer, offset: number) => {
        const data = _decode(buffer, offset);
        return new BN(
            [...data]
                .reverse()
                .map((i) => `00${i.toString(16)}`.slice(-2))
                .join(""),
            16
        );
    };

    layout.encode = (num: BN, buffer: Buffer, offset: number) => {
        const a = num.toArray().reverse();
        let b = Buffer.from(a);
        if (b.length !== 8) {
            const zeroPad = Buffer.alloc(8);
            b.copy(zeroPad);
            b = zeroPad;
        }
        return _encode(b, buffer, offset);
    };

    return layout;
};

export const StreamLayout: typeof BufferLayout.Structure = BufferLayout.struct([
    BufferLayout.u8('tag'),
    BufferLayout.u8('initialized'),
    publicKey('stream_id'),
    string('stream_name'),
    publicKey('treasurer_address'),
    BufferLayout.nu64('rate_amount'),
    BufferLayout.nu64('rate_interval_in_seconds'),
    BufferLayout.nu64('start_utc'),
    BufferLayout.nu64('rate_cliff_in_seconds'),
    BufferLayout.nu64('cliff_vest_amount'),
    BufferLayout.nu64('cliff_vest_percent'),
    publicKey('beneficiary_withdrawal_address'),
    publicKey('escrow_token_address'),
    BufferLayout.nu64('escrow_vested_amount'),
    BufferLayout.nu64('escrow_unvested_amount'),
    publicKey('treasury_address'),
    BufferLayout.nu64('escrow_estimated_depletion_utc'),
    BufferLayout.nu64('total_deposits'),
    BufferLayout.nu64('total_withdrawals')
]);