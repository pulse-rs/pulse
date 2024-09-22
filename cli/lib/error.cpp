#include <string>
#include <utility>

using namespace std;

struct Error : public std::exception {
    std::string kind;
    std::string message;

    Error(std::string k, std::string m) : kind(std::move(k)), message(std::move(m)) {
    }

    const char *what() const noexcept override {
        return (kind + ": " + message).c_str();
    }
};
