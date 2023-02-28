#!/usr/bin/env coffee

> ./pkg/_ > HashId

m = new HashId

m.set(
  new Uint8Array(1)
  2
)

console.log m.dump()

m = HashId.load m.dump()

console.log(
  m.get(
    new Uint8Array(1)
  )
)
