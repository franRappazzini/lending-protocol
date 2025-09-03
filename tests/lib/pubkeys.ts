import * as anchor from "@coral-xyz/anchor";

const PUBKEYS = {
  DEV: {
    USDC: new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
    LENDING_PROGRAM: new anchor.web3.PublicKey(""),
  },
  MAIN: {
    USDC: new anchor.web3.PublicKey(""),
    LENDING_PROGRAM: new anchor.web3.PublicKey("jup3YeL8QhtSx1e253b2FDvsMNC87fDrgQZivbrndc9"),
  },
};

export default PUBKEYS;
