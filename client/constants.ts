import path from "path";

export const DECIMALS = 9;
export const SOL_AMOUNT_TO_SWAP = 0.1;
export const PROGRAM_PATH = path.resolve(__dirname, "../dist/program");
export const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, "tokentrade.so");
export const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, "TokenTrade-keypair.json");
