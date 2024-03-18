import { ApiPromise } from '@polkadot/api';
import { blake2AsU8a } from '@polkadot/util-crypto';
import { expect } from 'chai';
import TFlipperOwnableV0Contract from 'typechain/contracts/t_flipper_ownable_v0';
import TFlipperV0Contract from 'typechain/contracts/t_flipper_v0';
import TFlipperV1Contract from 'typechain/contracts/t_flipper_v1';
import TFlipperOwnableV0Deployer from 'typechain/deployers/t_flipper_ownable_v0';
import TFlipperOwnableV1Deployer from 'typechain/deployers/t_flipper_ownable_v1';
import TFlipperV0Deployer from 'typechain/deployers/t_flipper_v0';
import TFlipperV1Deployer from 'typechain/deployers/t_flipper_v1';
import { SetCodeHashErrorBuilder } from 'typechain/types-returns/t_flipper_v0';
import 'wookashwackomytest-polkahat-chai-matchers';
import { getSigners, localApi, time } from 'wookashwackomytest-polkahat-network-helpers';

const CODE_UPGRADER_ROLE = stringToSelectorId('CODE_UPGRADER');

export function stringToSelectorId(str: string) {
  const strBlake2AsU8a = blake2AsU8a(str);
  const selectorU8Array = strBlake2AsU8a.slice(0, 4);
  const buffer = Buffer.from(selectorU8Array);
  const res = buffer.readUInt32BE(0);
  return res.toString();
}

