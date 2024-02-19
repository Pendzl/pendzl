import type { Abi, ContractPromise } from "@polkadot/api-contract";
import type { ApiPromise } from "@polkadot/api";
import type {
  EventDataTypeDescriptions,
  GasLimit,
  Result,
  ReturnNumber,
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
import { Id } from "./PSP34.type";

export interface PSP34MetadataQuery {
  getAttribute(
    id: Id,
    key: string,
    __options?: GasLimit
  ): Promise<QueryReturnType<Result<string | null, LangError>>>;
}

export interface PSP34MetadataTx {
  getAttribute(
    id: Id,
    key: string,
    __options?: GasLimit
  ): Promise<SignAndSendSuccessResponse>;
}

export interface PSP34Metadata {
  readonly query: PSP34MetadataQuery;
  readonly tx: PSP34MetadataTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => PSP34Metadata;
  withAddress: (address: string) => PSP34Metadata;
  withAPI: (api: ApiPromise) => PSP34Metadata;
}
