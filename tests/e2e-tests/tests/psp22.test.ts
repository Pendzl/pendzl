import { ApiPromise } from '@polkadot/api';
import { BN } from 'bn.js';
import { increaseBlockTimestamp, transferNoop } from 'tests/misc';
import { getLocalApiProviderWrapper, getSigners } from 'tests/setup/helpers';
import 'wookashwackomytest-polkahat-chai-matchers';
import { shouldBehaveLikeERC20 } from 'wookashwackomytest-pendzl-tests';
import MyPsp22Deployer from 'typechain/deployers/my_psp22';
import MyPsp22Contract from 'typechain/contracts/my_psp22';

const [owner, ...others] = getSigners();
const initialSupply = new BN(1000);
async function prepareEnvBase(api: ApiPromise) {
  await transferNoop(api);
  // to force using fake_time
  await increaseBlockTimestamp(api, 0);

  const deployRet = await new MyPsp22Deployer(api, owner).new(initialSupply);

  return { myPSP22: deployRet.contract };
}
describe.only('PSP 22', () => {
  let myPSP22: MyPsp22Contract;
  const apiProviderWrapper = getLocalApiProviderWrapper(9944);
  beforeEach(async () => {
    const api = await apiProviderWrapper.getAndWaitForReady();
    const contracts = await prepareEnvBase(api);
    myPSP22 = contracts.myPSP22;
  });

  shouldBehaveLikeERC20(() => ({ initialSupply, holder: owner, recipient: others[0], other: others[1], token: myPSP22 as any }));
});
