import { ApiPromise } from '@polkadot/api';
import BN from 'bn.js';
import { expect } from 'chai';
import TPsp22Contract from 'typechain/contracts/t_psp22';
import TVesterContract from 'typechain/contracts/t_vester';
import TPsp22Deployer from 'typechain/deployers/t_psp22';
import TVesterDeployer from 'typechain/deployers/t_vester';
import { VestingData, VestingSchedule } from 'typechain/types-arguments/t_vester';
import { clock, duration, getSigners, localApi, time } from 'wookashwackomytest-polkahat-network-helpers';
import 'wookashwackomytest-polkahat-chai-matchers';

const [deployer, alice, bob, charlie, dave] = getSigners();
describe('Vester', () => {
  const ctx: {
    mock: TVesterContract;
  } = {} as any;

  let api: ApiPromise;
  beforeEach(async () => {
    api = await localApi.get();
    await time.setTo(10);
    const mock = await new TVesterDeployer(api, deployer).new();
    ctx.mock = mock.contract;
  });

  interface CreateVestingScheduleArgs {
    vestTo: string;
    asset: string | null;
    amount: number;
    schedule: VestingSchedule;
  }

  function createDurationAsAmountScheduleArgs(
    vestTo: string,
    asset: string | null,
    waitingDuration: number,
    amountAsDuration: number,
  ): CreateVestingScheduleArgs {
    return {
      vestTo,
      asset,
      amount: amountAsDuration,
      schedule: {
        constant: [waitingDuration, amountAsDuration],
      },
    };
  }

  describe('vesting psp22', () => {
    const vesterSubmitter = alice;
    let asset: TPsp22Contract;
    let createVestArgs: CreateVestingScheduleArgs;
    const initialSupply = new BN(1000);
    const TOKEN_NAME = 'SomeToken';
    const SYBOL = 'ST';
    const DECIMALS = 18;
    beforeEach(async () => {
      asset = (await new TPsp22Deployer(api, deployer).new(initialSupply, TOKEN_NAME, SYBOL, DECIMALS)).contract;
      createVestArgs = createDurationAsAmountScheduleArgs(charlie.address, asset.address, 0, 100);
    });
    it('should fail to create vesting schedule due to insufficient allowance', async function () {
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .query.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []),
      ).to.be.revertedWithError({
        psp22Error: { insufficientAllowance: null },
      });
    });

    it('should fail to create vesting schedule due to insufficient balance after allowance', async function () {
      await asset.withSigner(vesterSubmitter).tx.increaseAllowance(ctx.mock.address, createVestArgs.amount);
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .query.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []),
      ).to.be.revertedWithError({
        psp22Error: { insufficientBalance: null },
      });
    });

    it('should successfully create a vesting schedule after minting', async function () {
      await asset.tx.tMint(vesterSubmitter.address, createVestArgs.amount);
      await asset.withSigner(vesterSubmitter).tx.increaseAllowance(ctx.mock.address, createVestArgs.amount);
      const tx = ctx.mock
        .withSigner(vesterSubmitter)
        .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []);
      await expect(tx).to.changePSP22Balances(
        asset,
        [vesterSubmitter.address, ctx.mock.address],
        [new BN(createVestArgs.amount).neg(), new BN(createVestArgs.amount)],
      );
      await expect(tx).to.emitEvent(ctx.mock, 'VestingScheduled', {
        creator: vesterSubmitter.address,
        asset: createVestArgs.asset,
        receiver: createVestArgs.vestTo,
        amount: createVestArgs.amount,
        schedule: createVestArgs.schedule,
      });
      await expect(tx).to.emitEvent(asset, 'Transfer', {
        from: vesterSubmitter.address,
        to: ctx.mock.address,
        value: createVestArgs.amount,
      });
      const timestamp = await clock.fromTx.timestamp(await tx);
      await expect(ctx.mock.query.vestingScheduleOf(createVestArgs.vestTo, createVestArgs.asset, 0, [])).to.haveOkResult({
        amount: createVestArgs.amount,
        released: 0,
        schedule: createVestArgs.schedule,
        creationTime: timestamp,
      } as VestingData);
      await expect(ctx.mock.query.nextIdVestOf(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult(1);
    });

    describe('release', () => {
      beforeEach(async () => {
        await asset.tx.tMint(vesterSubmitter.address, createVestArgs.amount);
        await asset.withSigner(vesterSubmitter).tx.increaseAllowance(ctx.mock.address, createVestArgs.amount);
        await ctx.mock
          .withSigner(vesterSubmitter)
          .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []);
        await time.increase(
          parseInt(createVestArgs.schedule.constant![0].toString()) + parseInt(createVestArgs.schedule.constant![1].toString()) + 1,
        );
      });

      it('anyone can release', async function () {
        await expect(ctx.mock.withSigner(charlie).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
        await expect(ctx.mock.withSigner(alice).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
        await expect(ctx.mock.withSigner(bob).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
      });
      it('should release tokens correctly', async function () {
        const tx = ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
        await expect(tx).to.emitEvent(asset, 'Transfer', {
          from: ctx.mock.address,
          to: createVestArgs.vestTo,
          value: createVestArgs.amount,
        });

        await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
          caller: bob.address,
          to: createVestArgs.vestTo,
          asset: createVestArgs.asset,
          amount: createVestArgs.amount,
        });
        await expect(tx).to.changePSP22Balances(
          asset,
          [ctx.mock.address, createVestArgs.vestTo],
          [new BN(createVestArgs.amount).neg(), new BN(createVestArgs.amount)],
        );
      });
    });

    describe('partial release', () => {
      let creationTimestamp: number;
      const waitingDuration = duration.days(3);
      const vestingDuration = duration.days(6);
      beforeEach(async () => {
        createVestArgs = createDurationAsAmountScheduleArgs(charlie.address, asset.address, waitingDuration, vestingDuration);
        await asset.tx.tMint(vesterSubmitter.address, createVestArgs.amount);
        await asset.withSigner(vesterSubmitter).tx.increaseAllowance(ctx.mock.address, createVestArgs.amount);
        const tx = await ctx.mock
          .withSigner(vesterSubmitter)
          .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, []);

        creationTimestamp = await clock.fromTx.timestamp(tx);
      });

      describe('timestamp changes to just before the start time', () => {
        beforeEach(async () => {
          await time.setTo(creationTimestamp + parseInt(createVestArgs.schedule.constant![0].toString()) - 1);
        });

        it('releases nothing', async () => {
          await expect(ctx.mock.withSigner(bob).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult(0);
          const tx = ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
          await expect(tx).to.emitEvent(asset, 'Transfer', {
            from: ctx.mock.address,
            to: createVestArgs.vestTo,
            value: 0,
          });

          await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
            caller: bob.address,
            to: createVestArgs.vestTo,
            asset: createVestArgs.asset,
            amount: 0,
          });
          await expect(tx).to.changePSP22Balances(asset, [ctx.mock.address, createVestArgs.vestTo], [new BN(0), new BN(0)]);
        });

        describe('timestamp changes to just after the start time', () => {
          beforeEach(async () => {
            await time.setTo(creationTimestamp + parseInt(createVestArgs.schedule.constant![0].toString()) + 2);
          });
          it('try release succeeds & does release adequate amount of tokens eq 1', async () => {
            await expect(ctx.mock.withSigner(bob).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult(1);
            const tx = ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
            await expect(tx).to.emitEvent(asset, 'Transfer', {
              from: ctx.mock.address,
              to: createVestArgs.vestTo,
              value: 1,
            });

            await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
              caller: bob.address,
              to: createVestArgs.vestTo,
              asset: createVestArgs.asset,
              amount: 1,
            });
            await expect(tx).to.changePSP22Balances(asset, [ctx.mock.address, createVestArgs.vestTo], [new BN(-1), new BN(1)]);

            await expect(ctx.mock.query.vestingScheduleOf(createVestArgs.vestTo, createVestArgs.asset, 0, [])).to.haveOkResult({
              amount: createVestArgs.amount,
              released: 1,
              schedule: createVestArgs.schedule,
              creationTime: creationTimestamp,
            } as VestingData);
          });

          describe('one day passes', () => {
            beforeEach(async () => {
              await ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
              await time.increase(duration.days(1));
            });

            it('try release succeeds & does release adequate amount of tokens', async () => {
              await expect(ctx.mock.withSigner(bob).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult(duration.days(1));
              const tx = ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
              await expect(tx).to.emitEvent(asset, 'Transfer', {
                from: ctx.mock.address,
                to: createVestArgs.vestTo,
                value: duration.days(1),
              });

              await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
                caller: bob.address,
                to: createVestArgs.vestTo,
                asset: createVestArgs.asset,
                amount: duration.days(1),
              });
              await expect(tx).to.changePSP22Balances(
                asset,
                [ctx.mock.address, createVestArgs.vestTo, bob.address],
                [new BN(-duration.days(1)), new BN(duration.days(1)), new BN(0)],
              );

              await expect(ctx.mock.query.vestingScheduleOf(createVestArgs.vestTo, createVestArgs.asset, 0, [])).to.haveOkResult({
                amount: createVestArgs.amount,
                released: 1 + duration.days(1),
                schedule: createVestArgs.schedule,
                creationTime: creationTimestamp,
              } as VestingData);
            });

            describe('timestamp changes to just after the end time', () => {
              beforeEach(async () => {
                await ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
                await time.setTo(
                  creationTimestamp +
                    parseInt(createVestArgs.schedule.constant![0].toString()) +
                    parseInt(createVestArgs.schedule.constant![1].toString()) +
                    1,
                );
              });

              it('try release succeeds & does release the rest of tokens', async () => {
                await expect(ctx.mock.withSigner(bob).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult(
                  createVestArgs.amount - 1 - duration.days(1),
                );
                const tx = ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
                await expect(tx).to.emitEvent(asset, 'Transfer', {
                  from: ctx.mock.address,
                  to: createVestArgs.vestTo,
                  value: createVestArgs.amount - 1 - duration.days(1),
                });

                await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
                  caller: bob.address,
                  to: createVestArgs.vestTo,
                  asset: createVestArgs.asset,
                  amount: createVestArgs.amount - 1 - duration.days(1),
                });
                await expect(tx).to.changePSP22Balances(
                  asset,
                  [ctx.mock.address, createVestArgs.vestTo],
                  [new BN(-(createVestArgs.amount - 1 - duration.days(1))), new BN(createVestArgs.amount - 1 - duration.days(1))],
                );

                await expect(ctx.mock.query.vestingScheduleOf(createVestArgs.vestTo, createVestArgs.asset, 0, [])).to.haveOkResult(null);
                await expect(asset.query.balanceOf(createVestArgs.vestTo)).to.haveOkResult(createVestArgs.amount);
              });
            });
          });
        });
      });
    });
  });

  describe('vesting native token', () => {
    const vesterSubmitter = alice;
    let createVestArgs: CreateVestingScheduleArgs;
    const amount = 10_000_000; // Adjusted to match the Rust test's amount
    beforeEach(async () => {
      createVestArgs = createDurationAsAmountScheduleArgs(charlie.address, null, 0, amount); // asset is null for native token
    });

    it('should fail to create vesting schedule due to invalid amount paid (less than required)', async function () {
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .query.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, [], {
            value: createVestArgs.amount - 1,
          }),
      ).to.be.revertedWithError({ invalidAmountPaid: null });
    });

    it('should fail to create vesting schedule due to invalid amount paid (more than required)', async function () {
      await expect(
        ctx.mock
          .withSigner(vesterSubmitter)
          .query.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, [], {
            value: createVestArgs.amount + 1,
          }),
      ).to.be.revertedWithError({ invalidAmountPaid: null });
    });

    it('should successfully create a vesting schedule with the exact amount paid', async function () {
      const tx = ctx.mock
        .withSigner(vesterSubmitter)
        .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, [], {
          value: createVestArgs.amount,
        });

      await expect(tx).to.emitNativeEvent('Transfer', {
        from: vesterSubmitter.address,
        to: ctx.mock.address,
        amount: createVestArgs.amount,
      });
      await expect(tx).to.emitEvent(ctx.mock, 'VestingScheduled', {
        creator: vesterSubmitter.address,
        asset: createVestArgs.asset,
        receiver: createVestArgs.vestTo,
        amount: createVestArgs.amount,
        schedule: createVestArgs.schedule,
      });
      await expect(tx).to.changeBalances([ctx.mock.address], [new BN(createVestArgs.amount)]);
    });
    describe('release', () => {
      beforeEach(async () => {
        await ctx.mock
          .withSigner(vesterSubmitter)
          .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, [], {
            value: createVestArgs.amount,
          });
        await time.increase(
          parseInt(createVestArgs.schedule.constant![0].toString()) + parseInt(createVestArgs.schedule.constant![1].toString()) + 1,
        );
      });

      it('anyone can release', async function () {
        await expect(ctx.mock.withSigner(charlie).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
        await expect(ctx.mock.withSigner(alice).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
        await expect(ctx.mock.withSigner(bob).query.release(createVestArgs.vestTo, createVestArgs.asset, [])).to.haveOkResult();
      });
      it('should release tokens correctly', async function () {
        const tx = ctx.mock.withSigner(bob).tx.release(createVestArgs.vestTo, createVestArgs.asset, []);
        await expect(tx).to.emitNativeEvent('Transfer', {
          from: ctx.mock.address,
          to: charlie.address,
          amount: createVestArgs.amount,
        });

        await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
          caller: bob.address,
          to: createVestArgs.vestTo,
          asset: createVestArgs.asset,
          amount: createVestArgs.amount,
        });
        await expect(tx).to.changeBalances([ctx.mock.address, charlie.address], [new BN(createVestArgs.amount).neg(), new BN(createVestArgs.amount)]);
      });
    });
  });

  describe('release when multiple schedules created', () => {
    const vestTo = charlie;
    const vesterSubmitter = bob;
    const releaseCaller = dave;
    const creationTimestamp = duration.days(1);
    const firstActionTimestamp = duration.days(365);
    const createVestArgsVec: CreateVestingScheduleArgs[] = [
      createDurationAsAmountScheduleArgs(vestTo.address, null, firstActionTimestamp - duration.days(9) - creationTimestamp, duration.days(6)), // overdue (at the first_action_timestamp)
      createDurationAsAmountScheduleArgs(vestTo.address, null, firstActionTimestamp - duration.days(6) - creationTimestamp, duration.days(9)), // started (at the first_action_timestamp)
      createDurationAsAmountScheduleArgs(vestTo.address, null, firstActionTimestamp + duration.days(1) - creationTimestamp, duration.days(5)), // not started (at the first_action_timestamp)
      createDurationAsAmountScheduleArgs(vestTo.address, null, firstActionTimestamp + duration.days(3) - creationTimestamp, duration.days(6)), // not started (at the first_action_timestamp)
      createDurationAsAmountScheduleArgs(vestTo.address, null, firstActionTimestamp + duration.days(18) - creationTimestamp, duration.days(46)), // not started (at the first_action_timestamp)
    ];
    let startingBalance: BN;
    beforeEach(async () => {
      startingBalance = (await api.query.system.account(vestTo.address)).data.free;
      await time.setTo(creationTimestamp);
      for (const createVestArgs of createVestArgsVec) {
        await ctx.mock
          .withSigner(vesterSubmitter)
          .tx.createVest(createVestArgs.vestTo, createVestArgs.asset, createVestArgs.amount, createVestArgs.schedule, [], {
            value: createVestArgs.amount,
          });
      }
      await time.setTo(firstActionTimestamp);
    });

    it('should release appropriate amount', async () => {
      await expect(ctx.mock.withSigner(releaseCaller).query.release(vestTo.address, null, [])).to.haveOkResult(duration.days(12) - 1);
      await expect(ctx.mock.query.nextIdVestOf(vestTo.address, null, [])).to.haveOkResult(5);
      await expect(ctx.mock.query.vestingScheduleOf(vestTo.address, null, 5, [])).to.haveOkResult(null);
      for (let i = 0; i < 5; i++) {
        await expect(ctx.mock.query.vestingScheduleOf(vestTo.address, null, i, [])).to.haveOkResult({
          amount: createVestArgsVec[i].amount,
          released: 0,
          schedule: createVestArgsVec[i].schedule,
          creationTime: creationTimestamp,
        } as VestingData);
      }
      const tx = ctx.mock.withSigner(releaseCaller).tx.release(vestTo.address, null, []);
      await expect(tx).to.emitNativeEvent('Transfer', {
        from: ctx.mock.address,
        to: vestTo.address,
        amount: duration.days(12) - 1,
      });
      await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
        caller: releaseCaller.address,
        to: vestTo.address,
        asset: null,
        amount: duration.days(12) - 1,
      });
    });

    describe('move time past 2nd schedule end', () => {
      beforeEach(async () => {
        await ctx.mock.withSigner(releaseCaller).tx.release(vestTo.address, null, []);
        const vestingSchedule = createVestArgsVec[1].schedule.constant!;
        const vestingEnd = creationTimestamp + parseInt(vestingSchedule[0].toString()) + parseInt(vestingSchedule[1].toString());
        await time.setTo(vestingEnd + duration.days(1));
      });

      it('should release appropriate amount', async () => {
        await expect(ctx.mock.withSigner(releaseCaller).query.release(vestTo.address, null, [])).to.haveOkResult(duration.days(7) - 1);
        await expect(ctx.mock.query.nextIdVestOf(vestTo.address, null, [])).to.haveOkResult(4);
        await expect(ctx.mock.query.vestingScheduleOf(vestTo.address, null, 4, [])).to.haveOkResult(null);
        for (let i = 0; i < 4; i++) {
          await expect(ctx.mock.query.vestingScheduleOf(vestTo.address, null, i, []), `vesting schedule number ${i}`).to.haveOkResult(
            (v: any) => !!v,
          );
        }
        const tx = ctx.mock.withSigner(releaseCaller).tx.release(vestTo.address, null, []);
        await expect(tx).to.emitNativeEvent('Transfer', {
          from: ctx.mock.address,
          to: vestTo.address,
          amount: duration.days(7) - 1,
        });
        await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
          caller: releaseCaller.address,
          to: vestTo.address,
          asset: null,
          amount: duration.days(7) - 1,
        });
      });

      describe('move time past last schedule end', () => {
        beforeEach(async () => {
          await ctx.mock.withSigner(releaseCaller).tx.release(vestTo.address, null, []);
          const vestingSchedule = createVestArgsVec[createVestArgsVec.length - 1].schedule.constant!;
          const vestingEnd = creationTimestamp + parseInt(vestingSchedule[0].toString()) + parseInt(vestingSchedule[1].toString());
          await time.setTo(vestingEnd + 1);
        });

        it('should release appropriate amount', async () => {
          await expect(ctx.mock.withSigner(releaseCaller).query.release(vestTo.address, null, [])).to.haveOkResult(duration.days(53) + 2);
          const tx = ctx.mock.withSigner(releaseCaller).tx.release(vestTo.address, null, []);
          await expect(tx).to.emitNativeEvent('Transfer', {
            from: ctx.mock.address,
            to: vestTo.address,
            amount: duration.days(53) + 2,
          });
          await expect(tx).to.emitEvent(ctx.mock, 'TokenReleased', {
            caller: releaseCaller.address,
            to: vestTo.address,
            asset: null,
            amount: duration.days(53) + 2,
          });
          for (let i = 0; i < 5; i++) {
            await expect(ctx.mock.query.vestingScheduleOf(vestTo.address, null, i, [])).to.haveOkResult(null);
          }
          await expect(ctx.mock.query.nextIdVestOf(vestTo.address, null, [])).to.haveOkResult(0);
          await expect(ctx.mock.query.release(vestTo.address, null, [])).to.haveOkResult(0);
          const balancePost = (await api.query.system.account(vestTo.address)).data.free;
          const vesterBalance = (await api.query.system.account(ctx.mock.address)).data.free;
          expect(vesterBalance).to.equal(0);
          expect(balancePost).to.equal(startingBalance.add(new BN(createVestArgsVec.reduce((acc, x) => acc + x.amount, 0))));
        });
      });
    });
  });
});
