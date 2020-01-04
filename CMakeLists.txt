cmake_minimum_required(VERSION 3.9)

project(uattool)


add_executable(uattool main.cpp)

# Link NLohmann's JSON library
find_package(nlohmann_json REQUIRED)
target_link_libraries(uattool PRIVATE nlohmann_json::nlohmann_json)