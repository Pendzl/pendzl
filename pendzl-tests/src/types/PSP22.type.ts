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

export interface PSP22Query {
  totalSupply(
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<BN, LangError>>>;
  balanceOf(
    owner: AccountId,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<BN, LangError>>>;
  allowance(
    owner: AccountId,
    spender: AccountId,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<BN, LangError>>>;
  transfer(
    to: AccountId,
    value: string | number | BN,
    data: Array<string | number | BN>,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  transferFrom(
    from: AccountId,
    to: AccountId,
    value: string | number | BN,
    data: Array<string | number | BN>,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  approve(
    spender: AccountId,
    value: string | number | BN,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  increaseAllowance(
    spender: AccountId,
    deltaValue: string | number | BN,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
  decreaseAllowance(
    spender: AccountId,
    deltaValue: string | number | BN,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, PSP22Error>, LangError>>>;
}

export interface PSP22Tx {
  totalSupply(__options?: ContractOptions): Promise<SignAndSendSuccessResponse>;
  balanceOf(
    owner: AccountId,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  allowance(
    owner: AccountId,
    spender: AccountId,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  transfer(
    to: AccountId,
    value: string | number | BN,
    data: Array<string | number | BN>,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  transferFrom(
    from: AccountId,
    to: AccountId,
    value: string | number | BN,
    data: Array<string | number | BN>,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  approve(
    spender: AccountId,
    value: string | number | BN,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  increaseAllowance(
    spender: AccountId,
    deltaValue: string | number | BN,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  decreaseAllowance(
    spender: AccountId,
    deltaValue: string | number | BN,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
}

export interface PSP22 {
  readonly query: PSP22Query;
  readonly tx: PSP22Tx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => PSP22;
  withAddress: (address: string) => PSP22;
  withAPI: (api: ApiPromise) => PSP22;
}
