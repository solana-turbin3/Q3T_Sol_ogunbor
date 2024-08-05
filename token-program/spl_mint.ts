import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import wallet from "../../wba-wallet.json";

// Load keypair from the imported wallet
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000_000n;

// Mint address
const mint = new PublicKey("JCCywbXaVhsiFQGG96k3MwWMYozkjhwCuzL8pKdi63MA");

(async () => {
  try {
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair, // payer
      mint, // mint address
      keypair.publicKey // owner address
    );

    const mintTx = await mintTo(
      connection,
      keypair, // payer
      mint, // mint address
      ata.address, // destination address (newly created ata)
      keypair.publicKey, // mint authority
      1_000n * token_decimals // amount, this will mint 1000 new tokens
    );

    console.log(`Your mint txid: ${mintTx}`);
    console.log(
      `Transaction: https://explorer.solana.com/tx/${mintTx}?cluster=devnet`
    );
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();