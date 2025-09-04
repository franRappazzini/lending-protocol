import * as anchor from "@coral-xyz/anchor";

const PUBKEYS = {
  DEV: {
    USDC: new anchor.web3.PublicKey("9BEcn9aPEmhSPbPQeFGjidRiEKki46fVQDyPpSQXPA2D"),
    LENDING_PROGRAM: new anchor.web3.PublicKey("9BEcn9aPEmhSPbPQeFGjidRiEKki46fVQDyPpSQXPA2D"),
  },
  MAIN: {
    USDC: new anchor.web3.PublicKey("9BEcn9aPEmhSPbPQeFGjidRiEKki46fVQDyPpSQXPA2D"),
    LENDING_PROGRAM: new anchor.web3.PublicKey("jup3YeL8QhtSx1e253b2FDvsMNC87fDrgQZivbrndc9"),
  },
};

export default PUBKEYS;
