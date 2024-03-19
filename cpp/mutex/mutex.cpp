#include <iostream>
#include <thread>
#include <mutex>

int main() {
    std::mutex mutex;
    int counter = 0;
    std::thread t1([&counter, &mutex]() {
        for (int i = 0; i < 1000000; ++i) {
            std::lock_guard<std::mutex> guard(mutex);
            ++counter;
        }
    });
    std::thread t2([&counter, &mutex]() {
        for (int i = 0; i < 1000000; ++i) {
            std::lock_guard<std::mutex> guard(mutex);
            ++counter;
        }
    });
    t1.join();
    t2.join();

    std::cout << counter << std::endl;

    return 0;
}