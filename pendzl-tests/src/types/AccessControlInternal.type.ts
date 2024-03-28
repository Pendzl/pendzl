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
import { AccessControlError } from "./AccessControl.type";
import { ContractOptions } from "@polkadot/api-contract/types";

interface AccessControlInternalQuery {
  tGrantRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
  tRevokeRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
  tEnsureHasRole(
    role: number | string | BN,
    __options?: ContractOptions
  ): Promise<
    QueryReturnType<Result<Result<null, AccessControlError>, LangError>>
  >;
}

interface AccessControlInternalTx {
  tGrantRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  tRevokeRole(
    role: number | string | BN,
    account: AccountId | null,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  tEnsureHasRole(
    role: number | string | BN,
    __options?: ContractOptions
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
