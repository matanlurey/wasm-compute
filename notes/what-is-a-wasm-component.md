# What is a WebAssembly Component (And Why)?

[![What is a WebAssembly Component (And Why)](https://img.youtube.com/vi/tAACYA1Mwv4/0.jpg)](https://www.youtube.com/watch?v=tAACYA1Mwv4)

Starting with the basics, [what is WebAssembly](https://webassembly.org/):

> WebAssembly is a binary instruction format for a stack-based virtual
> machine. Wasm is designed as a portable **compilation target** for programming
> languages, enabling deployment on the web for client and server applications.

For example, we can take our favorite programming languages (e.g.
[Rust][rust-wasm], [C/C++][c-wasm], [Go][go-wasm], [Swift][swift-wasm],
[Dart][dart-wasm]) and compile them to WebAssembly, just like we would compile
then to x86, ARM, or other native architectures, and run them either in a
modern web browser or a WASM server or application runtime.

[rust-wasm]: https://www.rust-lang.org/what/wasm
[c-wasm]: https://emscripten.org/
[go-wasm]: https://go.dev/blog/wasi
[swift-wasm]: https://swiftwasm.org/
[dart-wasm]: https://docs.flutter.dev/platform-integration/web/wasm

This buys us:

- **Portability**: run the same code on the web, server, or edge
- **Determinism**: consistent performance and behavior across platforms
- **Control-flow integrity**: execution guarantees, preventing some jump attacks
- **Sub-process sandboxing**: mitigates some security risks, crashes, and bugs

Popular use cases today include:

- **Port a large C/C++ codebase to the web**: [Unity][unity-wasm],
  [AutoCAD][auto-cad-wasm], [Photoshop][photoshop-wasm], [Earth][earth-wasm],
  and [Figma][figma-wasm].
- **Compute-intensive tasks**: [image processing][image-wasm],
  [machine learning][ml-wasm], [cryptography][crypto-wasm], and more.
- **Embed WASM in an existing system**: [game engines][game-wasm],
  [CDNs][cdns-wasm], [desktop plugins][plugins-wasm].
- **Alternative distributed computing**: [serverless][serverless-wasm],
  [distributed actors][actors-wasm], and [record/replay][record-replay-wasm].

> [!NOTE]
> Fastly is mostly focused on the last two categories.

[unity-wasm]: https://blog.unity.com/engine-platform/webassembly-is-here
[auto-cad-wasm]: https://madewithwebassembly.com/showcase/autocad/
[photoshop-wasm]: https://web.dev/articles/ps-on-the-web
[earth-wasm]: https://earth.google.com/web/
[figma-wasm]: https://www.figma.com/blog/webassembly-cut-figmas-load-time-by-3x/

[image-wasm]: https://github.com/silvia-odwyer/photon
[ml-wasm]: https://blog.tensorflow.org/2020/03/introducing-webassembly-backend-for-tensorflow-js.html
[crypto-wasm]: https://github.com/originjs/crypto-js-wasm

[game-wasm]: https://wasm4.org/
[cdns-wasm]: https://www.fastly.com/blog/how-fastly-and-developer-community-invest-in-webassembly-ecosystem
[plugins-wasm]: https://www.logicalcube.com/development/webassembly-wasi-plugins/

[serverless-wasm]: https://docs.fastly.com/products/compute
[actors-wasm]: https://wasmcloud.com/docs/concepts/actors
[record-replay-wasm]: https://www.bartoszsypytkowski.com/wasm-replayable-functions/

## Growing WASM usage by 4x

There are 4 different use cases that Luke talks about in the video.

### "SDKs for Free"

On most compute platforms, the _client_ controls the application code, and the
_platform_ controls the interface, the language runtime (choices of language),
and SDKs to use the platform.

This is both **lots of work to maintain** and **limits the choices** of the
client, but can be addressed by using WASM:

Area        | Need      | Limitation | Solution
----------- | --------- | ---------- | --------
Application | Language  | Choice     | Any language that compiles to WASM.
Platform    | Interface | Control    | WASM interface, shared/generated SDKs.

... and **this doesn't need Open API or gRPC**, just a WASM interface.

### "Secure polyglot packages"

To re-use code, there are bunch of existing languages, each with their own
ecosystems and registries. For example:

- [Rust](https://crates.io/)
- [Python](https://pypi.org/)
- [JavaScript](https://www.npmjs.com/)
- [Go](https://pkg.go.dev/)

When building an application that uses multiple languages, not only is hard to
use code across languages, all of those packages go into the same memory space
(can collide), and share the same capabilities (secrets, filesystems, services),
even if a particular package doesn't need that particular capability.

WASM can have a _language neutral_ _sandboxed_ OCI registry[^1], with packages
sandboxed in their own memory/capabilities:

- Better reuse of code from any language
- Mitigates supply-chain attacks
- Helps emerging languages (no need to boostrap a package registry)

[^1]: A few more details at <https://radu-matei.com/blog/wasm-to-oci/>.

### "Modularity without microservices"

To avoid the "big ball of mud" architecture, typically microservices are used,
with HTTP as the communication protocol. This has a high cost in terms of both
complexity and performance overhead, but does buy the ability to independently
scale individual services.

_... but what if we don't need to scale specific services?_

The typical answer is the "modular monoloith", where the application is broken
into modules, but still runs in a single process. However, this requires a
single language (or language family), and less isolation (shared global state).

Again, using WASM, we gain:

- Language neutrality
- Sandbox support
- Linkable modules

... combining the benefits of microservices and modular monoliths.

### "Virtual Platform Layering"

Imagine we have 4 teams, each owning their own service, and we want to refactor
out some common code (authentication, RPC, observability, etc.) to be owned by
a platform team.

We could create an _embedded shared library_, but it's:

- Harder to update: need to update all services, deal with release schedules.
- Harder to maintain: need to deal with different languages, versions, etc.
- Leakier abstraction: shared state, shared capabilities, shared bugs.

... or use the [sidecar proxy pattern][sidecar-proxy] (networked shared code):

- Significant runtime overhead

[sidecar-proxy]: https://iximiuz.com/en/posts/service-proxy-pod-sidecar-oh-my/

... or use WASM and it's _sandbox support_ and _linkable modules_:

- Separate teams write code in their own language, compile to WASM.
- Platform team writes the shared code in their own language, compiles to WASM.
- All code runs in the same process, but in separate sandboxes.

... and wherever it is deployed knows _nothing_ about the above, except WASM:

- Low-overhead calls between modules
- Strong encapsulation between modules

## Supporting these use cases

build | what we're building      | comparable to
----- | ------------------------ | -------------
from  | a new **compile target** | x86, ARM, RISC-V, ...
to    | a new **ecosystem**      | containers, npm, nix, maven, helm, ...

### What makes an ecosystem?

The idea is to upgrade from using a _container_ as the unit of distribution to
using a _WASM component_:

> An (emerging) standard portable lightweight finely-sandboxed cross-language
> compositional module.

Part of the ecosystem       | Containers                       | WASM Component
--------------------------- | -------------------------------- | --------------
Dstributable format         | Docker                           | OCI
Tools to build from sources | `docker build`                   | Compilers
Tools to deploy/run         | `docker run`, kubernetes         | WASI, WASI host
Tools to share and compose  | `docker push`, `docker compose`  | WASM registry

A component has:

- **Imports**: I/O performed, impl dependencies; _not_ a fixed global namespace
- **Internal definitions**: Embedded WASM modules (99% of the bytes)
- **Exports**: Host events/triggers, how the client code runs; not `main()`

So the _type_ of a component is just its _names_ and _types_ of imports/exports:

```txt
world my-plugin {
  import log: func(msg: string)
  import get: func(key: string) -> string

  export tick: func(delta: f64)
  export render: func() -> list<pixel>
}
```

The IDL for this component model is called [WIT][wit], with a _world_ defining
a contract between guests and hosts; guest compoinents _target_ a world (call
imports, implement exports) and host components _support_ a world (implement
imports, call exports). For example:

```txt
interface wasi:filesystem {
  read: func(f: file, o: u64, n: u64) -> list<u8>
  write: func(f: file, o: u64, b: list<u8>)
}

interface wasi:http/handler {
  handle: func(r: request) -> response
}

world my-plugin {
  import log:func(msg:string)
  import wasi:filesystem

  export wasi:http-handler
  export render func() -> list<pixel>
}
```

> _WASI's job is to standardize common interaces for use by **many** worlds,
> **not** to specify a single fixed world_

[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

## Composing WASM components

Imagine we have an old-school C-program that generates a thumbnail (`thumb.c`),
and we want to create a web service.

Using `wasi-sdk`, we can compile `thumb.c` -> `thumb-posix` with
`wasi:filesystem` and `wasi:cli/main`. But what we _want_ is a pure component
that takes an input stream of bytes and returns an output stream of bytes:

```txt
thumbsify: func(stream) -> stream
```

There is a new tool in the [bytecode alliance][] called `wasi-virt`, which
adapts the exports to take a stream and return a stream, and the imports to
provide a stream and consume a stream, and we now have a reusable component,
and can even publish it using `warg publish`.

[bytecode alliance]: https://bytecodealliance.org/

Now this component can be used from another language, perhaps JavaScript
(`thumb-api.js`), and can even compile _that_ into a WASM component (using
`compenentize-js`) and publish it as `thumb-api`.

Finally, using `wasm-compose`, we can compose both of the components above that
targets a very simple world, that can be run in a large variety of environments:

```txt
world caching-server {
  import wasi:keyvalue/cache
  export wasi:http/handler
}
```

Imagine we wanted to test this component locally, where we might just have
filesystems and sockets, and not a database or cloud storage, i.e. the "command"
world:

```txt
world command {
  import wasi:filesystem/types
  import wasi:sockets/*
  export wasi:cli/main
}
```

To run it, we'll compose it _again_ using `wasm compose` with `fs-cache` and
`http-server` that speaks raw sockets.

**This whole model is _compositional_**: we have **two** implementations of
`wasi:filesystem` in this one composite (the outer one, talking to the native
system, and the inner one, entirely virtualized) - _virtual platform layering_.

> Luke goes on to say it leads to WASI could stand for
> "WebAssembly **Standard** Interfaces", not "System Interface".

## Creating a WASM component

For example, given the following world:

```txt
world my-world {
  import log: func(msg: string)
  export run: func(args: list<string>)
}
```

If we wanted to implement it using JavaScript, we could write:

```js
import log from 'log';

export function run(args) {
  log(args.join(' '));
}
```

... and use `compenentize-js` to target the `my-world` world.

In Python, we could use a Pythom import and `componentize-py`:

```python
from my_world import log

class MyWorld:
  def run(args: list[str]):
    log(' '.join(args))
```

Or in Rust, we could use `cargo component`:

```rust
impl MyWorld for Component {
  fn run(&self, args: Vec<String>) {
    log(args.join(" "));
  }
}
```

> Most of these tools share underlying code like `wit-bindgen` and `wasm-tools`.

Value types that are shared between languages are defined in the `wit` IDL,
including:

- Characters
- Numbers
- Strings
- Lists
- Records
- Tuples
- Flags
- Variants
- Enums
- Options
- Results

And _resource_ types (passed by handle):

```txt
world my-world {
  import buffer::resource {
    constructor(bytes: list<u8>)
    append: func(bytes: list<u8>)
  }

  export build: func() -> buffer
}
```

```js
import Buffer from 'buffer';

export function build() {
  const b = new Buffer([1, 2, 3]);
  b.append([1, 2, 3]);
  return b;
}
```

### Sandboxing

Each component has it's own memory and tables. For example:

```txt
world secret-store-impl {
  import wasi:http/handler
  export secret-store
}

interface secret-store {
  resource secret {
    expose: func() -> string
  }

  get: func(name: string) -> secret
}

world secret-store-client {
  import secret-store
  export run:func()
}
```

An example in Rust:

```rust
impl SecretStore for Component {
  fn get(&self, name: String) -> Secret {
    /* ... */
  }
}

impl Secret for SecretImpl {
  fn expose(&self) -> String {
    /* ... */
  }
}
```

Using it from JavaScript:

```js
import { get } from 'secret-store';

export function run() {
  const secret = get('my-secret');
  const exposed = secret.expose();
  // ...
}
```

The component model has an adapter between the modules, and copies data by
values between linear memory, or sticks an address into an opaque-to-client
_resource_ handle, and returns that. Linear memory is _not_ shared between
components, and the host can _not_ access the guest's memory.

### Lightweight

Two phases of running WASM:

- **Ahead-of-time**: or **control plane**; compile, link, optimize, validate.
- **Run-time**: or **data plane**; instantiate, run, tear down.

A component works similarly:

- **Ahead-of-time**: fuse the modules together.
- **Run-time**: virtually no changes; execution platform can optionally contain
  a cache of shared modules.

### Portable

**"WebAssembly Core"** runs in browsers today, and has:

- A formal spec
- A reference interpreter
- A test suite

... and is exposed to the rest of the browser with a JS API.

**"Component Model"**, which is layered on top of the core, has:

- A formal spec
- A reference interpreter
- A test suite

... and it's own component model JS API, which extends the core JS API.

Components are not supported in the browser natively today, so there is
yet-another tool, `jco transpile`, which takes a component and transpiles it
into a core module and JavaScript glue code, and can run WASM components in the
browser _today_.

> WASI (filesystem, sockets, cli, http) is available to components, and could be
> transpiled into web APIs as well.

## Next steps

> Information as of September 11, 2023

- Developer preview release ("preview 2") of component model and subset of WASI.
- Stability and backwards compatibility as top-line goals.
- New features, including:
  - First wave of languages: Rust, JS, Python, Go, C
  - First wave of WASI proposals: filesystems, sockets, cli, http
  - Browser/node polyfill: `jco transpile`
  - Preliminary support for WASI virtualization: `wasi-virt`
  - Preliminary support for component composition: `wasm-compose`
  - Experimental component registry tooling: `warg`

> <https://bytecodealliance.org/articles/webassembly-the-updated-roadmap-for-developers>

And next year (i.e. this year, 2024):

- Improve concurrency story (automatic bindings for async interfaces)
- Better streaming performance
- Add native `future` and `stream` types to WIT+components

For example, moving from preview 2 to preview 3 in terms of HTTP:

```diff
- interface wasi:http/outgoing-handler {
-   handle: func(r: outgoing-request) -> incoming-response
- }
- interface wasi:http/incoming-handler {
-   handle: func(r: incoming-request) -> outgoing-response
- }
+ interface wasi:http/handler {
+   handle: func(r: request) -> response
+ }
```

... and have a single component that imports/exports the same interface.
