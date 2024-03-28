import { ApiPromise } from '@polkadot/api';
import '@c-forge/polkahat-chai-matchers';
import type { SignAndSendSuccessResponse } from '@c-forge/typechain-types';
import { getSigners, localApi } from '@c-forge/polkahat-network-helpers';
import TPausableDeployer from 'typechain/deployers/t_pausable';
import TPausableContract from 'typechain/contracts/t_pausable';
import { expect } from 'chai';

const [deployer, other, ...others] = getSigners();

describe('Pausable', function () {
  let tPausable: TPausableContract;
  let api: ApiPromise;
  beforeEach(async function () {
    api = await localApi.get();
    tPausable = (await new TPausableDeployer(api, deployer).new()).contract;
  });

  describe('when unpaused', function () {
    beforeEach(async function () {
      expect((await tPausable.query.paused()).value.ok).to.be.false;
    });

    it('can perform normal process ', async function () {
      expect((await tPausable.query.count()).value.ok?.toString()).to.equal('0');

      await tPausable.tx.normalProcess();
      expect((await tPausable.query.count()).value.ok?.toString()).to.equal('1');
    });

    it('cannot take drastic measure', async function () {
      await expect(tPausable.query.drasticMeasure()).to.be.revertedWithError('NotPaused');
    });

    describe('when paused', function () {
      let tx: SignAndSendSuccessResponse;
      beforeEach(async function () {
        tx = await tPausable.withSigner(other).tx.pause();
      });

      it('emits a Paused event', async function () {
        await expect(tx).to.emitEvent(tPausable, 'Paused', { account: other.address });
      });

      it('is paused', async function () {
        expect((await tPausable.query.paused()).value.ok).to.be.true;
      });

      it('cannot perform normal process in pause', async function () {
        await expect(tPausable.query.normalProcess()).to.be.revertedWithError('Paused');
      });

      it('can take a drastic measure in a pause', async function () {
        await expect(tPausable.tx.drasticMeasure()).to.be.eventually.fulfilled;
        expect((await tPausable.query.drasticMeasureTaken()).value.ok).to.be.true;
      });

      it('reverts when re-pausing', async function () {
        await expect(tPausable.query.pause()).to.be.revertedWithError('Paused');
      });

      describe('unpausing', function () {
        it('is unpausable', async function () {
          await expect(tPausable.tx.unpause()).to.be.eventually.fulfilled;
          expect((await tPausable.query.paused()).value.ok).to.be.false;
        });

        describe('when unpaused', function () {
          beforeEach(async function () {
            tx = await tPausable.withSigner(other).tx.unpause();
          });

          it('emits an Unpaused event', async function () {
            await expect(tx).to.emitEvent(tPausable, 'Unpaused', { account: other.address });
          });

          it('should resume allowing normal process', async function () {
            expect((await tPausable.query.count()).value.ok?.toString()).to.equal('0');
            await expect(tPausable.tx.normalProcess()).to.be.eventually.fulfilled;
            expect((await tPausable.query.count()).value.ok?.toString()).to.equal('1');
          });

          it('should prevent drastic measure', async function () {
            await expect(tPausable.query.drasticMeasure()).to.be.revertedWithError('NotPaused');
          });

          it('reverts when re-unpausing', async function () {
            await expect(tPausable.query.unpause()).to.be.revertedWithError('NotPaused');
          });
        });
      });
    });
  });
});
