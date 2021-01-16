#include <benchmark/benchmark.h>
#include <stdlib.h>

#include <iostream>

using namespace std;

bool isPrime(int number) {
    int counter = 0;
    for (int j = 2; j < number; j++) {
        if (number % j == 0) {
            counter = 1;
            break;
        }
    }
    return (counter == 0 ? true : false);
}

int calcNthPrime() {
    int n = N_VALUE;
    int num = 1, count = 0;

    while (true) {
        num++;
        if (isPrime(num)) {
            count++;
        }
        if (count == n) {
            cout << n << "th prime number is " << num << ".\n";
            break;
        }
    }
    return 0;
}

static void calcNthPrimeBenchmark(benchmark::State& state) {
    for (auto _ : state) {
        for (int i = 0; i < state.range(0); ++i) {
            calcNthPrime();
        }
    }
}

static void calcNthPrimeBenchmark_x2(benchmark::State& state) {
    for (auto _ : state) {
        for (int i = 0; i < state.range(0); ++i) {
            calcNthPrime();
            calcNthPrime();
        }
    }
}

BENCHMARK(calcNthPrimeBenchmark)->Arg(2)->Arg(4)->Arg(6);
BENCHMARK(calcNthPrimeBenchmark_x2)->Arg(2)->Arg(4)->Arg(6);

BENCHMARK_MAIN();
