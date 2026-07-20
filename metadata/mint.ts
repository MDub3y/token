import { getKeypairFromFile } from "@solana-developers/helpers";
import { TokenMetadata } from "@solana/spl-token-metadata";
import { getMintLen, ExtensionType, TYPE_SIZE, LENGTH_SIZE } from "@solana/spl-token";
import { clusterApiUrl, Connection, Keypair } from "@solana/web3.js";

const connection = new Connection("http://127.0.0.1:8899", "confirmed");

const payer = await getKeypairFromFile("~/.config/solana/id.json");
console.log("Payer: ", payer.publicKey.toBase58());

const mint = Keypair.generate();
console.log("mint", mint.publicKey.toBase58());

const metadata: TokenMetadata = {
    mint: mint.publicKey,
    name: "only possible on solana",
    symbol: "OPOS",
    uri: "https://c8.alamy.com/comp/3F058AT/spain-champions-2026-fifa-world-cup-2026-soccer-tournament-logo-with-castle-emblem-tshirt-tee-3F058AT.jpg",
    additionalMetadata: [
        ["a", "b"]
    ]
};

const mintSpace = getMintLen([
    ExtensionType.MetadataPointer
]);

const metadataSpace = TYPE_SIZE + LENGTH_SIZE;