# Threads and Async, Happily Living Together???

Sometimes, you get the feeling that the regular synchronous Rust teams and the async Rust teams don't talk to one another. It certainly can feel like it on `crates.io`---sometimes crates are synchronous, sometimes they are asynchronous, sometimes both (often with a maze of feature flags).

Fortunately, this isn't true---but there is a tension. Some future ideas may make things a lot easier (such as `maybe_async` discussed at RustConf), syntax changes to make it easier to move data around in an async environment.

There is, however, a tension: Rust is *really* good at CPU intensive tasks, and that's often why you're considering adopting it (although as you saw, it's no slouch at async services).

Sometimes, you just want an async interface---providing web control (REST or gRPC with `tonic`)---and the bulk of the program is pounding the CPUs as hard as possible to handle an intensive workload.

Good news! It's possible!