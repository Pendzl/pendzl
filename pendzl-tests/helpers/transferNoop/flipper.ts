import { CodePromise } from "@polkadot/api-contract";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { ApiPromise } from "@polkadot/api";
import {
  _genValidGasLimitAndValue,
  _signAndSend,
  SignAndSendSuccessResponse,
} from "wookashwackomytest-typechain-types";
import type { ConstructorOptions } from "wookashwackomytest-typechain-types";
import type { WeightV2 } from "@polkadot/types/interfaces";
import FsAPI from "fs";
import PathAPI from "path";

const fileName = "flipper";

export default class FlipperDeployer {
  readonly nativeAPI: ApiPromise;
  readonly signer: KeyringPair;

  constructor(nativeAPI: ApiPromise, signer: KeyringPair) {
    this.nativeAPI = nativeAPI;
    this.signer = signer;
  }

  /**
   * new
   *
   */
  async new(__options?: ConstructorOptions) {
    const abi = JSON.parse(
      FsAPI.readFileSync(
        PathAPI.resolve(__dirname, `${fileName}.json`)
      ).toString()
    );

    const wasm = FsAPI.readFileSync(
      PathAPI.resolve(__dirname, `${fileName}.wasm`)
    );
    const codePromise = new CodePromise(this.nativeAPI, abi, wasm);
    const gasLimit = (
      await _genValidGasLimitAndValue(this.nativeAPI, __options)
    ).gasLimit as WeightV2;

    const storageDepositLimit = __options?.storageDepositLimit;
    const tx = codePromise.tx["default"]!({
      gasLimit,
      storageDepositLimit,
      value: __options?.value,
    });
    let response;

    try {
      response = await _signAndSend(
        this.nativeAPI.registry,
        tx,
        this.signer,
        (event: any) => event
      );
    } catch (error) {
      console.log(error);
    }
    return {
      result: response as SignAndSendSuccessResponse,
    };
  }
}
