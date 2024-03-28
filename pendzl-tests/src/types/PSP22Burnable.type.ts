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

export interface PSP22BurnableQuery {
  burn(
    from: AccountId,
    amount: string | number | BN,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
}

export interface PSP22BurnableTx {
  burn(
    from: AccountId,
    amount: string | number | BN,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
}

export interface PSP22Burnable {
  readonly query: PSP22BurnableQuery;
  readonly tx: PSP22BurnableTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => PSP22Burnable;
  withAddress: (address: string) => PSP22Burnable;
  withAPI: (api: ApiPromise) => PSP22Burnable;
}
