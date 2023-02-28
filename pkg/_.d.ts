/* tslint:disable */
/* eslint-disable */
/**
*/
export class HashId {
  free(): void;
/**
*/
  constructor();
/**
* @param {Uint8Array} key
* @param {number} val
*/
  set(key: Uint8Array, val: number): void;
/**
* @param {Uint8Array} key
* @returns {number | undefined}
*/
  get(key: Uint8Array): number | undefined;
/**
* @returns {Uint8Array}
*/
  dump(): Uint8Array;
/**
* @returns {number}
*/
  maxId(): number;
/**
* @param {Uint8Array} bin
* @returns {HashId}
*/
  static load(bin: Uint8Array): HashId;
}
