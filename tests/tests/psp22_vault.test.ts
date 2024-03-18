import { ApiPromise } from '@polkadot/api';
import BN from 'bn.js';
import TPsp22Deployer from 'typechain/deployers/t_psp22';
import TPsp22Contract from 'typechain/contracts/t_psp22';
import TVault22Deployer from 'typechain/deployers/t_vault';
import TVaultContract from 'typechain/contracts/t_vault';
import 'wookashwackomytest-polkahat-chai-matchers';
import { expect } from 'chai';
import { Rounding } from 'typechain/types-arguments/t_vault';
import { getSigners, localApi } from 'wookashwackomytest-polkahat-network-helpers';

const name = 'My Token';
const symbol = 'MTKN';
const decimals: number = 12;

const MAX_U128 = new BN('340282366920938463463374607431768211455');

const [deployer, holder, recipient, spender, other, ...accounts] = getSigners();

let token: TPsp22Contract;
let vault: TVaultContract;

describe('ERC4626', function () {
  let api: ApiPromise;
  beforeEach(async () => {
    api = await localApi.get();
  });
  it('inherit decimals if from asset', async function () {
    for (const customDecimals of [0, 9, 12, 18, 36]) {
      token = (await new TPsp22Deployer(api, deployer).new(0, '', '', customDecimals)).contract;
      vault = (await new TVault22Deployer(api, deployer).new(token.address, 0, '', '', null)).contract;
      await expect(vault.query.tokenDecimals()).to.haveOkResult(customDecimals);
    }
  });

  it('asset has not yet been created', async function () {
    vault = (await new TVault22Deployer(api, deployer).new(other.address, 0, '', '', null)).contract;
    await expect(vault.query.tokenDecimals()).to.haveOkResult(decimals);
  });

  // removed excess underlying decimals

  it('decimals overflow', async function () {
    for (const offset of [1, 2, 3]) {
      token = (await new TPsp22Deployer(api, deployer).new(0, '', '', 255 - offset)).contract;
      vault = (await new TVault22Deployer(api, deployer).new(token.address, offset, '', '', null)).contract;
      await expect(vault.query.tokenDecimals()).to.haveOkResult(255);
    }
    for (const offset of [244, 250, 255]) {
      token = (await new TPsp22Deployer(api, deployer).new(0, '', '', decimals)).contract;
      vault = (await new TVault22Deployer(api, deployer).new(token.address, offset, '', '', null)).contract;
      await expect(vault.query.tokenDecimals()).to.be.eventually.rejected; //panics
    }
  });

  // removed reentrancy tests as our implementation doesn't allow for reentrency at all

  describe('limits', async function () {
    beforeEach(async function () {
      token = (await new TPsp22Deployer(api, deployer).new(0, '', '', decimals)).contract;
      vault = (await new TVault22Deployer(api, deployer).new(token.address, 0, '', '', MAX_U128.subn(1))).contract;
    });

    it('reverts on deposit() above max deposit', async function () {
      const maxDeposit = (await vault.query.maxDeposit(holder.address)).value.ok;
      await expect(vault.withSigner(holder).query.deposit(maxDeposit!.addn(1), recipient.address)).to.be.revertedWithError({
        custom: 'V:MaxDeposit',
      });
    });

    it('reverts on mint() above max mint', async function () {
      const maxMint = (await vault.query.maxMint(holder.address)).value.ok;
      await expect(vault.withSigner(holder).query.mint(maxMint!.addn(1), recipient.address)).to.be.revertedWithError({ custom: 'V:MaxMint' });
    });

    it('reverts on withdraw() above max withdraw', async function () {
      const maxWithdraw = (await vault.query.maxWithdraw(holder.address)).value.ok;
      await expect(vault.withSigner(holder).query.withdraw(maxWithdraw!.addn(1), recipient.address, holder.address)).to.be.revertedWithError({
        custom: 'V:MaxWithdraw',
      });
    });

    it('reverts on redeem() above max redeem', async function () {
      const maxRedeem = (await vault.query.maxRedeem(holder.address)).value.ok;
      await expect(vault.withSigner(holder).query.redeem(maxRedeem!.addn(1), recipient.address, holder.address)).to.be.revertedWithError({
        custom: 'V:MaxRedeem',
      });
    });
  });

  for (const offset of [0, 6, 18]) {
    const parseToken = (t: number) => new BN(t).mul(new BN(10).pow(new BN(decimals)));
    const parseShare = (share: number) => new BN(share).mul(new BN(10).pow(new BN(decimals + offset)));

    const virtualAssets = new BN(1);
    const virtualShares = new BN((10 ** offset).toString());

    describe(`offset: ${offset}`, function () {
      //   let token;
      //   let vault;
      beforeEach(async function () {
        token = (await new TPsp22Deployer(api, deployer).new(0, name, symbol, decimals)).contract;
        vault = (await new TVault22Deployer(api, deployer).new(token.address, offset, name + ' Vault', symbol + 'V', null)).contract;

        await token.tx.tMint(holder.address, new BN(MAX_U128).divn(2)); // 50% of maximum
        await token.withSigner(holder).tx.approve(vault.address, MAX_U128);
        await vault.withSigner(holder).tx.approve(spender.address, MAX_U128);
      });

      it('metadata', async function () {
        await expect(vault.query.tokenName()).to.haveOkResult(name + ' Vault');
        await expect(vault.query.tokenSymbol()).to.haveOkResult(symbol + 'V');
        await expect(vault.query.tokenDecimals()).to.haveOkResult(decimals + offset);
        await expect(vault.query.asset()).to.haveOkResult(token.address);
      });

      describe('empty vault: no assets & no shares', function () {
        it('status', async function () {
          await expect(vault.query.totalAssets()).to.haveOkResult(0);
        });

        it('deposit', async function () {
          await expect(vault.query.maxDeposit(holder.address)).to.haveOkResult(MAX_U128);
          await expect(vault.query.previewDeposit(parseToken(1))).to.haveOkResult(parseShare(1));

          const tx = vault.withSigner(holder).tx.deposit(parseToken(1), recipient.address);

          await expect(tx).to.changePSP22Balances(token, [holder.address, vault.address], [parseToken(1).neg(), parseToken(1)]);
          await expect(tx).to.changePSP22Balances(vault, [recipient.address], [parseShare(1)]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: holder.address, to: vault.address, value: parseToken(1) });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: recipient.address, value: parseShare(1) });
          await expect(tx).to.emitEvent(vault, 'Deposit', {
            sender: holder.address,
            owner: recipient.address,
            assets: parseToken(1),
            shares: parseShare(1),
          });
        });

        it('mint', async function () {
          await expect(vault.query.maxMint(holder.address)).to.haveOkResult(MAX_U128);
          await expect(vault.query.previewMint(parseShare(1))).to.haveOkResult(parseToken(1));

          const tx = vault.withSigner(holder).tx.mint(parseShare(1), recipient.address);

          await expect(tx).to.changePSP22Balances(token, [holder.address, vault.address], [parseToken(1).neg(), parseToken(1)]);
          await expect(tx).to.changePSP22Balances(vault, [recipient.address], [parseShare(1)]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: holder.address, to: vault.address, value: parseToken(1) });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: recipient.address, value: parseShare(1) });
          await expect(tx).to.emitEvent(vault, 'Deposit', {
            sender: holder.address,
            owner: recipient.address,
            assets: parseToken(1),
            shares: parseShare(1),
          });
        });

        it('withdraw', async function () {
          await expect(vault.query.maxWithdraw(holder.address)).to.haveOkResult(0n);
          await expect(vault.query.previewWithdraw(0)).to.haveOkResult(0n);

          const tx = vault.withSigner(holder).tx.withdraw(0, recipient.address, holder.address);

          await expect(tx).to.changePSP22Balances(token, [vault.address, recipient.address], [new BN(0), new BN(0)]);
          await expect(tx).to.changePSP22Balances(vault, [holder.address], [new BN(0)]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: recipient.address, value: new BN(0) });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: holder.address, to: null, value: new BN(0) });
          await expect(tx).to.emitEvent(vault, 'Withdraw', {
            owner: holder.address,
            sender: holder.address,
            receiver: recipient.address,
            assets: new BN(0),
            shares: new BN(0),
          });
        });

        it('redeem', async function () {
          await expect(vault.query.maxRedeem(holder.address)).to.haveOkResult(0);
          await expect(vault.query.previewRedeem(0)).to.haveOkResult(0);

          const tx = vault.withSigner(holder).tx.redeem(0, recipient.address, holder.address);

          await expect(tx).to.changePSP22Balances(token, [vault.address, recipient.address], [new BN(0), new BN(0)]);
          await expect(tx).to.changePSP22Balances(vault, [holder.address], [new BN(0)]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: recipient.address, value: new BN(0) });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: holder.address, to: null, value: new BN(0) });
          await expect(tx).to.emitEvent(vault, 'Withdraw', {
            owner: holder.address,
            sender: holder.address,
            receiver: recipient.address,
            assets: new BN(0),
            shares: new BN(0),
          });
        });
      });

      describe('inflation attack: offset price by direct deposit of assets', function () {
        beforeEach(async function () {
          // Donate 1 token to the vault to offset the price
          await token.tx.tMint(vault.address, parseToken(1));
        });

        it('status', async function () {
          await expect(vault.query.totalSupply()).to.haveOkResult(0);
          await expect(vault.query.totalAssets()).to.haveOkResult(parseToken(1));
        });

        /**
         * | offset | deposited assets     | redeemable assets    |
         * |--------|----------------------|----------------------|
         * | 0      | 1.000000000000000000 | 0.                   |
         * | 6      | 1.000000000000000000 | 0.999999000000000000 |
         * | 18     | 1.000000000000000000 | 0.999999999999999999 |
         *
         * Attack is possible, but made difficult by the offset. For the attack to be successful
         * the attacker needs to frontrun a deposit 10**offset times bigger than what the victim
         * was trying to deposit
         */
        it('deposit', async function () {
          const effectiveAssets: BN = (await vault.query.totalAssets()).value.ok!.add(virtualAssets);
          const effectiveShares: BN = (await vault.query.totalSupply()).value.ok!.add(virtualShares);

          const depositAssets: BN = parseToken(1);
          const expectedShares = depositAssets.mul(effectiveShares).div(effectiveAssets);

          await expect(vault.query.maxDeposit(holder.address)).to.haveOkResult(MAX_U128);
          await expect(vault.query.previewDeposit(depositAssets)).to.haveOkResult(expectedShares);

          const tx = vault.withSigner(holder).tx.deposit(depositAssets, recipient.address);

          await expect(tx).to.changePSP22Balances(token, [holder.address, vault.address], [depositAssets.neg(), depositAssets]);
          await expect(tx).to.changePSP22Balances(vault, [recipient.address], [expectedShares]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: holder.address, to: vault.address, value: depositAssets });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: recipient.address, value: expectedShares });
          await expect(tx).to.emitEvent(vault, 'Deposit', {
            sender: holder.address,
            owner: recipient.address,
            assets: depositAssets,
            shares: expectedShares,
          });
        });
        /**
         * | offset | deposited assets     | redeemable assets    |
         * |--------|----------------------|----------------------|
         * | 0      | 1000000000000000001. | 1000000000000000001. |
         * | 6      | 1000000000000000001. | 1000000000000000001. |
         * | 18     | 1000000000000000001. | 1000000000000000001. |
         *
         * Using mint protects against inflation attack, but makes minting shares very expensive.
         * The ER20 allowance for the underlying asset is needed to protect the user from (too)
         * large deposits.
         */
        it('mint', async function () {
          const effectiveAssets: BN = (await vault.query.totalAssets()).value.ok!.add(virtualAssets);
          const effectiveShares: BN = (await vault.query.totalSupply()).value.ok!.add(virtualShares);

          const mintShares: BN = parseShare(1);
          const expectedAssets = mintShares.mul(effectiveAssets).div(effectiveShares);

          await expect(vault.query.maxMint(holder.address)).to.haveOkResult(MAX_U128);
          await expect(vault.query.previewMint(mintShares)).to.haveOkResult(expectedAssets);

          const tx = vault.withSigner(holder).tx.mint(mintShares, recipient.address);

          await expect(tx).to.changePSP22Balances(token, [holder.address, vault.address], [expectedAssets.neg(), expectedAssets]);
          await expect(tx).to.changePSP22Balances(vault, [recipient.address], [mintShares]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: holder.address, to: vault.address, value: expectedAssets });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: recipient.address, value: mintShares });
          await expect(tx).to.emitEvent(vault, 'Deposit', {
            sender: holder.address,
            owner: recipient.address,
            assets: expectedAssets,
            shares: mintShares,
          });
        });

        it('withdraw', async function () {
          await expect(vault.query.maxWithdraw(holder.address)).to.haveOkResult(0);
          await expect(vault.query.previewWithdraw(0)).to.haveOkResult(0);

          const tx = vault.withSigner(holder).tx.withdraw(0, recipient.address, holder.address);

          await expect(tx).to.changePSP22Balances(token, [vault.address, recipient.address], [new BN(0), new BN(0)]);
          await expect(tx).to.changePSP22Balances(vault, [holder.address], [new BN(0)]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: recipient.address, value: new BN(0) });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: holder.address, to: null, value: new BN(0) });
          await expect(tx).to.emitEvent(vault, 'Withdraw', {
            owner: holder.address,
            sender: holder.address,
            receiver: recipient.address,
            assets: new BN(0),
            shares: new BN(0),
          });
        });

        it('redeem', async function () {
          await expect(vault.query.maxRedeem(holder.address)).to.haveOkResult(0);
          await expect(vault.query.previewRedeem(0)).to.haveOkResult(0);

          const tx = vault.withSigner(holder).tx.redeem(0, recipient.address, holder.address);

          await expect(tx).to.changePSP22Balances(token, [vault.address, recipient.address], [new BN(0), new BN(0)]);
          await expect(tx).to.changePSP22Balances(vault, [holder.address], [new BN(0)]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: recipient.address, value: new BN(0) });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: holder.address, to: null, value: new BN(0) });
          await expect(tx).to.emitEvent(vault, 'Withdraw', {
            owner: holder.address,
            sender: holder.address,
            receiver: recipient.address,
            assets: new BN(0),
            shares: new BN(0),
          });
        });
      });

      describe('full vault: assets & shares', function () {
        beforeEach(async function () {
          // Add 1 token of underlying asset and 100 shares to the vault
          await token.tx.tMint(vault.address, parseToken(1));
          await vault.tx.tMint(holder.address, parseShare(100));
        });

        it('status', async function () {
          await expect(vault.query.totalSupply()).to.haveOkResult(parseShare(100));
          await expect(vault.query.totalAssets()).to.haveOkResult(parseToken(1));
        });

        /**
         * | offset | deposited assets     | redeemable assets    |
         * |--------|--------------------- |----------------------|
         * | 0      | 1.000000000000000000 | 0.999999999999999999 |
         * | 6      | 1.000000000000000000 | 0.999999999999999999 |
         * | 18     | 1.000000000000000000 | 0.999999999999999999 |
         *
         * Virtual shares & assets captures part of the value
         */
        it('deposit', async function () {
          const effectiveAssets: BN = (await vault.query.totalAssets()).value.ok!.add(virtualAssets);
          const effectiveShares: BN = (await vault.query.totalSupply()).value.ok!.add(virtualShares);

          const depositAssets: BN = parseToken(1);
          const expectedShares = depositAssets.mul(effectiveShares).div(effectiveAssets);

          await expect(vault.query.maxDeposit(holder.address)).to.haveOkResult(MAX_U128);
          await expect(vault.query.previewDeposit(depositAssets)).to.haveOkResult(expectedShares);

          const tx = vault.withSigner(holder).tx.deposit(depositAssets, recipient.address);

          await expect(tx).to.changePSP22Balances(token, [holder.address, vault.address], [depositAssets.neg(), depositAssets]);
          await expect(tx).to.changePSP22Balances(vault, [recipient.address], [expectedShares]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: holder.address, to: vault.address, value: depositAssets });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: recipient.address, value: expectedShares });
          await expect(tx).to.emitEvent(vault, 'Deposit', {
            sender: holder.address,
            owner: recipient.address,
            assets: depositAssets,
            shares: expectedShares,
          });
        });

        /**
         * | offset | deposited assets     | redeemable assets    |
         * |--------|--------------------- |----------------------|
         * | 0      | 0.010000000000000001 | 0.010000000000000000 |
         * | 6      | 0.010000000000000001 | 0.010000000000000000 |
         * | 18     | 0.010000000000000001 | 0.010000000000000000 |
         *
         * Virtual shares & assets captures part of the value
         */
        it('mint', async function () {
          const effectiveAssets: BN = (await vault.query.totalAssets()).value.ok!.add(virtualAssets);
          const effectiveShares: BN = (await vault.query.totalSupply()).value.ok!.add(virtualShares);

          const mintShares: BN = parseShare(1);
          const expectedAssets = mintShares.mul(effectiveAssets).div(effectiveShares).addn(1); // add for the rounding up

          await expect(vault.query.maxMint(holder.address)).to.haveOkResult(MAX_U128);
          await expect(vault.query.previewMint(mintShares)).to.haveOkResult(expectedAssets);

          const tx = vault.withSigner(holder).tx.mint(mintShares, recipient.address);

          await expect(tx).to.changePSP22Balances(token, [holder.address, vault.address], [expectedAssets.neg(), expectedAssets]);
          await expect(tx).to.changePSP22Balances(vault, [recipient.address], [mintShares]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: holder.address, to: vault.address, value: expectedAssets });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: recipient.address, value: mintShares });
          await expect(tx).to.emitEvent(vault, 'Deposit', {
            sender: holder.address,
            owner: recipient.address,
            assets: expectedAssets,
            shares: mintShares,
          });
        });

        it('withdraw', async function () {
          const effectiveAssets: BN = (await vault.query.totalAssets()).value.ok!.add(virtualAssets);
          const effectiveShares: BN = (await vault.query.totalSupply()).value.ok!.add(virtualShares);

          const withdrawAssets: BN = parseToken(1);
          const expectedShares = withdrawAssets.mul(effectiveShares).div(effectiveAssets).addn(1); // add for the rounding

          await expect(vault.query.maxWithdraw(holder.address)).to.haveOkResult(withdrawAssets);
          await expect(vault.query.previewWithdraw(withdrawAssets)).to.haveOkResult(expectedShares);

          const tx = vault.withSigner(holder).tx.withdraw(withdrawAssets, recipient.address, holder.address);

          await expect(tx).to.changePSP22Balances(token, [vault.address, recipient.address], [withdrawAssets.neg(), withdrawAssets]);
          await expect(tx).to.changePSP22Balances(vault, [holder.address], [expectedShares.neg()]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: recipient.address, value: withdrawAssets });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: holder.address, to: null, value: expectedShares });
          await expect(tx).to.emitEvent(vault, 'Withdraw', {
            owner: holder.address,
            sender: holder.address,
            receiver: recipient.address,
            assets: withdrawAssets,
            shares: expectedShares,
          });
        });

        it('withdraw with approval', async function () {
          const assets = (await vault.query.previewWithdraw(parseToken(1))).value.ok;

          await expect(vault.withSigner(other).query.withdraw(parseToken(1), recipient.address, holder.address)).to.be.revertedWithError({
            insufficientAllowance: null,
          });

          await expect(vault.withSigner(spender).tx.withdraw(parseToken(1), recipient.address, holder.address)).to.eventually.be.fulfilled;
        });

        it('redeem', async function () {
          const effectiveAssets = (await vault.query.totalAssets()).value.ok!.add(virtualAssets);
          const effectiveShares = (await vault.query.totalSupply()).value.ok!.add(virtualShares);

          const redeemShares = parseShare(100);
          const expectedAssets = redeemShares.mul(effectiveAssets).div(effectiveShares);

          await expect(vault.query.maxRedeem(holder.address)).to.haveOkResult(redeemShares);
          await expect(vault.query.previewRedeem(redeemShares)).to.haveOkResult(expectedAssets);

          const tx = vault.withSigner(holder).tx.redeem(redeemShares, recipient.address, holder.address);

          await expect(tx).to.changePSP22Balances(token, [vault.address, recipient.address], [expectedAssets.neg(), expectedAssets]);
          await expect(tx).to.changePSP22Balances(vault, [holder.address], [redeemShares.neg()]);
          await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: recipient.address, value: expectedAssets });
          await expect(tx).to.emitEvent(vault, 'Transfer', { from: holder.address, to: null, value: redeemShares });
          await expect(tx).to.emitEvent(vault, 'Withdraw', {
            owner: holder.address,
            sender: holder.address,
            receiver: recipient.address,
            assets: expectedAssets,
            shares: redeemShares,
          });
        });

        it('redeem with approval', async function () {
          await expect(vault.withSigner(other).query.redeem(parseShare(100), recipient.address, holder.address)).to.be.revertedWithError({
            insufficientAllowance: null,
          });
          await expect(vault.withSigner(spender).tx.redeem(parseShare(100), recipient.address, holder.address)).to.eventually.be.fulfilled;
        });
      });
    });
  }

  // removed Fees test. One should test fees by oneself.

  /// Scenario inspired by solmate ERC4626 tests:
  /// https://github.com/transmissions11/solmate/blob/main/src/test/ERC4626.t.sol
  it('multiple mint, deposit, redeem & withdrawal', async function () {
    // test designed with both asset using similar decimals
    const [alice, bruce] = accounts;
    token = (await new TPsp22Deployer(api, deployer).new(0, name, symbol, 18)).contract;
    vault = (await new TVault22Deployer(api, deployer).new(token.address, 0, '', '', null)).contract;

    await token.tx.tMint(alice.address, 4000);
    await token.tx.tMint(bruce.address, 7001);
    await token.withSigner(alice).tx.approve(vault.address, 4000);
    await token.withSigner(bruce).tx.approve(vault.address, 7001);

    let tx;
    // 1. Alice mints 2000 shares (costs 2000 tokens)
    tx = await vault.withSigner(alice).tx.mint(2000, alice.address);

    await expect(tx).to.emitEvent(token, 'Transfer', { from: alice.address, to: vault.address, value: 2000 });
    await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: alice.address, value: 2000 });

    await expect(vault.query.previewDeposit(2000)).to.haveOkResult(2000);
    await expect(vault.query.balanceOf(alice.address)).to.haveOkResult(2000);
    await expect(vault.query.balanceOf(bruce.address)).to.haveOkResult(0);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(2000);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(0);
    await expect(vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(2000);
    await expect(vault.query.totalSupply()).to.haveOkResult(2000);
    await expect(vault.query.totalAssets()).to.haveOkResult(2000);

    // 2. Bruce deposits 4000 tokens (mints 4000 shares)
    tx = await vault.withSigner(bruce).tx.mint(4000, bruce.address);

    await expect(tx).to.emitEvent(token, 'Transfer', { from: bruce.address, to: vault.address, value: 4000 });
    await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: bruce.address, value: 4000 });

    await expect(vault.query.previewDeposit(4000)).to.haveOkResult(4000);
    await expect(vault.query.balanceOf(alice.address)).to.haveOkResult(2000);
    await expect(vault.query.balanceOf(bruce.address)).to.haveOkResult(4000);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(2000);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(4000);
    await expect(vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(6000);
    await expect(vault.query.totalSupply()).to.haveOkResult(6000);
    await expect(vault.query.totalAssets()).to.haveOkResult(6000);

    // 3. Vault mutates by +3000 tokens (simulated yield returned from strategy)
    await token.tx.tMint(vault.address, 3000);

    expect(await vault.query.balanceOf(alice.address)).to.haveOkResult(2000);
    expect(await vault.query.balanceOf(bruce.address)).to.haveOkResult(4000);
    expect(await vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(2999);
    expect(await vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(5999);
    expect(await vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(6000);
    expect(await vault.query.totalSupply()).to.haveOkResult(6000);
    expect(await vault.query.totalAssets()).to.haveOkResult(9000);

    // // 4. Alice deposits 2000 tokens (mints 1333 shares)
    tx = await vault.withSigner(alice).tx.deposit(2000, alice.address);

    await expect(tx).to.emitEvent(token, 'Transfer', { from: alice.address, to: vault.address, value: 2000 });
    await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: alice.address, value: 1333 });

    await expect(vault.query.balanceOf(alice.address)).to.haveOkResult(3333);
    await expect(vault.query.balanceOf(bruce.address)).to.haveOkResult(4000);

    await expect(vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(4999);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(6000);
    await expect(vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(7333);
    await expect(vault.query.totalSupply()).to.haveOkResult(7333);
    await expect(vault.query.totalAssets()).to.haveOkResult(11000);

    // 5. Bruce mints 2000 shares (costs 3001 assets)
    // NOTE: Bruce's assets spent got rounded towards infinity
    // NOTE: Alices's vault assets got rounded towards infinity
    tx = await vault.withSigner(bruce).tx.mint(2000, bruce.address);

    await expect(tx).to.emitEvent(token, 'Transfer', { from: bruce.address, to: vault.address, value: 3000 });
    await expect(tx).to.emitEvent(vault, 'Transfer', { from: null, to: bruce.address, value: 2000 });

    await expect(vault.query.balanceOf(alice.address)).to.haveOkResult(3333);
    await expect(vault.query.balanceOf(bruce.address)).to.haveOkResult(6000);

    await expect(vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(4999); // used to be 5000
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(9000);
    await expect(vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(9333);
    await expect(vault.query.totalSupply()).to.haveOkResult(9333);
    await expect(vault.query.totalAssets()).to.haveOkResult(14000); // used to be 14001

    // 6. Vault mutates by +3000 tokens
    await token.tx.tMint(vault.address, 3000);

    expect(await vault.query.balanceOf(alice.address)).to.haveOkResult(3333);
    expect(await vault.query.balanceOf(bruce.address)).to.haveOkResult(6000);
    expect(await vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(6070); // used to be 6071
    expect(await vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(10928); // used to be 10929
    expect(await vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(9333);
    expect(await vault.query.totalSupply()).to.haveOkResult(9333);
    expect(await vault.query.totalAssets()).to.haveOkResult(17000); // used to be 17001

    // 7. Alice redeem 1333 shares (2428 assets)
    tx = await vault.withSigner(alice).tx.redeem(1333, alice.address, alice.address);
    await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: alice.address, value: 2427 }); //used to be 2428
    await expect(tx).to.emitEvent(vault, 'Transfer', { from: alice.address, to: null, value: 1333 });

    await expect(vault.query.balanceOf(alice.address)).to.haveOkResult(2000);
    await expect(vault.query.balanceOf(bruce.address)).to.haveOkResult(6000);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(3643);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(10929);
    await expect(vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(8000);
    await expect(vault.query.totalSupply()).to.haveOkResult(8000);
    await expect(vault.query.totalAssets()).to.haveOkResult(14573);

    // 8. Bruce withdraws 2929 assets (1608 shares)
    tx = await vault.withSigner(bruce).tx.withdraw(2929, bruce.address, bruce.address);

    await expect(tx).to.emitEvent(vault, 'Transfer', { from: bruce.address, to: null, value: 1608 });
    await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: bruce.address, value: 2929 });

    await expect(vault.query.balanceOf(alice.address)).to.haveOkResult(2000);
    await expect(vault.query.balanceOf(bruce.address)).to.haveOkResult(4392);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(3643);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(8000);
    await expect(vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(6392);
    await expect(vault.query.totalSupply()).to.haveOkResult(6392);
    await expect(vault.query.totalAssets()).to.haveOkResult(11644);

    // 9. Alice withdraws 3643 assets (2000 shares)
    // NOTE: Bruce's assets have been rounded back towards infinity
    tx = await vault.withSigner(alice).tx.withdraw(3643, alice.address, alice.address);

    await expect(tx).to.emitEvent(vault, 'Transfer', { from: alice.address, to: null, value: 2000 });
    await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: alice.address, value: 3643 });

    await expect(vault.query.balanceOf(alice.address)).to.haveOkResult(0);
    await expect(vault.query.balanceOf(bruce.address)).to.haveOkResult(4392);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(0);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(8000); // used to be 8001
    await expect(vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(4392);
    await expect(vault.query.totalSupply()).to.haveOkResult(4392);
    await expect(vault.query.totalAssets()).to.haveOkResult(8001);

    // 10. Bruce redeem 4392 shares (8001 tokens)
    tx = await vault.withSigner(bruce).tx.redeem(4392, bruce.address, bruce.address);

    await expect(tx).to.emitEvent(vault, 'Transfer', { from: bruce.address, to: null, value: 4392 });
    await expect(tx).to.emitEvent(token, 'Transfer', { from: vault.address, to: bruce.address, value: 8000 }); // used to be 8001

    await expect(vault.query.balanceOf(alice.address)).to.haveOkResult(0);
    await expect(vault.query.balanceOf(bruce.address)).to.haveOkResult(0);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(alice.address)).value.ok!, Rounding.down)).to.haveOkResult(0);
    await expect(vault.query.convertToAssets((await vault.query.balanceOf(bruce.address)).value.ok!, Rounding.down)).to.haveOkResult(0);
    await expect(vault.query.convertToShares((await token.query.balanceOf(vault.address)).value.ok!, Rounding.down)).to.haveOkResult(0);
    await expect(vault.query.totalSupply()).to.haveOkResult(0);
    await expect(vault.query.totalAssets()).to.haveOkResult(1); // used to be 0
  });
});
