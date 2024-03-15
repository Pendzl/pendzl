import type { Abi, ContractPromise } from "@polkadot/api-contract";
import type { ApiPromise } from "@polkadot/api";
import type {
  EventDataTypeDescriptions,
  GasLimit,
  Result,
  SignAndSendSuccessResponse,
} from "wookashwackomytest-typechain-types";
import type { QueryReturnType } from "wookashwackomytest-typechain-types";
import type BN from "bn.js";
import type { KeyringPair } from "@polkadot/keyring/types";
import {
  AccountId,
  LangError,
} from "wookashwackomytest-polkahat-chai-matchers";
import { AccessControlError } from "./AccessControl.type";

interface AccessControlInternalQuery {
  tGrantRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: GasLimit
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
  tRevokeRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: GasLimit
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
  tEnsureHasRole(
    role: number | string | BN,
    __options?: GasLimit
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
}

interface AccessControlInternalTx {
  tGrantRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  tRevokeRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
  tEnsureHasRole(
    role: number | string | BN,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
}

export interface AccessControlInternal {
  readonly query: AccessControlInternalQuery;
  readonly tx: AccessControlInternalTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => AccessControlInternal;
  withAddress: (address: string) => AccessControlInternal;
  withAPI: (api: ApiPromise) => AccessControlInternal;
}
