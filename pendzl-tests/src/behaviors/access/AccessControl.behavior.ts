import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";
import {
  AccessControl,
  AccessControlError,
} from "../../types/AccessControl.type";
import "chai-as-promised";
import "wookashwackomytest-polkahat-chai-matchers";
import { AccessControlInternal } from "../../types/AccessControlInternal.type";

export const DEFAULT_ADMIN_ROLE = 0;
export const ROLE = 1;
export const OTHER_ROLE = 2;

type ShouldBehaveLikeAccessControlParams = {
  contract: AccessControl;
  defaultAdmin: KeyringPair;
  accounts: KeyringPair[];
};

export function shouldBehaveLikeAccessControl(
  getParams: () => ShouldBehaveLikeAccessControlParams
) {
  const ctx: ShouldBehaveLikeAccessControlParams & {
    authorized: KeyringPair;
    other: KeyringPair;
    otherAdmin: KeyringPair;
  } = {} as any;
  beforeEach(async function () {
    Object.assign(ctx, getParams());
    ctx.authorized = ctx.accounts[0];
    ctx.other = ctx.accounts[1];
    ctx.otherAdmin = ctx.accounts[2];
  });

  describe("default admin", function () {
    it("deployer has default admin role", async function () {
      expect(
        (
          await ctx.contract.query.hasRole(
            DEFAULT_ADMIN_ROLE,
            ctx.defaultAdmin.address
          )
        ).value.ok
      ).to.be.true;
    });

    it("other roles's admin is the default admin role", async function () {
      expect(
        (await ctx.contract.query.getRoleAdmin(ROLE)).value.ok?.toNumber()
      ).to.equal(DEFAULT_ADMIN_ROLE);
    });

    it("default admin role's admin is itself", async function () {
      expect(
        (
          await ctx.contract.query.getRoleAdmin(DEFAULT_ADMIN_ROLE)
        ).value.ok?.toNumber()
      ).to.equal(DEFAULT_ADMIN_ROLE);
    });
  });

  describe("granting", function () {
    beforeEach(async function () {
      await ctx.contract
        .withSigner(ctx.defaultAdmin)
        .tx.grantRole(ROLE, ctx.authorized.address);
    });

    it("non-admin cannot grant role to other accounts", async function () {
      await expect(
        ctx.contract
          .withSigner(ctx.other)
          .query.grantRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });

    it("accounts cannot be granted a role multiple times", async function () {
      await ctx.contract
        .withSigner(ctx.defaultAdmin)
        .tx.grantRole(ROLE, ctx.other.address);
      await expect(
        ctx.contract
          .withSigner(ctx.defaultAdmin)
          .query.grantRole(ROLE, ctx.other.address)
      ).to.be.revertedWithError(AccessControlError.roleRedundant);
    });
  });

  describe("revoking", function () {
    it("roles that are not had cannot be revoked", async function () {
      expect(
        (await ctx.contract.query.hasRole(ROLE, ctx.authorized.address)).value
          .ok
      ).to.be.false;

      await expect(
        ctx.contract
          .withSigner(ctx.defaultAdmin)
          .query.revokeRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });

    describe("with granted role", function () {
      beforeEach(async function () {
        await ctx.contract
          .withSigner(ctx.defaultAdmin)
          .tx.grantRole(ROLE, ctx.authorized.address);
      });

      it("admin can revoke role", async function () {
        await expect(
          ctx.contract
            .withSigner(ctx.defaultAdmin)
            .tx.revokeRole(ROLE, ctx.authorized.address)
        ).to.emitEvent(ctx.contract, "RoleRevoked", {
          role: ROLE,
          account: ctx.authorized.address,
          sender: ctx.defaultAdmin.address,
        });

        expect(
          (await ctx.contract.query.hasRole(ROLE, ctx.authorized.address)).value
            .ok
        ).to.be.false;
      });

      it("non-admin cannot revoke role", async function () {
        await expect(
          ctx.contract
            .withSigner(ctx.other)
            .query.revokeRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });

      it("a role cannot be revoked multiple times", async function () {
        await ctx.contract
          .withSigner(ctx.defaultAdmin)
          .tx.revokeRole(ROLE, ctx.authorized.address);

        await expect(
          ctx.contract
            .withSigner(ctx.defaultAdmin)
            .query.revokeRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });
    });
  });

  describe("renouncing", function () {
    it("roles that are not had cannot be renounced", async function () {
      const queryRes = await ctx.contract.query.hasRole(
        ROLE,
        ctx.authorized.address
      );
      await expect(
        ctx.contract
          .withSigner(ctx.authorized)
          .query.renounceRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });

    describe("with granted role", function () {
      beforeEach(async function () {
        await ctx.contract
          .withSigner(ctx.defaultAdmin)
          .tx.grantRole(ROLE, ctx.authorized.address);
      });

      it("bearer can renounce role", async function () {
        await expect(
          ctx.contract
            .withSigner(ctx.authorized)
            .tx.renounceRole(ROLE, ctx.authorized.address)
        ).to.emitEvent(ctx.contract, "RoleRevoked", {
          role: ROLE,
          account: ctx.authorized.address,
          sender: ctx.authorized.address,
        });

        expect(
          (await ctx.contract.query.hasRole(ROLE, ctx.authorized.address)).value
            ?.ok
        ).to.be.false;
      });

      it("only the sender can renounce their roles", async function () {
        expect(
          ctx.contract
            .withSigner(ctx.defaultAdmin)
            .query.renounceRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.invalidCaller);
      });

      it("a role cannot be renounced multiple times", async function () {
        await ctx.contract
          .withSigner(ctx.authorized)
          .tx.renounceRole(ROLE, ctx.authorized.address);

        await expect(
          ctx.contract
            .withSigner(ctx.authorized)
            .query.renounceRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });
    });
  });

  describe("setting role admin", function () {
    beforeEach(async function () {
      await expect(
        ctx.contract
          .withSigner(ctx.defaultAdmin)
          .tx.setRoleAdmin(ROLE, OTHER_ROLE)
      ).to.emitEvent(ctx.contract, "RoleAdminChanged", {
        role: ROLE,
        previous: DEFAULT_ADMIN_ROLE,
        new: OTHER_ROLE,
      });

      await ctx.contract
        .withSigner(ctx.defaultAdmin)
        .tx.grantRole(OTHER_ROLE, ctx.otherAdmin.address);
    });

    it("a role's admin role can be changed", async function () {
      expect(
        (await ctx.contract.query.getRoleAdmin(ROLE)).value.ok?.toNumber()
      ).to.equal(OTHER_ROLE);
    });

    it("the new admin can grant roles", async function () {
      await expect(
        ctx.contract
          .withSigner(ctx.otherAdmin)
          .tx.grantRole(ROLE, ctx.authorized.address)
      ).to.emitEvent(ctx.contract, "RoleGranted", {
        role: ROLE,
        grantee: ctx.authorized.address,
        grantor: ctx.otherAdmin.address,
      });
    });

    it("the new admin can revoke roles", async function () {
      await ctx.contract
        .withSigner(ctx.otherAdmin)
        .tx.grantRole(ROLE, ctx.authorized.address);
      await expect(
        ctx.contract
          .withSigner(ctx.otherAdmin)
          .tx.revokeRole(ROLE, ctx.authorized.address)
      ).to.emitEvent(ctx.contract, "RoleRevoked", {
        role: ROLE,
        account: ctx.authorized.address,
        sender: ctx.otherAdmin.address,
      });
    });

    it("a role's previous admins no longer grant roles", async function () {
      await expect(
        ctx.contract
          .withSigner(ctx.defaultAdmin)
          .query.grantRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });

    it("a role's previous admins no longer revoke roles", async function () {
      await expect(
        ctx.contract
          .withSigner(ctx.defaultAdmin)
          .query.revokeRole(ROLE, ctx.authorized.address)
      ).to.be.revertedWithError(AccessControlError.missingRole);
    });
  });
}

type ShouldBehaveLikeAccessInternalControlParams = {
  contract: AccessControlInternal;
  defaultAdmin: KeyringPair;
  accounts: KeyringPair[];
};

export function shouldBehaveLikeAccessControlInternal(
  getParams: () => ShouldBehaveLikeAccessInternalControlParams
) {
  const ctx: ShouldBehaveLikeAccessInternalControlParams & {
    authorized: KeyringPair;
    other: KeyringPair;
    otherAdmin: KeyringPair;
  } = {} as any;
  describe("internal functions", function () {
    beforeEach(async function () {
      Object.assign(ctx, getParams());
      ctx.authorized = ctx.accounts[0];
      ctx.other = ctx.accounts[1];
      ctx.otherAdmin = ctx.accounts[2];
    });
    describe("onlyRole modifier", function () {
      beforeEach(async function () {
        await ctx.contract
          .withSigner(ctx.defaultAdmin)
          .tx.tGrantRole(ROLE, ctx.authorized.address);
      });

      it("do not revert if sender has role", async function () {
        expect(
          (ctx.contract as AccessControlInternal)
            .withSigner(ctx.authorized)
            .query.tEnsureHasRole(ROLE)
        ).not.to.be.revertedWithError(AccessControlError.missingRole);
      });

      it("revert if sender doesn't have role #1", async function () {
        await expect(
          (ctx.contract as AccessControlInternal)
            .withSigner(ctx.other)
            .query.tEnsureHasRole(ROLE)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });

      it("revert if sender doesn't have role #2", async function () {
        await expect(
          (ctx.contract as AccessControlInternal)
            .withSigner(ctx.authorized)
            .query.tEnsureHasRole(OTHER_ROLE)
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });
    });
    describe("_grantRole", function () {
      it("return true if the account does not have the role", async function () {
        await expect(
          ctx.contract.tx.tGrantRole(ROLE, ctx.authorized.address)
        ).to.emitEvent(ctx.contract, "RoleGranted", {
          role: ROLE,
          grantee: ctx.authorized.address,
          grantor: ctx.defaultAdmin.address,
        });
      });

      it("return false if the account has the role", async function () {
        await ctx.contract.tx.tGrantRole(ROLE, ctx.authorized.address);

        await expect(
          ctx.contract.query.tGrantRole(ROLE, ctx.authorized.address)
        ).to.be.revertedWithError(AccessControlError.roleRedundant);
      });
    });

    describe("_revokeRole", function () {
      it("return true if the account has the role", async function () {
        await ctx.contract.tx.tGrantRole(ROLE, ctx.authorized.address);

        await expect(
          ctx.contract.tx.tRevokeRole(ROLE, ctx.authorized.address)
        ).to.emitEvent(ctx.contract, "RoleRevoked", {
          role: ROLE,
          account: ctx.authorized.address,
          sender: ctx.defaultAdmin.address,
        });
      });

      it("return false if the account does not have the role", async function () {
        await expect(
          (ctx.contract as AccessControlInternal).query.tRevokeRole(
            ROLE,
            ctx.authorized.address
          )
        ).to.be.revertedWithError(AccessControlError.missingRole);
      });
    });
  });
}
