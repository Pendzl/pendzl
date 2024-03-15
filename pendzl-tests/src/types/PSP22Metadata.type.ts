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
  PSP22Error,
} from "wookashwackomytest-polkahat-chai-matchers";

export interface PSP22MetadataQuery {
  tokenName(
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<string | null, LangError>>>;
  tokenSymbol(
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<string | null, LangError>>>;
  tokenDecimals(
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<BN, LangError>>>;
}

export interface PSP22MetadataTx {
  tokenName(__options?: GasLimit): Promise<SignAndSendSuccessResponse>;
  tokenSymbol(__options?: GasLimit): Promise<SignAndSendSuccessResponse>;
  tokenDecimals(__options?: GasLimit): Promise<SignAndSendSuccessResponse>;
}

export interface PSP22Metadata {
  readonly query: PSP22MetadataQuery;
  readonly tx: PSP22MetadataTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => PSP22Metadata;
  withAddress: (address: string) => PSP22Metadata;
  withAPI: (api: ApiPromise) => PSP22Metadata;
}
