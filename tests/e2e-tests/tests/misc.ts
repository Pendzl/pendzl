import { ApiPromise } from '@polkadot/api';
import { getSigners } from 'tests/setup/helpers';
import MyPsp22Deployer from 'typechain/deployers/my_psp22';

export async function getApiAt(api: ApiPromise, blockNumber: number) {
  const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
  const apiAt = await api.at(blockHash);
  return apiAt;
}

export async function setBlockTimestamp(api: ApiPromise, timestamp: number) {
  const signer = getSigners()[0];
  if (process.env.DEBUG) console.log(`setting timestamp to: ${timestamp}`);
  await api.tx.timestamp.setTime(timestamp).signAndSend(signer, {});
  await transferNoop(api);
  const timestampNowPostChange = parseInt((await api.query.timestamp.now()).toString());
  if (timestampNowPostChange !== timestamp) throw new Error('Failed to set custom timestamp');
}
export async function increaseBlockTimestamp(api: ApiPromise, deltaTimestamp: number): Promise<number> {
  const timestampNow = await api.query.timestamp.now();
  const timestampToSet = parseInt(timestampNow.toString()) + deltaTimestamp;
  if (process.env.DEBUG) console.log(`increasing timestamp by ${deltaTimestamp}`);
  await setBlockTimestamp(api, timestampToSet);
  const timestampNowPostChange = parseInt((await api.query.timestamp.now()).toString());
  if (timestampNowPostChange !== timestampToSet) throw new Error('Failed to set custom timestamp');
  return timestampToSet;
}

/// makes an operation just to force new block production.
export async function transferNoop(api: ApiPromise) {
  const signer = getSigners()[0];
  await new MyPsp22Deployer(api, signer).new(0); //TODO
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
