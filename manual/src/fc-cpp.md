# Fearful Concurrency with C++

> I know it's fashionable to beat up on C++, but it's really not a bad language! Unfortunately, it's really about *n* languages---where *n* is the number of organizations using it. You *can* write safe C++!

Let's start with an example of the problem Rust is seeking to solve. Let's write some modern C++:

```cpp
#include <thread>
#include <iostream>
#include <vector>
#include <thread>

const int N_THREADS = 100;
const int N_LOOPS = 10000;

int main() {
    int counter = 0;

    std::vector<std::thread> handles;

    for (int i = 0; i < N_THREADS; ++i) {
        handles.push_back(std::thread([&counter]() {
            for (int i = 0; i < N_LOOPS; ++i) {
                ++counter;
            }
        }));
    }

    for (auto& handle : handles) {
        handle.join();
    }

    std::cout << counter << std::endl;

    return 0;
}
```

And now let's make a `CMakeLists.txt` file to actually compile it. We're going to put warnings up to the max. Safety first, right?

```cmake
cmake_minimum_required(VERSION 3.5)
project(TypeChanges)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

if (MSVC)
    add_compile_options(/W4 /WX)
else()
    add_compile_options(-Wall -Wextra -pedantic -Werror)
endif()

#set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")
add_executable(data_race data_race.cpp)
```

Now let's build our program. It's easy... `cargo run`. No wait...

```bash
mkdir bin
cd bin
cmake ..
make
```

> Another often-overlooked superpower with Rust is Cargo and RustUp. Compiling this on my Mac required that I install the C++ toolchain, CMake --- which I then updated, and Make. Be glad I didn't also write some unit tests and integrate source control!

So now that we've built it, let's *run the program*...

```bash
./data_race
> 423724
```

Looks great! Let's run it again!

```bash
./data_race 
> 786657
```

Oh. I gave the game away with the program name - but Houston, we have a problem!