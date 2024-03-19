# Threaded Execution Model

When you are spawning threads, you are:

* Calling your operating system to create a thread.
* The OS creates the thread and adds it to the system scheduler. It gets its own execution context (with shared access to program memory).
* Threads are then scheduled by the OS. When switching thread:
    * The system scheduler decides that its time to switch.
    * Execution context (stack, execution pointer, register state, etc.) are stored.
    * Execution context for the new thread is restored.
    * The new thread runs until the scheduler decides to switch again.

That's *great* for setups that need to eat CPU like candy. On each CPU core, threads take it in turns. You don't have to write any special code.

And here's the downside:

* Every thread you make requires a system call.
* Every thread you make allocates a stack and execution context.
* The system scheduler won't let you have tens of thousands of threads (my Linux box tells me that it can make 60,000 maximum).
* Once you have a few thousand threads, the system scheduler is chugging a bit.

So for a really busy web service, you probably want to avoid spawning one thread per connection!