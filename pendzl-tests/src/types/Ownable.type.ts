import type { Abi, ContractPromise } from "@polkadot/api-contract";
import type { ApiPromise } from "@polkadot/api";
import type {
  EventDataTypeDescriptions,
  Result,
  SignAndSendSuccessResponse,
} from "@c-forge/typechain-types";
import type { QueryReturnType } from "@c-forge/typechain-types";
import type { KeyringPair } from "@polkadot/keyring/types";
import { AccountId, LangError } from "@c-forge/polkahat-chai-matchers";
import { ContractOptions } from "@polkadot/api-contract/types";

export enum OwnableError {
  CallerIsNotOwner = "CallerIsNotOwner",
  ActionRedundant = "ActionRedundant",
}

interface OwnableQuery {
  owner(
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<AccountId | null, LangError>>>;
  renounceOwnership(
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, OwnableError>, LangError>>>;
  transferOwnership(
    newOwner: AccountId,
    __options?: ContractOptions
  ): Promise<QueryReturnType<Result<Result<null, OwnableError>, LangError>>>;
}

interface OwnableTx {
  owner(__options?: ContractOptions): Promise<SignAndSendSuccessResponse>;
  renounceOwnership(
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
  transferOwnership(
    newOwner: AccountId,
    __options?: ContractOptions
  ): Promise<SignAndSendSuccessResponse>;
}

export interface Ownable {
  readonly query: OwnableQuery;
  readonly tx: OwnableTx;
  readonly nativeContract: ContractPromise;
  readonly address: string;
  readonly nativeAPI: ApiPromise;
  readonly contractAbi: Abi;
  readonly eventDataTypeDescriptions: EventDataTypeDescriptions;
  withSigner: (signer: KeyringPair) => Ownable;
  withAddress: (address: string) => Ownable;
  withAPI: (api: ApiPromise) => Ownable;
}
