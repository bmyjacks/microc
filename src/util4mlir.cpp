#include <iostream>

extern "C" {
    int read() {
        int value;
        std::cin >> value;
        return value;
    }

    void print(int value) {
        std::cout << value << std::endl;
    }
}