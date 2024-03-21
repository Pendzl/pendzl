# Summary

This project contains test suites to facilitate testing smart contracts behaviors, meaning to help verify they comply with standards.
The tests are based on OpenZeppelin tests that can be found here (https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/test)[https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/test].

Currently the following test suites are implemented:

- `shouldBehaveLikeAccessControl`
  - This suite verifies the behavior of a contract implementing AccessControl trait.
- `shouldBehaveLikeOwnable`
  - This suite verifies the behavior of a contract implementing Ownable trait.
- `shouldBehaveLikePSP22`
  - This suite verifies the behavior of a contract implementing PSP22 trait.
  - provides `shouldBehaveLikePSP22Transfer` and `shouldBehaveLikePSP22Approve` suites for more granular testing.
- `shouldBehaveLikePSP34`
  - This suite verifies the behavior of a contract implementing PSP34 trait.
  - provides `shouldBehaveLikePSP34Transfer` suites for more granular testing.

## Requirements

These suites require typechain generated classes to be provided as they rely on their way of interacting with the contracts.

# Usage

To use the test suites, you can import them in your test file and use them in your test cases. For example:

```typescript
import { ApiPromise } from "@polkadot/api";
import { localApi } from "wookashwackomytest-polkahat-network-helpers";
import MyAccessControlContract from "typechain/contracts/my_access_control";
import MyAccessControlDeployer from "typechain/deployers/my_access_control";
import { shouldBehaveLikeAccessControl } from "wookashwackomytest-pendzl-tests";
import { getSigners } from "wookashwackomytest-polkahat-network-helpers";
import "wookashwackomytest-polkahat-chai-matchers";

const [defaultAdmin, ...others] = getSigners();

describe("AccessControl", () => {
  let api: ApiPromise;
  let myAccessControlContract: MyAccessControlContract;
  beforeEach(async () => {
    api = await localApi.get();
    myAccessControlContract = (
      await new MyAccessControlDeployer(api, defaultAdmin).new()
    ).contract;
  });

  shouldBehaveLikeAccessControl(() => ({
    contract: myAccessControlContract,
    accounts: others,
    defaultAdmin,
  }));

  //...rest of the tests for the contract (your tests, other suites etc...)
});
```

# Types

This package provides a set of interfaces (types) that can be used to write test for smart contracts, as well as interacting with them in scripts/programs using Typescript. Each interface corresponds with a `pendzl` trait.
The types following types are provided:

- `AccessControl`
- `Ownable`
- `PSP22`
- `PSP22Burnable`
- `PSP22Mintable`
- `PSP22Metadata`
- `PSP34`
- `PSP34Metadata`
