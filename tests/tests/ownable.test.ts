import { ApiPromise } from '@polkadot/api';
import { expect } from 'chai';
import { localApi } from '@c-forge/polkahat-network-helpers';
import TOwnableContract from 'typechain/contracts/t_ownable';
import TOwnableDeployer from 'typechain/deployers/t_ownable';
import { shouldBehaveLikeOwnable } from '@c-forge/pendzl-tests';
import { getSigners } from '@c-forge/polkahat-network-helpers';
import '@c-forge/polkahat-chai-matchers';

const [deployer, owner, ...others] = getSigners();
describe('Ownable', () => {
  let tOwnable: TOwnableContract;
  let api: ApiPromise;
  beforeEach(async () => {
    api = await localApi.get();
  });

  shouldBehaveLikeOwnable(() => ({ ownableDeployerCall: () => new TOwnableDeployer(api, deployer).new(owner.address), owner, other: others[0] }));

  describe('OwnableInteral', function () {
    beforeEach(async function () {
      tOwnable = (await new TOwnableDeployer(api, owner).new(owner.address)).contract;
    });

    describe('_update_owner', function () {
      it('emits event and updates owner', async () => {
        await tOwnable.withSigner(owner).tx.renounceOwnership();
        expect((await tOwnable.query.owner()).value.ok).to.equal(null);

        await expect(tOwnable.tx.tUpdateOwner(owner.address)).to.emitEvent(tOwnable, 'OwnershipTransferred', {
          new: owner.address,
        });
        expect((await tOwnable.query.owner()).value.ok).to.equal(owner.address);
      });
    });

    describe('_only_owner', function () {
      it('reverts if not owner', async () => {
        await expect(tOwnable.withSigner(others[0]).query.tOnlyOwner()).to.be.revertedWithError('CallerIsNotOwner');
      });

      it('does pass if owner', async () => {
        await expect(tOwnable.withSigner(owner).tx.tOnlyOwner()).to.be.eventually.fulfilled;
      });
    });
  });
});
