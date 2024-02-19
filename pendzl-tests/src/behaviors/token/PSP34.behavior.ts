import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import "wookashwackomytest-polkahat-chai-matchers";
import { SignAndSendSuccessResponse } from "wookashwackomytest-typechain-types";
import { BN } from "bn.js";
import { Id, PSP34 } from "../../types/PSP34.type";
import { getSigners } from "wookashwackomytest-polkahat-network-helpers";

export const firstTokenId: Id = { u128: new BN(79216) };
export const secondTokenId = { u64: new BN(19235217) };
export const thirdTokenId = { u32: new BN(51345) };

export const nonExistentTokenId = { u8: new BN(13) };

export type ShouldBehaveLikePSP34Params = {
  token: PSP34;
  owner: KeyringPair;
};

export function shouldBehaveLikePSP34(
  getParams: () => ShouldBehaveLikePSP34Params
) {
  const ctx: ShouldBehaveLikePSP34Params & {
    tx: any;
    newOwner: KeyringPair;
    approved: KeyringPair;
    operator: KeyringPair;
    to: KeyringPair;
    other: KeyringPair;
  } = {} as any;

  describe("with minted tokens", function () {
    beforeEach(async function () {
      Object.assign(ctx, getParams());
      [ctx.newOwner, ctx.approved, ctx.operator, ctx.to, ctx.other] =
        getSigners().filter((signer) => signer.address !== ctx.owner.address);
      await expect(
        ctx.token.query.ownerOf(firstTokenId),
        `to use shouldBehaveLikePSP34 test you should set ownership of ${firstTokenId} to owner`
      ).to.haveOkResult(ctx.owner.address);
      await expect(
        ctx.token.query.ownerOf(secondTokenId),
        `to use shouldBehaveLikePSP34 test you should set ownership of ${secondTokenId} to owner`
      ).to.haveOkResult(ctx.owner.address);

      await expect(
        ctx.token.query.ownerOf(nonExistentTokenId),
        `to use shouldBehaveLikePSP34 test  ${nonExistentTokenId} should not exist`
      ).to.haveOkResult(null);

      await expect(
        ctx.token.query.totalSupply(),
        `to use shouldBehaveLikePSP34 test only ${firstTokenId} and ${secondTokenId} should exist. The total_supply should be 2.`
      ).to.haveOkResult(2);
    });

    describe("balanceOf", function () {
      describe(`when the given address owns ${firstTokenId} and ${secondTokenId} tokens`, function () {
        it("returns the amount of tokens owned by the given address", async function () {
          await expect(
            ctx.token.query.balanceOf(ctx.owner.address)
          ).to.haveOkResult(2);
        });
      });

      describe("when the given address does not own any tokens", function () {
        it("returns 0", async function () {
          await expect(
            ctx.token.query.balanceOf(ctx.other.address)
          ).to.haveOkResult(0);
        });
      });
    });
    describe("ownerOf", function () {
      describe("when the given token ID was tracked by this token", function () {
        const tokenId = firstTokenId;

        it("returns the owner of the given token ID", async function () {
          await expect(ctx.token.query.ownerOf(tokenId)).to.haveOkResult(
            ctx.owner.address
          );
        });
      });

      describe("when the given token ID was not tracked by this token", function () {
        const tokenId = nonExistentTokenId;

        it("returns null", async function () {
          await expect(ctx.token.query.ownerOf(tokenId)).to.haveOkResult(null);
        });
      });
    });

    describe("transfer", function () {
      const tokenId = firstTokenId;
      let tx: Promise<SignAndSendSuccessResponse>;

      beforeEach(async function () {
        await ctx.token
          .withSigner(ctx.owner)
          .tx.approve(ctx.approved.address, tokenId, true);
        await ctx.token
          .withSigner(ctx.owner)
          .tx.approve(ctx.operator.address, null, true);
      });
      shouldTransferTokenByUser(() => ({
        token: ctx.token,
        fnName: "transfer",
        owner: ctx.owner,
        approved: ctx.approved,
        operator: ctx.operator,
        other: ctx.other,
        to: ctx.to,
        tokenId: tokenId,
        nonExistentTokenId: nonExistentTokenId,
      }));
    });

    describe("approve", function () {
      const tokenId = firstTokenId;

      describe("when clearing approval", function () {
        describe("when there was no prior approval", function () {
          beforeEach(async function () {
            ctx.tx = ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, false);
          });

          it("is stays unapproved", async function () {
            await ctx.tx;
            await expect(
              ctx.token.query.allowance(
                ctx.owner.address,
                ctx.approved!.address,
                tokenId
              )
            ).to.haveOkResult(false);
          });
          it("emits no Approval event", async function () {
            const tx = await ctx.tx;
            await expect(ctx.tx).to.emitEvent(ctx.token, "Approval");
          });
        });

        describe("when there was a prior approval", function () {
          beforeEach(async function () {
            await ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, true);
            ctx.tx = ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, false);
          });

          it("is clears approval", async function () {
            await ctx.tx;
            await expect(
              ctx.token.query.allowance(
                ctx.owner.address,
                ctx.approved.address,
                tokenId
              )
            ).to.haveOkResult(false);
          });
          it("emits an Approval event", async function () {
            await expect(ctx.tx).to.emitEvent(ctx.token, "Approval", {
              owner: ctx.owner.address,
              operator: ctx.approved!.address,
              id: tokenId as any,
              approved: false,
            });
          });
        });
      });

      describe("when approving anaccount", function () {
        describe("when there was a prior approval to the same address", function () {
          beforeEach(async function () {
            await ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, true);
            ctx.tx = ctx.token
              .withSigner(ctx.owner)
              .tx.approve(ctx.approved.address, tokenId, true);
          });

          it("is stays approved", async function () {
            await ctx.tx;
            await expect(
              ctx.token.query.allowance(
                ctx.owner.address,
                ctx.approved.address,
                tokenId
              )
            ).to.haveOkResult(true);
          });
          it("emits an Approval event", async function () {
            await expect(ctx.tx).to.emitEvent(ctx.token, "Approval", {
              owner: ctx.owner.address,
              operator: ctx.approved!.address,
              id: tokenId as any,
              approved: true,
            });
          });
        });
      });

      describe("when the sender does not own the given token ID", function () {
        it("reverts", async function () {
          await expect(
            ctx.token
              .withSigner(ctx.other)
              .query.approve(ctx.approved.address, tokenId, true)
          ).to.be.revertedWithError({ notApproved: null });
        });
      });

      describe("when the sender is approved for the given token ID", function () {
        beforeEach(async function () {
          await ctx.token
            .withSigner(ctx.owner)
            .query.approve(ctx.approved.address, tokenId, true);
        });
        it("reverts", async function () {
          await expect(
            ctx.token
              .withSigner(ctx.approved)
              .query.approve(ctx.other.address, tokenId, true)
          ).to.be.revertedWithError({ notApproved: null });
        });
      });

      describe("when the sender is an operator", function () {
        beforeEach(async function () {
          await ctx.token
            .withSigner(ctx.owner)
            .tx.approve(ctx.operator.address, null, true);
        });

        it("reverts", async function () {
          await expect(
            ctx.token
              .withSigner(ctx.operator)
              .query.approve(ctx.approved.address, tokenId, true)
          ).to.be.revertedWithError({ notApproved: null });
        });
      });
    });
  });
}

