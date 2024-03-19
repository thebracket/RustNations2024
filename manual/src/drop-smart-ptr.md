# Old School Smart Pointers

Let's channel some early 1990s C++ and put `Drop` to work!

```rust
use std::alloc::{alloc, dealloc, Layout};

struct Memory {
    raw_memory: *const u8,
    layout: Layout,
}

impl Memory {
    fn new() -> Self {
        println!("Allocating buffer");
        let (layout, ptr) = unsafe {
            let layout = Layout::array::<u8>(4096).unwrap();
            let ptr = alloc(layout);
            (layout, ptr)
        };

        Self {
            raw_memory: ptr,
            layout,
        }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        println!("Deallocating buffer");
        unsafe {
            dealloc(self.raw_memory as *mut u8, self.layout);
        }
    }
}

fn main() {
    let blob = Memory::new();
}

```

Congratulations! You've built your very own implemenation of `Box`. Admittedly, it's very much a `wish.com Box` since it's missing features like actually using the data - but those few lines of code are enough to ensure you allocate, and deallocate without leaking memory.

> This was one of the first uses of destructors. Since you had to make everything with `new`, you could `delete` them all in the destructor. Sometimes, even when they were null...