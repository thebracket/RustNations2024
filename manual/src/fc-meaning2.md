# Fearless Concurrency

Threading is **hard**. You divide your problem up into chunks, have as many CPU cores as you can solve your problem, recombine the results, and hopefully get the right answer.

As you'll see in a moment, it's far from a bed of roses. You have to worry about:

* Is your problem big enough that spawning threads doesn't actually slow it down?
* Is your problem naturally parallelizable?
* If I share data between threads, will that data still work?
* If threads are communicating with other threads, when can I safely clean up my memory?

And more...

Rust has a comprehensive answer to fearless concurrency. It still isn't *easy*, but it's a lot less likely to eat your data and make your life miserable!

* **Data Race Protection**. You can't compile a data-race in Safe Rust (and if you can, the core team want to hear from you)
* **No Use After Free**. Your program won't compile if you destroy something at the wrong time.
* **Easy Thread Semantics**. Rust makes it easy to use threads, so you're less afraid to try.

So should you shout Leeroy Jenkins and charge? Probably not - but Rust makes it a lot easier to try, and quickly iterate into a safe strategy.