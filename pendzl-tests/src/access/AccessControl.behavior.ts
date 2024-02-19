// import { KeyringPair } from "@polkadot/keyring/types";
// import { expect } from "chai";
// import "wookashwackomytest-polkahat-chai-matchers";

// export const DEFAULT_ADMIN_ROLE = 0;
// export const ROLE = 1;
// export const OTHER_ROLE = 2;

// export enum ExpectedAccessControlError {
//   invalidCaller = "InvalidCaller",
//   missingRole = "MissingRole",
//   roleRedundant = "RoleRedundant",
// }

// type ShouldBehaveLikeAccessControlParams = {
//   mock: any;
//   defaultAdmin: KeyringPair;
//   delay: number;
//   accounts: KeyringPair[];
//   authorized: KeyringPair;
//   other: KeyringPair;
//   otherAdmin: KeyringPair;
// };

// export function shouldBehaveLikeAccessControl(
//   getParams: () => ShouldBehaveLikeAccessControlParams
// ) {
//   let ctx: ShouldBehaveLikeAccessControlParams;
//   beforeEach(async function () {
//     ctx = getParams();
//     [ctx.authorized, ctx.other, ctx.otherAdmin] = ctx.accounts;
//   });

//   describe("default admin", function () {
//     it("deployer has default admin role", async function () {
//       expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin.address)).to.be
//         .true;
//     });

//     it("other roles's admin is the default admin role", async function () {
//       expect(await ctx.mock.tx.getRoleAdmin(ROLE)).to.equal(DEFAULT_ADMIN_ROLE);
//     });

//     it("default admin role's admin is itself", async function () {
//       expect(await ctx.mock.tx.getRoleAdmin(DEFAULT_ADMIN_ROLE)).to.equal(
//         DEFAULT_ADMIN_ROLE
//       );
//     });
//   });

//   describe("granting", function () {
//     beforeEach(async function () {
//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .grantRole(ROLE, ctx.authorized);
//     });

//     it("non-admin cannot grant role to other accounts", async function () {
//       await expect(
//         ctx.mock.tx.withSigner(ctx.other).grantRole(ROLE, ctx.authorized)
//       ).to.be.revertedWithError(ExpectedAccessControlError.missingRole);
//     });

//     it("accounts can be granted a role multiple times", async function () {
//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .grantRole(ROLE, ctx.authorized);
//       expect(
//         ctx.mock.tx.withSigner(ctx.defaultAdmin).grantRole(ROLE, ctx.authorized)
//       ).to.not.emitEvent(ctx.mock, "RoleGranted");
//     });
//   });

//   describe("revoking", function () {
//     it("roles that are not had can be revoked", async function () {
//       expect(await ctx.mock.tx.hasRole(ROLE, ctx.authorized)).to.be.false;

//       await expect(
//         ctx.mock.tx.withSigner(ctx.defaultAdmin).revokeRole(ROLE, ctx.authorized)
//       ).to.not.emitEvent(ctx.mock, "RoleRevoked");
//     });

//     describe("with granted role", function () {
//       beforeEach(async function () {
//         await ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .grantRole(ROLE, ctx.authorized);
//       });

//       it("admin can revoke role", async function () {
//         await expect(
//           ctx.mock.tx.withSigner(ctx.defaultAdmin).revokeRole(ROLE, ctx.authorized)
//         )
//           .to.emitEvent(ctx.mock, "RoleRevoked")
//           .withArgs(ROLE, ctx.authorized, ctx.defaultAdmin);

//         expect(await ctx.mock.tx.hasRole(ROLE, ctx.authorized)).to.be.false;
//       });

//       it("non-admin cannot revoke role", async function () {
//         await expect(
//           ctx.mock.tx.withSigner(ctx.other).revokeRole(ROLE, ctx.authorized)
//         )
//           .to.be.revertedWithError(
//             ctx.mock,
//             ExpectedAccessControlError.missingRole
//           )
//           .withArgs(ctx.other, DEFAULT_ADMIN_ROLE);
//       });

//       it("a role can be revoked multiple times", async function () {
//         await ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .revokeRole(ROLE, ctx.authorized);