export type shouldTransferTokenByUserParams = {
  fnName: string;
  token: any;
  owner: KeyringPair;
  approved: KeyringPair;
  operator: KeyringPair;
  to: KeyringPair;
  other: KeyringPair;
  tokenId: Id;
  nonExistentTokenId: Id;
};

export function shouldTransferTokenByUser(
  getCtx: () => shouldTransferTokenByUserParams
) {
  describe("when called by the owner", function () {
    let ctx: shouldTransferTokenByUserParams;
    let tx: Promise<SignAndSendSuccessResponse>;
    beforeEach(function () {
      ctx = getCtx();
      tx = ctx.token
        .withSigner(ctx.owner)
        .tx[ctx.fnName](ctx.to.address, ctx.tokenId, []);
    });
    transferWasSuccessful(() => ({
      tx: tx,
      token: ctx.token,
      from: ctx.owner.address,
      to: ctx.to.address,
      tokenId: ctx.tokenId,
    }));
  });

  describe("when called by the approved individual", function () {
    let ctx: shouldTransferTokenByUserParams;
    let tx: Promise<SignAndSendSuccessResponse>;
    beforeEach(function () {
      ctx = getCtx();
      tx = ctx.token
        .withSigner(ctx.approved)
        .tx[ctx.fnName](ctx.to.address, ctx.tokenId, []);
    });
    transferWasSuccessful(() => ({
      tx: tx,
      from: ctx.owner.address,
      to: ctx.to.address,
      tokenId: ctx.tokenId,
      token: ctx.token,
    }));
  });

  describe("when called by the operator", function () {
    let ctx: shouldTransferTokenByUserParams;
    let tx: Promise<SignAndSendSuccessResponse>;
    beforeEach(function () {
      ctx = getCtx();
      tx = ctx.token
        .withSigner(ctx.operator)
        .tx[ctx.fnName](ctx.to.address, ctx.tokenId, []);
    });
    transferWasSuccessful(() => ({
      tx: tx,
      from: ctx.owner.address,
      to: ctx.to.address,
      tokenId: ctx.tokenId,
      token: ctx.token,
    }));
  });

  describe("when sent to the owner", function () {
    let ctx: shouldTransferTokenByUserParams;
    let tx: Promise<SignAndSendSuccessResponse>;
    beforeEach(function () {
      ctx = getCtx();
      tx = ctx.token
        .withSigner(ctx.owner)
        .tx[ctx.fnName](ctx.owner.address, ctx.tokenId, []);
    });

    it("keeps ownership of the token", async function () {
      await tx;
      expect(await ctx.token.query.ownerOf(ctx.tokenId)).to.equal(
        ctx.owner.address
      );
    });

    it("emits only a transfer event", async function () {
      await expect(tx).to.emitEvent(ctx.token, "Transfer", {
        from: ctx.owner.address,
        to: ctx.owner.address,
        id: ctx.tokenId as any,
      });
    });

    it("keeps the owner balance", async function () {
      expect(tx).to.changePSP34Balances(
        ctx.token,
        [ctx.owner.address],
        [new BN(0)]
      );
    });
  });

  describe("when the sender is not authorized for the token id", function () {
    let ctx: shouldTransferTokenByUserParams;
    let query;
    beforeEach(function () {
      ctx = getCtx();
      query = ctx.token
        .withSigner(ctx.to)
        .query[ctx.fnName](ctx.to.address, ctx.tokenId, []);
    });
    it("reverts", async function () {
      await expect(
        ctx.token
          .withSigner(ctx.other)
          .query.transfer(ctx.other.address, ctx.tokenId, [])
      ).to.be.revertedWithError({ notApproved: null });
    });
  });

  describe("when the given token ID does not exist", function () {
    let ctx: shouldTransferTokenByUserParams;
    let query: any;
    beforeEach(function () {
      ctx = getCtx();
      query = ctx.token
        .withSigner(ctx.to)
        .query[ctx.fnName](ctx.to.address, nonExistentTokenId, []);
    });
    it("reverts", async function () {
      await expect(query).to.be.revertedWithError({ tokenNotExists: null });
    });
  });
}

