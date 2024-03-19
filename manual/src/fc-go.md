# Fearful Concurrency with Go???

Go is a memory safe language that unleashes the concurrent power of your server! Unfortunately, it has the exact same problem:

```go
package main

import (
	"fmt"
	"sync"
)

const LOOP_COUNTER = 10000
const N_THREADS = 100

var counter = 0
var wg sync.WaitGroup

func adder() {
	for i := 0; i < LOOP_COUNTER; i++ {
		counter += 1
	}
	wg.Done()
}

func main() {
	for i := 0; i < N_THREADS; i++ {
		wg.Add(1)
		go adder()
	}
	wg.Wait()
	fmt.Println(counter)
}
```

> Let me take a moment to say that the syntax with wait groups is just horrible! It's also really easy to forget to add `wg.Done()` in the appropriate places.

So feeding this into the Go Playground, and running it:

```
550560
619934
```

Oh. Go didn't help, either.

> In fairness to Go, if you enable data race detection with the `-race` flag, it will warn you about this!