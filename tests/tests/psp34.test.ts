import { ApiPromise } from '@polkadot/api';
import { BN } from 'bn.js';
import { expect } from 'chai';
import TPSP34MetadataContract from 'typechain/contracts/t_psp34_metadata';
import TPSP34MetadataDeployer from 'typechain/deployers/t_psp34_metadata';
import { nonExistentTokenId } from 'wookashwackomytest-pendzl-tests';
import {
  firstTokenId,
  secondTokenId,
  shouldBehaveLikePSP34,
  testPSP34TransferCorrectness,
} from 'wookashwackomytest-pendzl-tests/src/behaviors/token/PSP34.behavior';
import 'wookashwackomytest-polkahat-chai-matchers';
import { getSigners, localApi } from 'wookashwackomytest-polkahat-network-helpers';
import { SignAndSendSuccessResponse } from 'wookashwackomytest-typechain-types';

const [deployer, owner, approved, operator, to] = getSigners();
describe('PSP34', () => {
  let tPSP34: TPSP34MetadataContract;
  let api: ApiPromise;
  beforeEach(async () => {
    api = await localApi.get();
    tPSP34 = (await new TPSP34MetadataDeployer(api, deployer).new('', '')).contract;
  });

  describe('shouldBehaveLikePSP34', function () {
    beforeEach(async () => {
      await tPSP34.withSigner(owner).tx.tMint(owner.address, firstTokenId);
      await tPSP34.withSigner(owner).tx.tMint(owner.address, secondTokenId);
    });

    shouldBehaveLikePSP34(() => ({
      token: tPSP34 as any,
      owner,
    }));
  });

  describe('OwnableInteral', function () {
    describe('_mint', function () {
      describe('with minted token', async function () {
        let tx: SignAndSendSuccessResponse;
        beforeEach(async function () {
          const qq = await tPSP34.query.tMint(owner.address, firstTokenId);

          tx = await tPSP34.tx.tMint(owner.address, firstTokenId);
        });

        it('emits a Transfer event', async function () {
          await expect(tx).to.emitEvent(tPSP34, 'Transfer', { from: null, to: owner.address, id: firstTokenId });
        });

        it('creates the token', async function () {
          await expect(tPSP34.query.balanceOf(owner.address)).to.haveOkResult(1);
          await expect(tPSP34.query.ownerOf(firstTokenId)).to.haveOkResult(owner.address);
        });

        it('reverts when adding a token id that already exists', async function () {
          await expect(tPSP34.query.tMint(owner.address, firstTokenId)).to.be.revertedWithError({ tokenExists: null });
        });
      });
    });

    describe('_burn', function () {
      it('reverts when burning a non-existent token id', async function () {
        await expect(tPSP34.query.tBurn(owner.address, firstTokenId)).to.be.revertedWithError({ tokenNotExists: null });
      });

      describe('with minted tokens', function () {
        beforeEach(async function () {
          await tPSP34.tx.tMint(owner.address, firstTokenId);
          await tPSP34.tx.tMint(owner.address, secondTokenId);
        });

        describe('with burnt token', function () {
          let tx: SignAndSendSuccessResponse;
          beforeEach(async function () {
            tx = await tPSP34.tx.tBurn(owner.address, firstTokenId);
          });

          it('emits a Transfer event', async function () {
            await expect(tx).to.emitEvent(tPSP34, 'Transfer', { from: owner.address, to: null, id: firstTokenId });
          });

          it('deletes the token', async function () {
            await expect(tPSP34.query.balanceOf(owner.address)).to.haveOkResult(1);
            await expect(tPSP34.query.ownerOf(firstTokenId)).to.be.haveOkResult(null);
          });

          it('reverts when burning a token id that has been deleted', async function () {
            await expect(tPSP34.query.tBurn(owner.address, firstTokenId)).to.be.revertedWithError({ tokenNotExists: null });
          });
        });
      });
    });
    describe('_transfer', function () {
      const tokenId = firstTokenId;
      let tx: Promise<SignAndSendSuccessResponse>;
      beforeEach(async function () {
        await tPSP34.withSigner(owner).tx.tMint(owner.address, tokenId);
      });
      describe('when called by the owner', function () {
        beforeEach(function () {
          tx = tPSP34.withSigner(owner).tx.tTransfer(owner.address, to.address, tokenId, []);
        });
        testPSP34TransferCorrectness(() => ({
          tx: tx,
          token: tPSP34,
          from: owner.address,
          to: to.address,
          tokenId: tokenId,
        }));
      });

      describe('when called by the approved individual', function () {
        beforeEach(function () {
          tx = tPSP34.withSigner(approved).tx.tTransfer(owner.address, to.address, tokenId, []);
        });
        testPSP34TransferCorrectness(() => ({
          tx: tx,
          token: tPSP34,
          from: owner.address,
          to: to.address,
          tokenId: tokenId,
        }));
      });

      describe('when called by the operator', function () {
        beforeEach(function () {
          tx = tPSP34.withSigner(operator).tx.tTransfer(owner.address, to.address, tokenId, []);
        });
        testPSP34TransferCorrectness(() => ({
          tx: tx,
          token: tPSP34,
          from: owner.address,
          to: to.address,
          tokenId: tokenId,
        }));
      });

      describe('when sent to the owner', function () {
        beforeEach(function () {
          tx = tPSP34.withSigner(owner).tx.tTransfer(owner.address, owner.address, tokenId, []);
        });

        it('keeps ownership of the tPSP34', async function () {
          await tx;
          await expect(tPSP34.query.ownerOf(tokenId)).to.haveOkResult(owner.address);
        });

        it('emits only a transfer event', async function () {
          await expect(tx).to.emitEvent(tPSP34, 'Transfer', {
            from: owner.address,
            to: owner.address,
            id: tokenId as any,
          });
        });

        it('keeps the owner balance', async function () {
          expect(tx).to.changePSP34Balances(tPSP34, [owner.address], [new BN(0)]);
        });
      });

      // describe('when the sender is not authorized for the token id', function () {
      //   let query;
      //   beforeEach(function () {
      //     query = tPSP34.withSigner(to).query.tTransfer(owner.address, to.address, tokenId, []);
      //   });
      //   it('reverts', async function () {
      //     await expect(query).to.be.revertedWithError({
      //       notApproved: null,
      //     });
      //   });
      // });

      describe('when the given token ID does not exist', function () {
        let query: any;
        beforeEach(function () {
          query = tPSP34.withSigner(owner).query.tTransfer(owner.address, to.address, nonExistentTokenId, []);
        });
        it('reverts', async function () {
          await expect(query).to.be.revertedWithError({ tokenNotExists: null });
        });
      });
    });
  });
});