//         expect(
//           ctx.mock.tx.withSigner(ctx.defaultAdmin).revokeRole(ROLE, ctx.authorized)
//         ).to.not.emitEvent(ctx.mock, "RoleRevoked");
//       });
//     });
//   });

//   describe("renouncing", function () {
//     it("roles that are not had can be renounced", async function () {
//       await expect(
//         ctx.mock.tx.withSigner(ctx.authorized).renounceRole(ROLE, ctx.authorized)
//       ).to.not.emitEvent(ctx.mock, "RoleRevoked");
//     });

//     describe("with granted role", function () {
//       beforeEach(async function () {
//         await ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .grantRole(ROLE, ctx.authorized);
//       });

//       it("bearer can renounce role", async function () {
//         await expect(
//           ctx.mock.tx.withSigner(ctx.authorized).renounceRole(ROLE, ctx.authorized)
//         )
//           .to.emitEvent(ctx.mock, "RoleRevoked")
//           .withArgs(ROLE, ctx.authorized, ctx.authorized);

//         expect(await ctx.mock.tx.hasRole(ROLE, ctx.authorized)).to.be.false;
//       });

//       it("only the sender can renounce their roles", async function () {
//         expect(
//           ctx.mock
//             .withSigner(ctx.defaultAdmin)
//             .renounceRole(ROLE, ctx.authorized)
//         ).to.be.revertedWithError(ctx.mock, "AccessControlBadConfirmation");
//       });

//       it("a role can be renounced multiple times", async function () {
//         await ctx.mock
//           .withSigner(ctx.authorized)
//           .renounceRole(ROLE, ctx.authorized);

//         await expect(
//           ctx.mock.tx.withSigner(ctx.authorized).renounceRole(ROLE, ctx.authorized)
//         ).not.to.emitEvent(ctx.mock, "RoleRevoked");
//       });
//     });
//   });

//   describe("setting role admin", function () {
//     beforeEach(async function () {
//       await expect(ctx.mock.tx.$_setRoleAdmin(ROLE, OTHER_ROLE))
//         .to.emitEvent(ctx.mock, "RoleAdminChanged")
//         .withArgs(ROLE, DEFAULT_ADMIN_ROLE, OTHER_ROLE);

//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .grantRole(OTHER_ROLE, ctx.otherAdmin);
//     });

//     it("a role's admin role can be changed", async function () {
//       expect(await ctx.mock.tx.getRoleAdmin(ROLE)).to.equal(OTHER_ROLE);
//     });

//     it("the new admin can grant roles", async function () {
//       await expect(
//         ctx.mock.tx.withSigner(ctx.otherAdmin).grantRole(ROLE, ctx.authorized)
//       )
//         .to.emitEvent(ctx.mock, "RoleGranted")
//         .withArgs(ROLE, ctx.authorized, ctx.otherAdmin);
//     });

//     it("the new admin can revoke roles", async function () {
//       await ctx.mock.tx.withSigner(ctx.otherAdmin).grantRole(ROLE, ctx.authorized);
//       await expect(
//         ctx.mock.tx.withSigner(ctx.otherAdmin).revokeRole(ROLE, ctx.authorized)
//       )
//         .to.emitEvent(ctx.mock, "RoleRevoked")
//         .withArgs(ROLE, ctx.authorized, ctx.otherAdmin);
//     });

//     it("a role's previous admins no longer grant roles", async function () {
//       await expect(
//         ctx.mock.tx.withSigner(ctx.defaultAdmin).grantRole(ROLE, ctx.authorized)
//       )
//         .to.be.revertedWithError(
//           ctx.mock,
//           ExpectedAccessControlError.missingRole
//         )
//         .withArgs(ctx.defaultAdmin, OTHER_ROLE);
//     });

//     it("a role's previous admins no longer revoke roles", async function () {
//       await expect(
//         ctx.mock.tx.withSigner(ctx.defaultAdmin).revokeRole(ROLE, ctx.authorized)
//       )
//         .to.be.revertedWithError(
//           ctx.mock,
//           ExpectedAccessControlError.missingRole
//         )
//         .withArgs(ctx.defaultAdmin, OTHER_ROLE);
//     });
//   });

