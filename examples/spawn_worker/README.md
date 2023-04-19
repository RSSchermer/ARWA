# Custom Events Example

This example contains a minimal ARWA program that demonstrates spawning worker threads.

Run this example locally with [Trunk](https://trunkrs.dev/):

```
$ trunk serve
```

Then visit http://localhost:8080 in a browser to see the result.

Note that this example requires additional compilation flags (see [.cargo/config.toml](.cargo/config.toml)) 
and the server must specify additional cross origin policy headers (see [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements)
and [Trunk.toml](trunk.toml)).
