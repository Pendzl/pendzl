import { SubmittableExtrinsic } from "@polkadot/api/types";
import { KeyringPair } from "@polkadot/keyring/types";
import BN from "bn.js";
import { expect } from "chai";
import { SignAndSendSuccessResponse } from "@c-forge/typechain-types";
import { PSP22 } from "../../types/PSP22.type";
import "@c-forge/polkahat-chai-matchers";

type shouldBehaveLikePSP22Params = {
  token: PSP22;
  initialSupply: BN;
  holder: KeyringPair;
  recipient: KeyringPair;
  other: KeyringPair;
};
export function shouldBehaveLikePSP22(
  getParams: () => shouldBehaveLikePSP22Params
) {
  const ctx: shouldBehaveLikePSP22Params & {
    tx: any;
    transfer: (
      from: KeyringPair,
      to: KeyringPair,
      value: BN
    ) => Promise<SignAndSendSuccessResponse>;
    approve: (
      from: KeyringPair,
      to: KeyringPair,
      amount: BN
    ) => Promise<SignAndSendSuccessResponse>;
  } = {} as any;
  beforeEach(async function () {
    Object.assign(ctx, getParams());
    ctx.transfer = (from: KeyringPair, to: KeyringPair, value: BN) =>
      ctx.token.withSigner(from).tx.transfer(to.address, value, []);
    ctx.approve = (from: KeyringPair, to: KeyringPair, amount: BN) =>
      ctx.token.withSigner(from).tx.approve(to.address, amount);
  });

  it("total supply: returns the total token value", async function () {
    await expect(ctx.token.query.totalSupply()).to.haveOkResult(
      ctx.initialSupply
    );
  });

  describe("balanceOf", function () {
    it("returns zero when the requested account has no tokens", async function () {
      expect(
        (
          await ctx.token.query.balanceOf(ctx.other.address)
        ).value.ok?.toString()
      ).to.equal("0");
    });

    it("returns the total token value when the requested account has some tokens", async function () {
      expect(
        (
          await ctx.token.query.balanceOf(ctx.holder.address)
        ).value.ok?.toString()
      ).to.equal(ctx.initialSupply.toString());
    });
  });

  describe("transfer", function () {
    shouldBehaveLikePSP22Transfer(() => ctx);
  });

  describe("transfer from", function () {
    describe("when the token owner is not the zero address", function () {
      describe("when the recipient is not the zero address", function () {
        describe("when the spender has enough allowance", function () {
          beforeEach(async function () {
            await ctx.token
              .withSigner(ctx.holder)
              .tx.approve(ctx.recipient.address, ctx.initialSupply);
          });

          describe("when the token owner has enough balance", function () {
            let value: BN;
            beforeEach(async function () {
              value = ctx.initialSupply;
              ctx.tx = await ctx.token
                .withSigner(ctx.recipient)
                .tx.transferFrom(
                  ctx.holder.address,
                  ctx.other.address,
                  value,
                  []
                );
            });

            it("transfers the requested value", async function () {
              await expect(ctx.tx).to.changePSP22Balances(
                ctx.token,
                [ctx.holder.address, ctx.other.address],
                [value.neg(), value]
              );
            });
            it("decreases the spender allowance", async function () {
              await expect(ctx.tx).to.changePSP22Allowances(
                ctx.token,
                [[ctx.holder.address, ctx.recipient.address]],
                [value.neg()]
              );
            });

            it("emits a transfer event", async function () {
              await expect(ctx.tx).to.emitEvent(ctx.token, "Transfer", {
                from: ctx.holder.address,
                to: ctx.other.address,
                value: value,
              });
            });

            it("emits an approval event", async function () {
              const allowance = (
                await ctx.token.query.allowance(
                  ctx.holder.address,
                  ctx.recipient.address
                )
              ).value.unwrap();
              await expect(ctx.tx).to.emitEvent(ctx.token, "Approval", {
                owner: ctx.holder.address,
                spender: ctx.recipient.address,
                value: allowance.toString(),
              });
            });
          });

          it("reverts when the token owner does not have enough balance", async function () {
            const value = ctx.initialSupply;
            await ctx.token
              .withSigner(ctx.holder)
              .tx.transfer(ctx.other.address, 1, []);
            await expect(
              ctx.token
                .withSigner(ctx.recipient)
                .query.transferFrom(
                  ctx.holder.address,
                  ctx.other.address,
                  value,
                  []
                )
            ).to.be.revertedWithError({
              insufficientBalance: null,
            });
          });
        });

        describe("when the spender does not have enough allowance", function () {
          let allowance: BN;

          beforeEach(async function () {
            allowance = ctx.initialSupply.subn(1);
            await ctx.token
              .withSigner(ctx.holder)
              .tx.approve(ctx.recipient.address, allowance);
          });

          it("reverts when the token owner has enough balance", async function () {
            const value = ctx.initialSupply;
            await expect(
              ctx.token
                .withSigner(ctx.recipient)
                .query.transferFrom(
                  ctx.holder.address,
                  ctx.other.address,
                  value,
                  []
                )
            ).to.be.revertedWithError({
              insufficientAllowance: null,
            });
          });

          it("reverts when the token owner does not have enough balance", async function () {
            const value = allowance;
            await ctx.token
              .withSigner(ctx.holder)
              .tx.transfer(ctx.other.address, 2, []);
            await expect(
              ctx.token
                .withSigner(ctx.recipient)
                .query.transferFrom(
                  ctx.holder.address,
                  ctx.other.address,
                  value,
                  []
                )
            ).to.be.revertedWithError({
              insufficientBalance: null,
            });
          });
        });

        // our PSP22 implementation doesnt support unlimited allowance
        // describe('when the spender has unlimited allowance', function () {
        //   beforeEach(async function () {
        //     await ctx.token.withSigner(ctx.holder).tx.approve(ctx.recipient.address, MAX_U128);
        //     ctx.tx = await ctx.token.withSigner(ctx.recipient).tx.transferFrom(ctx.holder.address, ctx.other.address, 1, []);
        //   });

        //   it('does decrease the spender allowance', async function () {
        //     expect((await ctx.token.query.allowance(ctx.holder.address, ctx.recipient.address)).value.ok?.toString()).to.equal(
        //       new BN(MAX_U128).subn(1).toString(),
        //     );
        //   });

        //   it('does emit an approval event', async function () {
        //     await expect(ctx.tx).to.emitEvent(ctx.token, 'Approval', {
        //       owner: ctx.holder.address,
        //       spender: ctx.recipient.address,
        //       value: new BN(MAX_U128).subn(1).toString(),
        //     });
        //   });
        // });
      });

      // our implementation doesn't revert on transfer to zero accountId
      // it('reverts when the recipient is the zero address', async function () {
      //   const value = initialSupply;
      //   await ctx.token.connect(ctx.holder).approve(ctx.recipient, value);
      //   await expect(ctx.token.connect(ctx.recipient).transferFrom(ctx.holder, ethers.ZeroAddress, value))
      //     .to.be.revertedWithCustomError(ctx.token, 'ERC20InvalidReceiver')
      //     .withArgs(ethers.ZeroAddress);
      // });
    });

    // it('reverts when the token owner is the zero address', async function () {
    //   const value = 0n;
    //   await expect(ctx.token.connect(ctx.recipient).transferFrom(ethers.ZeroAddress, ctx.recipient, value))
    //     .to.be.revertedWithCustomError(ctx.token, 'ERC20InvalidApprover')
    //     .withArgs(ethers.ZeroAddress);
    // });
  });

  describe("approve", function () {
    shouldBehaveLikePSP22Approve(() => ctx);
  });
}

