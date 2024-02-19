import { ApiPromise } from "@polkadot/api";
import { getSigners } from "../signers";
import FlipperDeployer from "./flipper";
/// makes an operation just to force new block production.
export async function transferNoop(api: ApiPromise) {
  const signer = getSigners()[0];
  await new FlipperDeployer(api, signer).new(); //TODO
  return;
  await new Promise((resolve, reject) => {
    api.tx.balances
      .transferKeepAlive(signer.address, 1)
      .signAndSend(signer, ({ status }) => {
        if (status.isInBlock) {
          resolve(status.asInBlock.toString());
        }
      })
      .catch((error: any) => {
        reject(error);
      });
  });
}
