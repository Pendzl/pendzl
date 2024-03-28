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
import {
  AccountId,
  LangError,
  PSP22Error,
} from "@c-forge/polkahat-chai-matchers";
import { ContractOptions } from "@polkadot/api-contract/types";

export interface PSP22MetadataQuery {
  tokenName(
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<string | null, LangError>>>;
  tokenSymbol(
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<string | null, LangError>>>;
  tokenDecimals(
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<BN, LangError>>>;
}

export interface PSP22MetadataTx {
  tokenName(__options?: ContractOptions): Promise<SignAndSendSuccessResponse>;
  tokenSymbol(__options?: ContractOptions): Promise<SignAndSendSuccessResponse>;
  tokenDecimals(
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
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