//   describe("onlyRole modifier", function () {
//     beforeEach(async function () {
//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .grantRole(ROLE, ctx.authorized);
//     });

//     it("do not revert if sender has role", async function () {
//       await ctx.mock.tx.withSigner(ctx.authorized).$_checkRole(ROLE);
//     });

//     it("revert if sender doesn't have role #1", async function () {
//       await expect(ctx.mock.tx.withSigner(ctx.other).$_checkRole(ROLE))
//         .to.be.revertedWithError(
//           ctx.mock,
//           ExpectedAccessControlError.missingRole
//         )
//         .withArgs(ctx.other, ROLE);
//     });

//     it("revert if sender doesn't have role #2", async function () {
//       await expect(ctx.mock.tx.withSigner(ctx.authorized).$_checkRole(OTHER_ROLE))
//         .to.be.revertedWithError(
//           ctx.mock,
//           ExpectedAccessControlError.missingRole
//         )
//         .withArgs(ctx.authorized, OTHER_ROLE);
//     });
//   });

//   describe("internal functions", function () {
//     describe("_grantRole", function () {
//       it("return true if the account does not have the role", async function () {
//         await expect(ctx.mock.tx.$_grantRole(ROLE, ctx.authorized))
//           .to.emitEvent(ctx.mock, "return$_grantRole")
//           .withArgs(true);
//       });

//       it("return false if the account has the role", async function () {
//         await ctx.mock.tx.$_grantRole(ROLE, ctx.authorized);

//         await expect(ctx.mock.tx.$_grantRole(ROLE, ctx.authorized))
//           .to.emitEvent(ctx.mock, "return$_grantRole")
//           .withArgs(false);
//       });
//     });

//     describe("_revokeRole", function () {
//       it("return true if the account has the role", async function () {
//         await ctx.mock.tx.$_grantRole(ROLE, ctx.authorized);

//         await expect(ctx.mock.tx.$_revokeRole(ROLE, ctx.authorized))
//           .to.emitEvent(ctx.mock, "return$_revokeRole")
//           .withArgs(true);
//       });

//       it("return false if the account does not have the role", async function () {
//         await expect(ctx.mock.tx.$_revokeRole(ROLE, ctx.authorized))
//           .to.emitEvent(ctx.mock, "return$_revokeRole")
//           .withArgs(false);
//       });
//     });
//   });
// }

// export function shouldBehaveLikeAccessControlDefaultAdminRules() {
//   beforeEach(async function () {
//     [ctx.newDefaultAdmin, ctx.other] = ctx.accounts;
//   });

//   for (const getter of ["owner", "defaultAdmin"]) {
//     describe(`${getter}()`, function () {
//       it("has a default set to the initial default admin", async function () {
//         const value = await ctx.mock[getter]();
//         expect(value).to.equal(ctx.defaultAdmin);
//         expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, value)).to.be.true;
//       });

//       it("changes if the default admin changes", async function () {
//         // Starts an admin transfer
//         await ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .beginDefaultAdminTransfer(ctx.newDefaultAdmin);

//         const value = await ctx.mock[getter]();
//         expect(value).to.equal(ctx.newDefaultAdmin);
//       });
//     });
//   }

//   it("should revert if granting default admin role", async function () {
//     await expect(
//       ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .grantRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)
//     ).to.be.revertedWithError(
//       ctx.mock,
//       "AccessControlEnforcedDefaultAdminRules"
//     );
//   });

//   it("should revert if revoking default admin role", async function () {
//     await expect(
//       ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .revokeRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)
//     ).to.be.revertedWithError(
//       ctx.mock,
//       "AccessControlEnforcedDefaultAdminRules"
//     );
//   });

//   it("should revert if defaultAdmin's admin is changed", async function () {
//     await expect(
//       ctx.mock.tx.$_setRoleAdmin(DEFAULT_ADMIN_ROLE, OTHER_ROLE)
//     ).to.be.revertedWithError(
//       ctx.mock,
//       "AccessControlEnforcedDefaultAdminRules"
//     );
//   });

