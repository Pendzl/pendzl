import { ApiPromise } from '@polkadot/api';
import BN from 'bn.js';
import { shouldBehaveLikePSP22, shouldBehaveLikePSP22Approve, shouldBehaveLikePSP22Transfer } from 'wookashwackomytest-pendzl-tests';
import { getSigners, localApi } from 'wookashwackomytest-polkahat-network-helpers';
import TPsp22Deployer from 'typechain/deployers/t_psp22';
import TPsp22Contract from 'typechain/contracts/t_psp22';
import type { KeyringPair } from '@polkadot/keyring/types';
import { MAX_U128 } from 'wookashwackomytest-polkahat-chai-matchers';
import 'wookashwackomytest-polkahat-chai-matchers';
import { expect } from 'chai';
import { SignAndSendSuccessResponse } from 'wookashwackomytest-typechain-types';

const [owner, ...others] = getSigners();
const initialSupply = new BN(1000);
const TOKEN_NAME = 'SomeToken';
const SYBOL = 'ST';
const DECIMALS = 18;
async function prepareEnvBase(api: ApiPromise) {
  const deployRet = await new TPsp22Deployer(api, owner).new(initialSupply, TOKEN_NAME, SYBOL, DECIMALS);

  return { tPSP22: deployRet.contract };
}
describe('PSP 22', () => {
  const ctx: {
    token: TPsp22Contract;
    holder: KeyringPair;
    other: KeyringPair;
    recipient: KeyringPair;
    tx: SignAndSendSuccessResponse;
    initialSupply: BN;
    totalSupply: BN;
    transfer: (from: KeyringPair, to: KeyringPair, value: BN) => Promise<SignAndSendSuccessResponse>;
    approve: (holder: KeyringPair, spender: KeyringPair, value: BN) => Promise<SignAndSendSuccessResponse>;
  } = {} as any;

  beforeEach(async () => {
    const api = await localApi.get();
    const contracts = await prepareEnvBase(api);
    ctx.initialSupply = initialSupply;
    ctx.token = contracts.tPSP22;
    ctx.holder = owner;
    ctx.recipient = others[0];
    ctx.other = others[1];
  });

  shouldBehaveLikePSP22(() => ctx);

  describe('metadata', () => {
    it('has a name', async function () {
      await expect(ctx.token.query.tokenName()).to.haveOkResult(TOKEN_NAME);
    });

    it('has a symbol', async function () {
      await expect(ctx.token.query.tokenSymbol()).to.haveOkResult(SYBOL);
    });

    it('has 18 decimals', async function () {
      await expect(ctx.token.query.tokenDecimals()).to.haveOkResult(18);
    });
  });
  describe('_mint', function () {
    const value = new BN(50);

    it('rejects overflow', async function () {
      await expect(ctx.token.query.tMint(ctx.recipient.address, MAX_U128)).to.be.revertedWithError({ custom: 'M::Overflow' });
    });

    describe('for a non zero account', function () {
      beforeEach('minting', async function () {
        ctx.tx = await ctx.token.tx.tMint(ctx.recipient.address, value);
      });

      it('increments totalSupply', async function () {
        await expect(ctx.token.query.totalSupply()).to.haveOkResult(initialSupply.add(value));
      });

      it('increments recipient balance', async function () {
        await expect(ctx.tx).to.changePSP22Balances(ctx.token, [ctx.recipient.address], [value]);
      });

      it('emits Transfer event', async function () {
        await expect(ctx.tx).to.emitEvent(ctx.token, 'Transfer', { from: null, to: ctx.recipient.address, value });
      });
    });
  });

  describe('_burn', function () {
    it('rejects burning more than balance', async function () {
      await expect(ctx.token.query.tBurn(ctx.holder.address, initialSupply.addn(1))).to.be.revertedWithError({ insufficientBalance: null });
    });

    const describeBurn = function (description: string, value: BN) {
      describe(description, function () {
        beforeEach('burning', async function () {
          ctx.tx = await ctx.token.tx.tBurn(ctx.holder.address, value);
        });

        it('decrements totalSupply', async function () {
          await expect(ctx.token.query.totalSupply()).to.haveOkResult(initialSupply.sub(value));
        });

        it('decrements holder balance', async function () {
          await expect(ctx.tx).to.changePSP22Balances(ctx.token, [ctx.holder.address], [value.neg()]);
        });

        it('emits Transfer event', async function () {
          await expect(ctx.tx).to.emitEvent(ctx.token, 'Transfer', { from: ctx.holder.address, to: null, value });
        });
      });
    };

    describeBurn('for entire balance', initialSupply);
    describeBurn('for less value than balance', initialSupply.subn(1));
  });

  describe('_update', function () {
    const value = new BN(1);

    beforeEach(async function () {
      ctx.totalSupply = (await ctx.token.query.totalSupply()).value.unwrap();
    });

    it('from is none', async function () {
      const tx = await ctx.token.tx.tUpdate(null, ctx.holder.address, value);
      await expect(tx).to.emitEvent(ctx.token, 'Transfer', { from: null, to: ctx.holder.address, value });

      await expect(ctx.token.query.totalSupply()).to.haveOkResult(ctx.totalSupply.add(value));
      await expect(tx).to.changePSP22Balances(ctx.token, [ctx.holder.address], [value]);
    });

    it('to is none', async function () {
      const tx = await ctx.token.tx.tUpdate(ctx.holder.address, null, value);
      await expect(tx).to.emitEvent(ctx.token, 'Transfer', { from: ctx.holder.address, to: null, value });

      await expect(ctx.token.query.totalSupply()).to.haveOkResult(ctx.totalSupply.sub(value));
      await expect(tx).to.changePSP22Balances(ctx.token, [ctx.holder.address], [value.neg()]);
    });

    describe('from and to are the same address', function () {
      it('null address', async function () {
        const tx = await ctx.token.tx.tUpdate(null, null, value);
        await expect(tx).to.emitEvent(ctx.token, 'Transfer', { from: null, to: null, value });

        await expect(ctx.token.query.totalSupply()).to.haveOkResult(ctx.totalSupply);
      });

      describe('non null address', function () {
        it('reverts without balance', async function () {
          await expect(ctx.token.query.tUpdate(ctx.recipient.address, ctx.recipient.address, value)).to.be.revertedWithError({
            insufficientBalance: null,
          });
        });

        it('executes with balance', async function () {
          const tx = await ctx.token.tx.tUpdate(ctx.holder.address, ctx.holder.address, value);
          await expect(tx).to.changePSP22Balances(ctx.token, [ctx.holder.address], [new BN(0)]);
          await expect(tx).to.emitEvent(ctx.token, 'Transfer', { from: ctx.holder.address, to: ctx.holder.address, value });
        });
      });
    });
  });

  describe('_transfer', function () {
    beforeEach(function () {
      ctx.transfer = (from: KeyringPair, to: KeyringPair, value: BN) => ctx.token.tx.tTransfer(from.address, to.address, value);
    });

    shouldBehaveLikePSP22Transfer(() => ctx);
  });

  describe('_approve', function () {
    beforeEach(function () {
      ctx.approve = (holder: KeyringPair, spender: KeyringPair, value: BN) => ctx.token.tx.tApprove(holder.address, spender.address, value);
    });

    shouldBehaveLikePSP22Approve(() => ctx);
  });
});
