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