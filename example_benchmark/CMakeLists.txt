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

macro(add_findprime_benchmark BENCHMARK_BASE_NAME NUMBER)
    set(EXE_NAME "${BENCHMARK_BASE_NAME}-${NUMBER}")
    add_executable(${EXE_NAME})

    target_sources(${EXE_NAME}
        PRIVATE
            "src/main.cpp"
    )

    target_link_libraries(${EXE_NAME}
        PRIVATE
            benchmark::benchmark
    )

    target_compile_definitions(${EXE_NAME}
        PRIVATE
            N_VALUE=${NUMBER}
    )
endmacro()

add_findprime_benchmark(${PROJECT_NAME} 500)
add_findprime_benchmark(${PROJECT_NAME} 1000)
add_findprime_benchmark(${PROJECT_NAME} 2000)
add_findprime_benchmark(${PROJECT_NAME} 4000)
