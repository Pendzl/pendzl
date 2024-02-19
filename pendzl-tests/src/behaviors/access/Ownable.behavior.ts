import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import "wookashwackomytest-polkahat-chai-matchers";
import { SignAndSendSuccessResponse } from "wookashwackomytest-typechain-types";
import { Ownable } from "../../types/Ownable.type";

export type ShouldBehaveLikeOwnableParams = {
  ownableDeployerCall: () => Promise<{
    result: SignAndSendSuccessResponse;
    contract: any;
  }>;
  owner: KeyringPair;
  other: KeyringPair;
};
export function shouldBehaveLikeOwnable(
  getParams: () => ShouldBehaveLikeOwnableParams
) {
  const ctx: ShouldBehaveLikeOwnableParams & {
    ownable: Ownable;
    tx: any;
  } = {} as any;

  describe("shouldBehaveLikeOwnable", function () {
    beforeEach(async function () {
      Object.assign(ctx, getParams());
      ctx.ownable = (await ctx.ownableDeployerCall()).contract;
    });

    describe("On deployment", function () {
      beforeEach(async function () {
        Object.assign(ctx, getParams());
        const promise = await ctx.ownableDeployerCall();
        ctx.tx = promise.result;
        ctx.ownable = promise.contract;
      });
      it("emits ownership transfer events during construction", async function () {
        await expect(ctx.tx).to.emitEvent(ctx.ownable, "OwnershipTransferred", {
          new: ctx.owner.address,
        });
      });

      it("has an owner", async function () {
        const owner = (await ctx.ownable.query.owner()).value.ok!;
        expect(owner).to.equal(ctx.owner.address);
      });
    });

    // pendzl::ownalbe allows for transfer to zero address
    // it("rejects zero address for initialOwner", async function () {
    //   await expect(ethers.deployContract("$Ownable", [ethers.ZeroAddress]))
    //     .to.be.revertedWithCustomError(
    //       { interface: ctx.ownable.interface },
    //       "OwnableInvalidOwner"
    //     )
    //     .withArgs(ethers.ZeroAddress);
    // });

    describe("transfer ownership", function () {
      it("changes owner after transfer and emits event", async function () {
        await expect(
          ctx.ownable
            .withSigner(ctx.owner)
            .tx.transferOwnership(ctx.other.address)
        ).to.emitEvent(ctx.ownable, "OwnershipTransferred", {
          new: ctx.other.address,
        });
        const newOwner = (await ctx.ownable.query.owner()).value.ok!;
        expect(newOwner).to.equal(ctx.other.address);
      });

      it("prevents non-owners from transferring", async function () {
        await expect(
          ctx.ownable
            .withSigner(ctx.other)
            .query.transferOwnership(ctx.other.address)
        ).to.be.revertedWithError("CallerIsNotOwner");
      });
    });

    describe("renounce ownership", function () {
      it("loses ownership after renouncement and emit event", async function () {
        await expect(
          ctx.ownable.withSigner(ctx.owner).tx.renounceOwnership()
        ).to.emitEvent(ctx.ownable, "OwnershipTransferred", { new: null });

        expect((await ctx.ownable.query.owner()).value.ok!).to.equal(null);
      });

      it("prevents non-owners from renouncement", async function () {
        await expect(
          ctx.ownable.withSigner(ctx.other).query.renounceOwnership()
        ).to.be.revertedWithError("CallerIsNotOwner");
      });

      // add elsewehre
      // it("allows to recover access using the internal _transferOwnership", async function () {
      //   await ctx.ownable.connect(ctx.owner).renounceOwnership();

      //   await expect(ctx.ownable.$_transferOwnership(ctx.other))
      //     .to.emit(ctx.ownable, "OwnershipTransferred")
      //     .withArgs(ethers.ZeroAddress, ctx.other);

      //   expect(await ctx.ownable.owner()).to.equal(ctx.other);
      // });
    });
  });
}