describe('PSP34Metadata', function () {
  describe('with shouldBehaveLikeERC721Metadata tokens', function () {
    let tPSP34: TPSP34MetadataContract;
    let api: ApiPromise;
    const tokenName = 'TokenName';
    const tokenSymbol = 'TokenSymbol';
    beforeEach(async () => {
      api = await localApi.get();
      tPSP34 = (await new TPSP34MetadataDeployer(api, deployer).new(tokenName, tokenSymbol)).contract;
    });

    describe('name and symbol atributes', function () {
      it('has a name', async function () {
        await expect(tPSP34.query.getAttribute({ u8: new BN(0) }, 'name')).to.haveOkResult(tokenName);
      });

      it('has a symbol', async function () {
        await expect(tPSP34.query.getAttribute({ u8: new BN(0) }, 'symbol')).to.haveOkResult(tokenSymbol);
      });
    });

    describe('token atribute', function () {
      it('return none by default', async function () {
        await expect(tPSP34.query.getAttribute(firstTokenId, 'name')).to.haveOkResult(null);
      });

      describe('_set atribute', function () {
        const KEY = 'key';
        const ATRIBUTE = 'ATRIBUTE';
        const ATRIBUTE2 = 'ATRIBUTE2';
        it('atribute can be set', async function () {
          await tPSP34.tx.tSetAtribute(firstTokenId, KEY, ATRIBUTE);
          await expect(tPSP34.query.getAttribute(firstTokenId, KEY)).to.haveOkResult(ATRIBUTE);
        });

        it('it can be overriten', async function () {
          await tPSP34.tx.tSetAtribute(firstTokenId, KEY, ATRIBUTE);
          await tPSP34.tx.tSetAtribute(firstTokenId, KEY, ATRIBUTE2);
          await expect(tPSP34.query.getAttribute(firstTokenId, KEY)).to.haveOkResult(ATRIBUTE2);
        });
      });
    });
  });
});
