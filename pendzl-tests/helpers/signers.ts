import { createTestKeyring } from "@polkadot/keyring/testing";
import type { KeyringPair } from "@polkadot/keyring/types";
export const getSigners = () => {
  return createTestKeyring({ type: "sr25519" }).pairs;
};
export const getSignersWithoutOwner = (
  signers: KeyringPair[],
  ownerIndex: number
) => [...signers.slice(0, ownerIndex), ...signers.slice(ownerIndex + 1)];
export function converSignerToAddress(signer?: KeyringPair | string): string {
  if (!signer) return "";
  return typeof signer !== "string" ? signer.address : signer;
}
