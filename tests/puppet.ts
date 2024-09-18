import { PuppetMaster } from './../target/types/puppet_master';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Puppet } from "../target/types/puppet";
import { expect } from "chai";

describe("puppet", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const puppetProgram = anchor.workspace.Puppet as Program<Puppet>;
  const puppetMasterProgram = anchor.workspace.PuppetMaster as Program<PuppetMaster>;

  const puppetKeyPair = anchor.web3.Keypair.generate();
  const authorityKeypair= anchor.web3.Keypair.generate();

  it('Does CPI', async () => {
    await puppetProgram.methods.initialize(authorityKeypair.publicKey).accounts({
      puppet: puppetKeyPair.publicKey,
      user: provider.wallet.publicKey,
    }).signers([puppetKeyPair]).rpc();

    await puppetMasterProgram.methods.pullString(new anchor.BN(42)).accounts({
      puppetProgram: puppetProgram.programId,
      puppet: puppetKeyPair.publicKey,
      authority:authorityKeypair.publicKey,
    }).rpc()

    expect(
      (
        await puppetProgram.account.data.fetch(puppetKeyPair.publicKey)
      ).data.toNumber()
    ).to.equal(42)
  })




});
