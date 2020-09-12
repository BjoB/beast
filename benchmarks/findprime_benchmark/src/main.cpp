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
    if (counter == 0) {
        return true;
    } else {
        return false;
    }
}

int calcNthPrime() {
    int n = 2000, num = 1;
    int count = 0;

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
        calcNthPrime();
    }
}

BENCHMARK(calcNthPrimeBenchmark);

BENCHMARK_MAIN();