export type transferWasSuccesfulParams = {
  tx: Promise<SignAndSendSuccessResponse>;
  token: any;
  from: string;
  to: string;
  tokenId: Id;
};

export function transferWasSuccessful(ctx: () => transferWasSuccesfulParams) {
  it("transfers the ownership of the given token ID to the given address", async function () {
    await ctx().tx;
    await expect(ctx().token.query.ownerOf(ctx().tokenId)).to.haveOkResult(
      ctx().to
    );
  });

  it("emits a Transfer event", async function () {
    await expect(ctx().tx).to.emitEvent(ctx().token, "Transfer", {
      from: ctx().from,
      to: ctx().to,
      id: ctx().tokenId as any,
    });
  });

  it("clears the approval for the token ID with no event", async function () {
    await expect(ctx().tx).not.to.emitEvent(ctx().token, "Approval");

    await expect(
      ctx().token.query.allowance(ctx().from, ctx().to, ctx().tokenId)
    ).to.haveOkResult(false);
  });

  it("adjusts owners and receiver balances", async function () {
    if (ctx().from === ctx().to) {
      await expect(ctx().tx).to.changePSP34Balances(
        ctx().token,
        [ctx().from],
        [new BN(0)]
      );
    } else {
      await expect(ctx().tx).to.changePSP34Balances(
        ctx().token,
        [ctx().from, ctx().to],
        [new BN(-1), new BN(1)]
      );
    }
  });
}
