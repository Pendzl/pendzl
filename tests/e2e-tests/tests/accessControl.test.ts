import { ApiPromise } from '@polkadot/api';
import { localApi } from 'wookashwackomytest-polkahat-network-helpers';
import TAccessControlContract from 'typechain/contracts/t_access_control';
import TAccessControlDeployer from 'typechain/deployers/t_access_control';
import { shouldBehaveLikeAccessControl } from 'wookashwackomytest-pendzl-tests';
import { getSigners } from 'wookashwackomytest-polkahat-network-helpers';
import 'wookashwackomytest-polkahat-chai-matchers';

const [defaultAdmin, ...others] = getSigners();

describe('AccessControl', () => {
  let api: ApiPromise;
  let accessControlMock: TAccessControlContract;
  beforeEach(async () => {
    api = await localApi.get();
    accessControlMock = (await new TAccessControlDeployer(api, defaultAdmin).new()).contract;
  });

  shouldBehaveLikeAccessControl(() => ({
    mock: accessControlMock,
    accounts: others,
    defaultAdmin,
  }));
});