//   it("should not grant the default admin role twice", async function () {
//     await expect(
//       ctx.mock.tx.$_grantRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)
//     ).to.be.revertedWithError(
//       ctx.mock,
//       "AccessControlEnforcedDefaultAdminRules"
//     );
//   });

//   describe("begins a default admin transfer", function () {
//     it("reverts if called by non default admin accounts", async function () {
//       await expect(
//         ctx.mock
//           .withSigner(ctx.other)
//           .beginDefaultAdminTransfer(ctx.newDefaultAdmin)
//       )
//         .to.be.revertedWithError(
//           ctx.mock,
//           ExpectedAccessControlError.missingRole
//         )
//         .withArgs(ctx.other, DEFAULT_ADMIN_ROLE);
//     });
//     });
//   });

//   describe("accepts transfer admin", function () {
//     beforeEach(async function () {
//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .beginDefaultAdminTransfer(ctx.newDefaultAdmin);
//       ctx.acceptSchedule = (await time.clock.timestamp()) + ctx.delay;
//     });

//     it("should revert if caller is not pending default admin", async function () {
//       await time.increaseTo.timestamp(ctx.acceptSchedule + 1n, false);
//       await expect(ctx.mock.tx.withSigner(ctx.other).acceptDefaultAdminTransfer())
//         .to.be.revertedWithError(ctx.mock, "AccessControlInvalidDefaultAdmin")
//         .withArgs(ctx.other);
//     });

//     describe("when caller is pending default admin and delay has passed", function () {
//       beforeEach(async function () {
//         await time.increaseTo.timestamp(ctx.acceptSchedule + 1n, false);
//       });

//       it("accepts a transfer and changes default admin", async function () {
//         // Emit events
//         await expect(
//           ctx.mock.tx.withSigner(ctx.newDefaultAdmin).acceptDefaultAdminTransfer()
//         )
//           .to.emitEvent(ctx.mock, "RoleRevoked")
//           .withArgs(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin, ctx.newDefaultAdmin)
//           .to.emitEvent(ctx.mock, "RoleGranted")
//           .withArgs(
//             DEFAULT_ADMIN_ROLE,
//             ctx.newDefaultAdmin,
//             ctx.newDefaultAdmin
//           );

//         // Storage changes
//         expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)).to
//           .be.false;
//         expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, ctx.newDefaultAdmin))
//           .to.be.true;
//         expect(await ctx.mock.tx.owner()).to.equal(ctx.newDefaultAdmin);

//         // Resets pending default admin and schedule
//         const { newAdmin, schedule } = await ctx.mock.tx.pendingDefaultAdmin();
//         expect(newAdmin).to.equal(ethers.ZeroAddress);
//         expect(schedule).to.equal(0);
//       });
//     });
//   });

//   describe("cancels a default admin transfer", function () {
//     it("reverts if called by non default admin accounts", async function () {
//       await expect(ctx.mock.tx.withSigner(ctx.other).cancelDefaultAdminTransfer())
//         .to.be.revertedWithError(
//           ctx.mock,
//           ExpectedAccessControlError.missingRole
//         )
//         .withArgs(ctx.other, DEFAULT_ADMIN_ROLE);
//     });

//     describe("when there is a pending default admin transfer", function () {
//       beforeEach(async function () {
//         await ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .beginDefaultAdminTransfer(ctx.newDefaultAdmin);
//         ctx.acceptSchedule = (await time.clock.timestamp()) + ctx.delay;
//       });

//       for (const [fromSchedule, tag] of [
//         [-1n, "before"],
//         [0n, "exactly when"],
//         [1n, "after"],
//       ]) {
//         it(`resets pending default admin and schedule ${tag} transfer schedule passes`, async function () {
//           // Advance until passed delay
//           await time.increaseTo.timestamp(
//             ctx.acceptSchedule + fromSchedule,
//             false
//           );

//           await expect(
//             ctx.mock.tx.withSigner(ctx.defaultAdmin).cancelDefaultAdminTransfer()
//           ).to.emitEvent(ctx.mock, "DefaultAdminTransferCanceled");

//           const { newAdmin, schedule } = await ctx.mock.tx.pendingDefaultAdmin();
//           expect(newAdmin).to.equal(ethers.ZeroAddress);
//           expect(schedule).to.equal(0);
//         });
//       }