type shouldBehaveLikePSP22TransferParams = {
  token: any;
  initialSupply: BN;
  holder: KeyringPair;
  recipient: KeyringPair;
  transfer: (
    from: KeyringPair,
    to: KeyringPair,
    value: BN
  ) => Promise<SignAndSendSuccessResponse>;
};

export function shouldBehaveLikePSP22Transfer(
  getParams: () => shouldBehaveLikePSP22TransferParams
) {
  const ctx: shouldBehaveLikePSP22TransferParams & {
    tx: any;
    transfer: (
      from: KeyringPair,
      to: KeyringPair,
      value: BN
    ) => Promise<SignAndSendSuccessResponse>;
  } = {} as any;
  describe("when the recipient is not the zero address", function () {
    beforeEach(function () {
      Object.assign(ctx, getParams());
    });
    it("reverts when the sender does not have enough balance", async function () {
      const value = ctx.initialSupply.addn(1);
      await expect(
        ctx.token
          .withSigner(ctx.holder)
          .query.transfer(ctx.recipient.address, value)
      ).to.be.revertedWithError({
        insufficientBalance: null,
      });
    });

    describe("when the sender transfers all balance", function () {
      let value: BN;

      beforeEach(async function () {
        value = ctx.initialSupply;
        ctx.tx = await ctx.transfer(ctx.holder, ctx.recipient, value);
      });

      it("transfers the requested value", async function () {
        await expect(ctx.tx).to.changePSP22Balances(
          ctx.token,
          [ctx.holder.address, ctx.recipient.address],
          [value.neg(), value]
        );
      });

      it("emits a transfer event", async function () {
        await expect(ctx.tx).to.emitEvent(ctx.token, "Transfer", {
          from: ctx.holder.address,
          to: ctx.recipient.address,
          value: value,
        });
      });
    });

    describe("when the sender transfers zero tokens", function () {
      const value = new BN(0);

      beforeEach(async function () {
        ctx.tx = await ctx.transfer(ctx.holder, ctx.recipient, value);
      });

      it("transfers the requested value", async function () {
        await expect(ctx.tx).to.changePSP22Balances(
          ctx.token,
          [ctx.holder.address, ctx.recipient.address],
          [new BN(0), new BN(0)]
        );
      });

      it("emits a transfer event", async function () {
        await expect(ctx.tx).to.emitEvent(ctx.token, "Transfer", {
          from: ctx.holder.address,
          to: ctx.recipient.address,
          value: value,
        });
      });
    });
  });
}
type shouldBehaveLikePSP22ApproveParams = {
  token: PSP22;
  initialSupply: BN;
  holder: KeyringPair;
  recipient: KeyringPair;
  approve: (
    from: KeyringPair,
    to: KeyringPair,
    amount: BN
  ) => Promise<SignAndSendSuccessResponse>;
};

