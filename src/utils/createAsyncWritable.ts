import { writable, readable, derived, type Writable } from 'svelte/store';

export const createAsyncWritable = <T>(storeItem: Writable<T>, asyncMethod: () => Promise<any>, initialValue: T) => {
  return derived<Writable<T>, T>(storeItem, (_, set) => {
    Promise.resolve(asyncMethod()).then((result) => {
      console.log('>>>>', result);
      set(result);
    }).catch(err => {
      console.log(err);
    })

  }, initialValue);
}