//       it("should revert if the previous default admin tries to accept", async function () {
//         await ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .cancelDefaultAdminTransfer();

//         // Advance until passed delay
//         await time.increaseTo.timestamp(ctx.acceptSchedule + 1n, false);

//         // Previous pending default admin should not be able to accept after cancellation.
//         await expect(
//           ctx.mock.tx.withSigner(ctx.newDefaultAdmin).acceptDefaultAdminTransfer()
//         )
//           .to.be.revertedWithError(ctx.mock, "AccessControlInvalidDefaultAdmin")
//           .withArgs(ctx.newDefaultAdmin);
//       });
//     });

//     describe("when there is no pending default admin transfer", async function () {
//       it("should succeed without changes", async function () {
//         await expect(
//           ctx.mock.tx.withSigner(ctx.defaultAdmin).cancelDefaultAdminTransfer()
//         ).to.not.emitEvent(ctx.mock, "DefaultAdminTransferCanceled");

//         const { newAdmin, schedule } = await ctx.mock.tx.pendingDefaultAdmin();
//         expect(newAdmin).to.equal(ethers.ZeroAddress);
//         expect(schedule).to.equal(0);
//       });
//     });
//   });

//   describe("renounces admin", function () {
//     beforeEach(async function () {
//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .beginDefaultAdminTransfer(ethers.ZeroAddress);
//       ctx.expectedSchedule = (await time.clock.timestamp()) + ctx.delay;
//     });

//     it("reverts if caller is not default admin", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);
//       await expect(
//         ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .renounceRole(DEFAULT_ADMIN_ROLE, ctx.other)
//       ).to.be.revertedWithError(ctx.mock, "AccessControlBadConfirmation");
//     });

//     it("renouncing the admin role when not an admin doesn't affect the schedule", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);
//       await ctx.mock
//         .withSigner(ctx.other)
//         .renounceRole(DEFAULT_ADMIN_ROLE, ctx.other);

//       const { newAdmin, schedule } = await ctx.mock.tx.pendingDefaultAdmin();
//       expect(newAdmin).to.equal(ethers.ZeroAddress);
//       expect(schedule).to.equal(ctx.expectedSchedule);
//     });

//     it("keeps defaultAdmin consistent with hasRole if another non-defaultAdmin user renounces the DEFAULT_ADMIN_ROLE", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);

//       // This passes because it's a noop
//       await ctx.mock
//         .withSigner(ctx.other)
//         .renounceRole(DEFAULT_ADMIN_ROLE, ctx.other);

//       expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)).to.be
//         .true;
//       expect(await ctx.mock.tx.defaultAdmin()).to.equal(ctx.defaultAdmin);
//     });

//     it("renounces role", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);
//       await expect(
//         ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .renounceRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)
//       )
//         .to.emitEvent(ctx.mock, "RoleRevoked")
//         .withArgs(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin, ctx.defaultAdmin);

//       expect(await ctx.mock.tx.hasRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin)).to.be
//         .false;
//       expect(await ctx.mock.tx.defaultAdmin()).to.equal(ethers.ZeroAddress);
//       expect(await ctx.mock.tx.owner()).to.equal(ethers.ZeroAddress);

//       const { newAdmin, schedule } = await ctx.mock.tx.pendingDefaultAdmin();
//       expect(newAdmin).to.equal(ethers.ZeroAddress);
//       expect(schedule).to.equal(0);
//     });

//     it("allows to recover access using the internal _grantRole", async function () {
//       await time.increaseBy.timestamp(ctx.delay + 1n, false);
//       await ctx.mock
//         .withSigner(ctx.defaultAdmin)
//         .renounceRole(DEFAULT_ADMIN_ROLE, ctx.defaultAdmin);

//       await expect(
//         ctx.mock
//           .withSigner(ctx.defaultAdmin)
//           .$_grantRole(DEFAULT_ADMIN_ROLE, ctx.other)
//       )
//         .to.emitEvent(ctx.mock, "RoleGranted")
//         .withArgs(DEFAULT_ADMIN_ROLE, ctx.other, ctx.defaultAdmin);
//     });
//   });
// }
