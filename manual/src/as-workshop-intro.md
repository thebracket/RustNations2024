# Async Workshop

We've talked about the basics of how async works, and the easiest way to shoot yourself in the foot with it (CPU heavy blocking tasks). We've griped about how early it is in development, and noticed that it performs pretty well. We talked about how great it is for I/O tasks.

So now lets put the rubber to the road and collectively build a useful Rust REST server.

Our objectives:

* Create a database. We'll use SQLite, so it runs locally without us all having to setup Postgres.
* Use `sqlx` to define our database and set it up for us on start-up.
* Create a database layer to manipulate our data.
* Use `Axum` to create a "hello world" server (to get us familiar with Axum).
* We'll create "get all", "get one", "delete 1", and "update 1" endpoints. Along the way, you'll learn how Tower --- part of the Axum stack --- makes dependency injection easy.
* We'll use `reqwest` to build a client to call them (so we don't also have to learn HTML/JavaScript)
* Then we'll pound the living daylights out of it and see just how much service performance you can get out of a Macbook Air M1 (or whatever you brought!).
* If time is going well, we'll also add a cache.

So when you're done, you'll have a remarkably resilient web service!

Let's add some more requirements:

* We want nice clean code.
* We don't want a very big program at all, and don't want to writes tons of boilerplate.