const [defaultAdmin, alice, upgrader, charlie, bob] = getSigners();
describe('Set Code Hash', () => {
  describe('AccessControl', () => {
    const ctx: {
      flipper: TFlipperV0Contract;
      api: ApiPromise;
    } = {} as any;

    beforeEach(async () => {
      ctx.api = await localApi.get();
      await time.setTo(10, ctx.api);
      const v0 = await new TFlipperV0Deployer(ctx.api, defaultAdmin).new(false);
      ctx.flipper = v0.contract;
    });

    describe('v0', () => {
      it('upgradeable field returns values correctly', async () => {
        await expect(ctx.flipper.query.get()).haveOkResult(false);
        const tx = ctx.flipper.tx.flip();
        await expect(tx).to.be.fulfilled;
        await expect(tx).to.emitEvent(ctx.flipper, 'Flipped', { newValue: true });
        await expect(ctx.flipper.query.get()).haveOkResult(true);
        await expect(ctx.flipper.query.getValV0()).to.haveOkResult(1337);
        await expect(ctx.flipper.query.getStructV0()).to.haveOkResult({
          a: true,
          inner: {
            x: false,
            y: 42,
          },
        });
      });

      describe('when upgrading code', () => {
        let originalHash: string;
        let newCodeHash: string;
        beforeEach(async () => {
          const flipperv1 = (await new TFlipperV1Deployer(await localApi.get(), defaultAdmin).deploy()).contract;
          const resOriginal = (await ctx.api.query.contracts.contractInfoOf(ctx.flipper.address)).toHuman() as { codeHash: string };
          originalHash = resOriginal.codeHash;
          const res = (await ctx.api.query.contracts.contractInfoOf(flipperv1.address)).toHuman() as { codeHash: string };
          newCodeHash = res.codeHash;
        });
        it('only CODE_UPGRADER should be able to upgrade', async () => {
          await expect(ctx.flipper.withSigner(alice).query.setCodeHash(newCodeHash)).to.be.revertedWithError(
            SetCodeHashErrorBuilder.PermissionError('AC::MissingRole'),
          );
          await expect(ctx.flipper.withSigner(defaultAdmin).query.setCodeHash(newCodeHash)).to.be.revertedWithError(
            SetCodeHashErrorBuilder.PermissionError('AC::MissingRole'),
          );
          await expect(ctx.flipper.withSigner(defaultAdmin).tx.grantRole(CODE_UPGRADER_ROLE, charlie.address)).to.be.fulfilled;
          await expect(ctx.flipper.withSigner(charlie).query.setCodeHash(newCodeHash)).to.haveOkResult();
          await expect(ctx.flipper.withSigner(charlie).tx.setCodeHash(newCodeHash)).to.be.fulfilled;
        });

        it('should return error when failing to upgrade to invalid code', async () => {
          await expect(ctx.flipper.withSigner(defaultAdmin).tx.grantRole(CODE_UPGRADER_ROLE, upgrader.address)).to.be.fulfilled;
          //replace last chars with 0
          const invalidCode = newCodeHash.slice(0, -4) + '0000';
          await expect(ctx.flipper.withSigner(upgrader).query.setCodeHash(invalidCode)).to.be.revertedWithError(
            SetCodeHashErrorBuilder.SetCodeHashFailed(),
          );
        });

        describe('after upgrade to v1', () => {
          let flipperAsV1: TFlipperV1Contract;

          beforeEach(async () => {
            (await ctx.flipper.withSigner(defaultAdmin).query.grantRole(CODE_UPGRADER_ROLE, upgrader.address)).value.unwrapRecursively();
            await ctx.flipper.withSigner(defaultAdmin).tx.grantRole(CODE_UPGRADER_ROLE, upgrader.address);
            (await ctx.flipper.withSigner(upgrader).query.setCodeHash(newCodeHash)).value.unwrapRecursively();
            await ctx.flipper.withSigner(upgrader).tx.setCodeHash(newCodeHash);
            flipperAsV1 = new TFlipperV1Contract(ctx.flipper.address, ctx.flipper.signer, ctx.flipper.nativeAPI);
          });
          it(`code hash was updated`, async () => {
            const res = (await ctx.api.query.contracts.contractInfoOf(ctx.flipper.address)).toHuman() as { codeHash: string };
            expect(res.codeHash).to.equal(newCodeHash);
          });

          it(`reverting back to previous hash should work`, async () => {
            (await flipperAsV1.withSigner(upgrader).query.setCodeHash(originalHash)).value.unwrapRecursively();
            await flipperAsV1.withSigner(upgrader).tx.setCodeHash(originalHash);
          });

          it('new upgradeable field returns values correctly', async () => {
            await expect(flipperAsV1.query.get()).haveOkResult(false);
            const tx = flipperAsV1.tx.flip();
            await expect(tx).to.be.fulfilled;
            await expect(tx).to.emitEvent(ctx.flipper, 'Flipped', { newValue: true });
            await expect(flipperAsV1.query.get()).haveOkResult(true);
            await expect(flipperAsV1.query.getValV0()).to.haveOkResult(1337);
            await expect(flipperAsV1.query.getStructV0()).to.haveOkResult({
              a: true,
              inner: {
                x: false,
                y: 42,
              },
              newField: {
                ab: false,
                cd: 0,
              },
            });
          });

          it('return value fn no longer returns hardcoded value', async () => {
            await expect(flipperAsV1.query.returnValue()).to.haveOkResult(1337);
            await expect(flipperAsV1.tx.setValue(50)).to.eventually.be.fulfilled;
            await expect(flipperAsV1.query.returnValue()).to.haveOkResult(50);
          });

          it('new setStructV0Inner method works', async () => {
            await expect(flipperAsV1.tx.setStructV0Inner(true, 100)).to.eventually.be.fulfilled;
            await expect(flipperAsV1.query.getStructV0()).to.haveOkResult({
              a: true,
              inner: {
                x: true,
                y: 100,
              },
              newField: {
                ab: false,
                cd: 0,
              },
            });
          });

          it('new setStructV0NewField method works (new field got added properly)', async () => {
            await expect(flipperAsV1.tx.setStructV0NewField(true, 5678)).to.eventually.be.fulfilled;
            await expect(flipperAsV1.query.getStructV0()).to.haveOkResult({
              a: true,
              inner: {
                x: false,
                y: 42,
              },
              newField: {
                ab: true,
                cd: 5678,
              },
            });
          });
        });
      });
    });
  });
  describe('Ownable', () => {
    const ctx: {
      flipper: TFlipperOwnableV0Contract;
      api: ApiPromise;
    } = {} as any;
    const owner = bob;

    beforeEach(async () => {
      ctx.api = await localApi.get();
      await time.setTo(10, ctx.api);
      const v0 = await new TFlipperOwnableV0Deployer(ctx.api, owner).new(false);
      ctx.flipper = v0.contract;
    });

    describe('v0', () => {
      it('upgradeable field returns values correctly', async () => {
        await expect(ctx.flipper.query.get()).haveOkResult(false);
        const tx = ctx.flipper.tx.flip();
        await expect(tx).to.be.fulfilled;
        await expect(tx).to.emitEvent(ctx.flipper, 'Flipped', { newValue: true });
        await expect(ctx.flipper.query.get()).haveOkResult(true);
        await expect(ctx.flipper.query.getValV0()).to.haveOkResult(1337);
        await expect(ctx.flipper.query.getStructV0()).to.haveOkResult({
          a: true,
          inner: {
            x: false,
            y: 42,
          },
        });
      });

      describe('when upgrading code', () => {
        let originalHash: string;
        let newCodeHash: string;
        beforeEach(async () => {
          const flipperv1 = (await new TFlipperOwnableV1Deployer(await localApi.get(), defaultAdmin).deploy()).contract;
          const resOriginal = (await ctx.api.query.contracts.contractInfoOf(ctx.flipper.address)).toHuman() as { codeHash: string };
          originalHash = resOriginal.codeHash;
          const res = (await ctx.api.query.contracts.contractInfoOf(flipperv1.address)).toHuman() as { codeHash: string };
          newCodeHash = res.codeHash;
        });
        it('only owner should be able to upgrade', async () => {
          await expect(ctx.flipper.withSigner(alice).query.setCodeHash(newCodeHash)).to.be.revertedWithError(
            SetCodeHashErrorBuilder.PermissionError('O::CallerIsNotOwner'),
          );
          await expect(ctx.flipper.withSigner(defaultAdmin).query.setCodeHash(newCodeHash)).to.be.revertedWithError(
            SetCodeHashErrorBuilder.PermissionError('O::CallerIsNotOwner'),
          );
          await expect(ctx.flipper.withSigner(owner).query.setCodeHash(newCodeHash)).to.haveOkResult();
          await expect(ctx.flipper.withSigner(owner).tx.setCodeHash(newCodeHash)).to.be.fulfilled;
        });

        it('should return error when failing to upgrade to invalid code', async () => {
          //replace last chars with 0
          const invalidCode = newCodeHash.slice(0, -4) + '0000';
          await expect(ctx.flipper.withSigner(owner).query.setCodeHash(invalidCode)).to.be.revertedWithError(
            SetCodeHashErrorBuilder.SetCodeHashFailed(),
          );
        });

        describe('after upgrade to v1', () => {
          let flipperAsV1: TFlipperV1Contract;

          beforeEach(async () => {
            (await ctx.flipper.withSigner(owner).query.setCodeHash(newCodeHash)).value.unwrapRecursively();
            await ctx.flipper.withSigner(owner).tx.setCodeHash(newCodeHash);
            flipperAsV1 = new TFlipperV1Contract(ctx.flipper.address, ctx.flipper.signer, ctx.flipper.nativeAPI);
          });
          it(`code hash was updated`, async () => {
            const res = (await ctx.api.query.contracts.contractInfoOf(ctx.flipper.address)).toHuman() as { codeHash: string };
            expect(res.codeHash).to.equal(newCodeHash);
          });

          it(`reverting back to previous hash should work`, async () => {
            (await flipperAsV1.withSigner(owner).query.setCodeHash(originalHash)).value.unwrapRecursively();
            await flipperAsV1.withSigner(owner).tx.setCodeHash(originalHash);
          });

          it('new upgradeable field returns values correctly', async () => {
            await expect(flipperAsV1.query.get()).haveOkResult(false);
            const tx = flipperAsV1.tx.flip();
            await expect(tx).to.be.fulfilled;
            await expect(tx).to.emitEvent(ctx.flipper, 'Flipped', { newValue: true });
            await expect(flipperAsV1.query.get()).haveOkResult(true);
            await expect(flipperAsV1.query.getValV0()).to.haveOkResult(1337);
            await expect(flipperAsV1.query.getStructV0()).to.haveOkResult({
              a: true,
              inner: {
                x: false,
                y: 42,
              },
              newField: {
                ab: false,
                cd: 0,
              },
            });
          });

          it('return value fn no longer returns hardcoded value', async () => {
            await expect(flipperAsV1.query.returnValue()).to.haveOkResult(1337);
            await expect(flipperAsV1.tx.setValue(50)).to.eventually.be.fulfilled;
            await expect(flipperAsV1.query.returnValue()).to.haveOkResult(50);
          });

          it('new setStructV0Inner method works', async () => {
            await expect(flipperAsV1.tx.setStructV0Inner(true, 100)).to.eventually.be.fulfilled;
            await expect(flipperAsV1.query.getStructV0()).to.haveOkResult({
              a: true,
              inner: {
                x: true,
                y: 100,
              },
              newField: {
                ab: false,
                cd: 0,
              },
            });
          });

          it('new setStructV0NewField method works (new field got added properly)', async () => {
            await expect(flipperAsV1.tx.setStructV0NewField(true, 5678)).to.eventually.be.fulfilled;
            await expect(flipperAsV1.query.getStructV0()).to.haveOkResult({
              a: true,
              inner: {
                x: false,
                y: 42,
              },
              newField: {
                ab: true,
                cd: 5678,
              },
            });
          });
        });
      });
    });
  });
});
