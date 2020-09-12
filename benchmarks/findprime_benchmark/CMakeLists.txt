cmake_minimum_required(VERSION 3.13.5)

project(findprime-benchmark LANGUAGES CXX)

################
# Dependencies #
################

find_package(benchmark REQUIRED)

set(BENCHMARK_ENABLE_TESTING OFF CACHE BOOL "Enable benchmark self tests." FORCE)

#########
# Build #
#########

add_executable(${PROJECT_NAME})

target_sources(${PROJECT_NAME}
    PRIVATE
        "src/main.cpp"
)

target_link_libraries(${PROJECT_NAME}
    PRIVATE
        benchmark::benchmark
)
