import type { Abi, ContractPromise } from "@polkadot/api-contract";
import type { ApiPromise } from "@polkadot/api";
import type {
  EventDataTypeDescriptions,
  Result,
  SignAndSendSuccessResponse,
} from "@c-forge/typechain-types";
import type { QueryReturnType } from "@c-forge/typechain-types";
import type BN from "bn.js";
import type { KeyringPair } from "@polkadot/keyring/types";
import { AccountId, LangError } from "@c-forge/polkahat-chai-matchers";
import { ContractOptions } from "@polkadot/api-contract/types";

export enum AccessControlError {
  invalidCaller = "InvalidCaller",
  missingRole = "MissingRole",
  roleRedundant = "RoleRedundant",
}

interface AccessControlQuery {
  hasRole(
    role: number | string | BN,
    address: AccountId | null,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<boolean, LangError>>>;
  getRoleAdmin(
    role: number | string | BN,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<BN, LangError>>>;
  grantRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
  revokeRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
  renounceRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;

  setRoleAdmin(
    role: number | string | BN,
    newRole: number | string | BN,
    __options?: ContractOptions
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
}

interface AccessControlTx {
  hasRole(
    role: number | string | BN,
    address: AccountId | null,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  getRoleAdmin(
    role: number | string | BN,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  grantRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  revokeRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  renounceRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  setRoleAdmin(
    role: number | string | BN,
    newRole: number | string | BN,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
}

export interface AccessControl {
  readonly query: AccessControlQuery;
  readonly tx: AccessControlTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => AccessControl;
  withAddress: (address: string) => AccessControl;
  withAPI: (api: ApiPromise) => AccessControl;
}
