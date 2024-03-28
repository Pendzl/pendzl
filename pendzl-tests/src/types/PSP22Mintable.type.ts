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

export interface PSP22MintableQuery {
  mint(
    to: AccountId,
    amount: string | number | BN,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
}

export interface PSP22MintableTx {
  mint(
    to: AccountId,
    amount: string | number | BN,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
}

export interface PSP22Mintable {
  readonly query: PSP22MintableQuery;
  readonly tx: PSP22MintableTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => PSP22Mintable;
  withAddress: (address: string) => PSP22Mintable;
  withAPI: (api: ApiPromise) => PSP22Mintable;
}
