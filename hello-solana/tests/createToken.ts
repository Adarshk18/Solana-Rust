import { LAMPORTS_PER_SOL, SystemProgram, Transaction, sendAndConfirmTransaction, Keypair, Connection } from "@solana/web3.js";
import {MINT_SIZE, TOKEN_2022_PROGRAM_ID, createInitializeMint2Instruction, getMinimumBalanceForRentExemptMint, getMint}  from "@solana/spl-token";

const connection = new Connection("http://localhost:8899", "confirmed");

const wallet = new Keypair();

const signature = await connection.requestAirdrop(
    wallet.publicKey,
    LAMPORTS_PER_SOL
);

await connection.confirmTransaction(signature, "confirmed");

//Generate keypair to use address of the mint account
const mint = new Keypair();

//calculate the minimum balance for rent exemption of the mint account
const rentExemptionLamports = await getMinimumBalanceForRentExemptMint(connection);

//Instruction to create the mint account
const createAccountInstruction = SystemProgram.createAccount({
    fromPubkey: wallet.publicKey,
    newAccountPubkey: mint.publicKey,
    space: MINT_SIZE,
    lamports: rentExemptionLamports,
    programId: TOKEN_2022_PROGRAM_ID
});


//Instruction to initialize the mint account
const initializeMintInstruction = createInitializeMint2Instruction(
    mint.publicKey,
    2,
    wallet.publicKey,
    wallet.publicKey,
    TOKEN_2022_PROGRAM_ID
);


//Build transaction with instructions to create new account and initialize the mint
const transaction = new Transaction().add(
    createAccountInstruction,
    initializeMintInstruction
);

const transactionSignature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [wallet, mint]
);

console.log("Transaction Signature:", `${transactionSignature}`);

const mintAccount = await getMint(
    connection,
    mint.publicKey,
    "confirmed",
    TOKEN_2022_PROGRAM_ID
);

console.log("Mint Account:", mintAccount);
