# "Why do this?"

Synchronous Rust is really nice to write, really powerful. If your organization is adopting Rust, they probably want power and safety. At the same time, you probably need an I/O interface to drive your service.

One very powerful pattern is to have a small async messaging core, linked to a really powerful threaded calculating monster. I've given you the building blocks for this:

1. Async receives instructions.
2. Async passes the workload in via a channel, possibly passing an async "oneshot" channel to receive a reply.
3. The threaded core handles the workload.
4. The reply is sent back into async world (which has been happily awaiting a response and doing other things).
5. The async system can handle the network chatter.

Another pattern is to `spawn_blocking` for every threaded task. That can be fine, but you have to control the size of your blocking pool. Like most things in engineering, it depends what you need to do.