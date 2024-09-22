#include <iostream>
using namespace std;


namespace io {
    void println() {
        std::cout << std::endl;
    }

    template <typename T, typename... Args>
    void println(T first, Args... args) {
        std::cout << first << " " << std::boolalpha;
        println(args...);
    }

    void eprintln() {
        std::cerr << std::endl;
    }

    template <typename T, typename... Args>
    void eprintln(T first, Args... args) {
        std::cerr << first << " " << std::boolalpha;
        eprintln(args...);
    }

    void print() {
        std::cout;
    }

    template <typename T, typename... Args>
    void print(T first, Args... args) {
        std::cout << first << " " << std::boolalpha;
        print(args...);
    }

    void eprint() {
        std::cerr;
    }

    template <typename T, typename... Args>
    void eprint(T first, Args... args) {
        std::cerr << first << " " << std::boolalpha;
        eprint(args...);
    }
}