export function shouldBehaveLikePSP22Approve(
  getParams: () => shouldBehaveLikePSP22ApproveParams
) {
  const ctx: shouldBehaveLikePSP22ApproveParams & {
    tx: any;
  } = {} as any;
  describe("when the spender is not the zero address", function () {
    beforeEach(async function () {
      Object.assign(ctx, getParams());
    });
    describe("when the sender has enough balance", function () {
      let value: BN;

      beforeEach(async function () {
        value = ctx.initialSupply;
      });

      it("emits an approval event", async function () {
        await expect(
          ctx.approve(ctx.holder, ctx.recipient, value)
        ).to.emitEvent(ctx.token, "Approval", {
          owner: ctx.holder.address,
          spender: ctx.recipient.address,
          value: value,
        });
      });

      it("approves the requested value when there was no approved value before", async function () {
        await ctx.approve(ctx.holder, ctx.recipient, value);

        expect(
          (
            await ctx.token.query.allowance(
              ctx.holder.address,
              ctx.recipient.address
            )
          ).value.ok?.toString()
        ).to.equal(value.toString());
      });

      it("approves the requested value and replaces the previous one when the spender had an approved value", async function () {
        await ctx.approve(ctx.holder, ctx.recipient, new BN(1));
        await ctx.approve(ctx.holder, ctx.recipient, value);

        expect(
          (
            await ctx.token.query.allowance(
              ctx.holder.address,
              ctx.recipient.address
            )
          ).value.ok?.toString()
        ).to.equal(value.toString());
      });
    });

    describe("when the sender does not have enough balance", function () {
      let value: BN;

      beforeEach(async function () {
        value = ctx.initialSupply.addn(1);
      });

      it("emits an approval event", async function () {
        await expect(
          ctx.approve(ctx.holder, ctx.recipient, value)
        ).to.emitEvent(ctx.token, "Approval", {
          owner: ctx.holder.address,
          spender: ctx.recipient.address,
          value: value,
        });
      });

      it("approves the requested value when there was no approved value before", async function () {
        await ctx.approve(ctx.holder, ctx.recipient, value);

        expect(
          (
            await ctx.token.query.allowance(
              ctx.holder.address,
              ctx.recipient.address
            )
          ).value.ok?.toString()
        ).to.equal(value.toString());
      });

      it("approves the requested value and replaces the previous one when the spender had an approved value", async function () {
        await ctx.approve(ctx.holder, ctx.recipient, new BN(1));
        await ctx.approve(ctx.holder, ctx.recipient, value);

        expect(
          (
            await ctx.token.query.allowance(
              ctx.holder.address,
              ctx.recipient.address
            )
          ).value.ok?.toString()
        ).to.equal(value.toString());
      });
    });
  });
}
