#include <string>
#include "./error.cpp"

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#else
    #include <unistd.h>
    #include <limits.h>
#endif
#include <stdexcept>

namespace env {
    std::string get_cwd() {
        char buffer[1024];

#if defined(_WIN32) || defined(_WIN64)
        if (GetCurrentDirectoryA(sizeof(buffer), buffer)) {
            return {buffer};
        } else {
            throw Error("NotFound", "Could not get current working directory.");
        }
#else
            if (getcwd(buffer, sizeof(buffer)) != NULL) {
                return { buffer };
            } else {
                throw Error("NotFound", "Could not get current working directory.");
            }
#endif
    }

    std::string get_home() {
        char *home = getenv("HOME");
        if (home != nullptr) {
            return {home};
        } else {
            throw Error("NotFound", "Could not get home directory.");
        }
    }
}
