import { ApiPromise } from '@polkadot/api';
import { localApi } from 'wookashwackomytest-polkahat-network-helpers';
import TAccessControlContract from 'typechain/contracts/t_access_control';
import TAccessControlDeployer from 'typechain/deployers/t_access_control';
import {
  AccessControlError,
  AccessControlInternal,
  shouldBehaveLikeAccessControl,
  shouldBehaveLikeAccessControlInternal,
} from 'wookashwackomytest-pendzl-tests';
import { getSigners } from 'wookashwackomytest-polkahat-network-helpers';
import 'wookashwackomytest-polkahat-chai-matchers';
import type { KeyringPair } from '@polkadot/keyring/types';
import { expect } from 'chai';

const [defaultAdmin, ...others] = getSigners();

export const DEFAULT_ADMIN_ROLE = 0;
export const ROLE = 1;
export const OTHER_ROLE = 2;

describe('AccessControl', () => {
  let api: ApiPromise;
  let accessControlMock: TAccessControlContract;
  beforeEach(async () => {
    api = await localApi.get();
    accessControlMock = (await new TAccessControlDeployer(api, defaultAdmin).new()).contract;
  });

  shouldBehaveLikeAccessControl(() => ({
    contract: accessControlMock,
    accounts: others,
    defaultAdmin,
  }));

  shouldBehaveLikeAccessControlInternal(() => ({
    contract: accessControlMock,
    accounts: others,
    defaultAdmin,
  }));
});
