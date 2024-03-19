# Is this fast?

(Demo on my PC)

Let's run the server in release mode: `cargo run --release`.

Now pull up `htop` and search for axum.

Now we'll use `ali` to hit it.

`ali --rate 100 http://localhost:3001` shows that 100 hits per second has negligible impact.
`ali --rate 1000 http://localhost:3001` on my workstation at the office shows that we're starting to hit the CPU a little bit.
`ali --rate 5000 http://localhost:3001` on my workstation shows that we're starting to see some latency variance and hitting the CPU pretty hard.

We haven't seen any non 200 results, yet!

## Try it again single threaded

So with just 1 thread...

`ali --rate 100 http://localhost:3001` shows that 100 hits per second has negligible impact.
`ali --rate 1000 http://localhost:3001` shows that 1,000 hits per second is causing the occasional latency spike.
`ali --rate 5000 http://localhost:3001` on my workstation shows latency really starting to spike sometimes. We still haven't had any errors!
`ali --rate 10000 http://localhost:3001` on my workstation shows latency really starting to hurt sometimes. We've started to see the occasional error, too.

> Not too bad for 159 lines of code.