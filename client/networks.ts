import { BN } from 'bn.js';
import fs from 'mz/fs';

import {
	Account,
	createMint,
	getOrCreateAssociatedTokenAccount,
	mintTo,
	TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import {
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	sendAndConfirmTransaction,
	SystemProgram,
	Transaction,
	TransactionInstruction,
} from '@solana/web3.js';

import {
	createKeypairFromFile,
	getPayer,
	getRpcUrl,
} from './utils';
import {DECIMALS, PROGRAM_KEYPAIR_PATH, PROGRAM_SO_PATH, SOL_AMOUNT_TO_SWAP} from "./constants";

let connection: Connection;
let mint: PublicKey;
let solToken: Keypair;
let solAccount: Account;
let programId: PublicKey;
let otherTokenPublicKey: PublicKey;
let otherTokenAcount: Account;

export async function initApp() {
  await establishConnection();
  await createOrGetSolToken();
  await initSolMint();
  await createOtherToken();
  await addSolTokenMint();
  await checkProgramHashBeenDeployed();
  await establishOtherPublicKey();
  await establishOtherTokenAccount();
  await addOtherTokenMin();
  await initialize();
}

export async function establishConnection(): Promise<void> {
  const rpcUrl = await getRpcUrl();
  connection = new Connection(rpcUrl, "confirmed");
  const version = await connection.getVersion();

  console.log("Connection to cluster established:", rpcUrl, version);
}

export async function createOrGetSolToken(): Promise<void> {
  if (!solToken) {
    solToken = await getPayer();
  }

  await airdropSolIfNeeded(solToken.publicKey);
  console.log("Using Sol Token", solToken.publicKey.toBase58());
}

async function airdropSolIfNeeded(benificier: PublicKey) {
  const balance = await connection.getBalance(benificier);
  console.log("Current balance is", balance / LAMPORTS_PER_SOL);

  if (balance < LAMPORTS_PER_SOL) {
    console.log("Airdropping 1 SOL...");
    const sig = await connection.requestAirdrop(benificier, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(sig);

    console.log(
      "Current balance after airdrop: ",
      (await connection.getBalance(benificier)) / LAMPORTS_PER_SOL
    );
  }
}

export async function initSolMint(): Promise<void> {
  mint = await createMint(
    connection,
    solToken,
    solToken.publicKey,
    solToken.publicKey,
    DECIMALS
  );

  console.log(`Inti mint Sol Token ${mint.toBase58()}`);
}

export async function createOtherToken(): Promise<void> {
  solAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    solToken,
    mint,
    solToken.publicKey
  );

  console.log(`Using Other Token ${solAccount.address.toBase58()}`);
}

export async function addSolTokenMint(): Promise<void> {
  await mintTo(
    connection,
    solToken,
    mint,
    solAccount.address,
    solToken,
    1000 * LAMPORTS_PER_SOL
  );

  console.log(`Mint 1000 tokens to other Token ${solAccount.address.toBase58()}`);
}

export async function checkProgramHashBeenDeployed(): Promise<void> {
  // Read program id from keypair file
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/tokentrade.so\``
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        "Program needs to be deployed with `solana program deploy dist/program/tokentrade.so`"
      );
    } else {
      throw new Error("Program needs to be built and deployed");
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);
}

export async function establishOtherPublicKey(): Promise<void> {
  const [publicKey] = await PublicKey.findProgramAddress(
    [Buffer.from("vault"), mint.toBuffer()],
    programId
  );
  otherTokenPublicKey = publicKey;

  console.log(`Using token ${otherTokenPublicKey.toBase58()}`);
}

export async function establishOtherTokenAccount(): Promise<void> {
  otherTokenAcount = await getOrCreateAssociatedTokenAccount(
    connection,
    solToken,
    mint,
    otherTokenPublicKey,
    true
  );

  console.log(`Using other token account ${otherTokenAcount.address.toBase58()}`);
}

export async function addOtherTokenMin(): Promise<void> {
  await mintTo(
    connection,
    solToken,
    mint,
    otherTokenAcount.address,
    solToken,
    1000 * LAMPORTS_PER_SOL
  );

  console.log(`Mint 1000 tokens to other Token ${otherTokenAcount.address.toBase58()}`);
}

export async function initialize(): Promise<void> {
  const instructionData = Buffer.from(Uint8Array.of(0));
  const instruction = new TransactionInstruction({
    keys: [
      {
        pubkey: solToken.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: otherTokenPublicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mint,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: instructionData,
  });

  const txSig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [solToken]
  );
  console.log(
    `Finish initialize, more info: \nhttps://explorer.solana.com/tx/${txSig}?cluster=devnet`
  );

  await airdropSolIfNeeded(otherTokenPublicKey); //airdrop after create vault on-chain
}

export async function transferSolToToken(): Promise<void> {
  const instructionData = Buffer.from(
    Uint8Array.of(
      1,
      ...new BN(SOL_AMOUNT_TO_SWAP * LAMPORTS_PER_SOL).toArray("le", 8)
    )
  );

  const instruction = new TransactionInstruction({
    keys: [
      {
        pubkey: solToken.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: solAccount.address,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: programId,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mint,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: otherTokenPublicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: otherTokenAcount.address,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: instructionData,
  });

  const swapSig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [solToken]
  );
  console.log(
    `Finish transfer Sol to Token, more info:  \nhttps://explorer.solana.com/tx/${swapSig}?cluster=devnet`
  );
}

export async function transferTokenToSol(): Promise<void> {
  const instructionData = Buffer.from(
    Uint8Array.of(
      2,
      ...new BN(SOL_AMOUNT_TO_SWAP * 10 * LAMPORTS_PER_SOL).toArray("le", 8)
    )
  );

  const instruction = new TransactionInstruction({
    keys: [
      {
        pubkey: solToken.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: solAccount.address,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: programId,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: mint,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: otherTokenPublicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: otherTokenAcount.address,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: TOKEN_PROGRAM_ID,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: instructionData,
  });

  const swapSig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [solToken]
  );

  console.log(
    `Finish transfer Token to SOL, more info: \nhttps://explorer.solana.com/tx/${swapSig}?cluster=devnet`
  );
}
