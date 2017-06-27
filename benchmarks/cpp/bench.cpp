#include <chrono>
#include <algorithm>
#include <random>
#include <iostream>
#include <parallel/algorithm>
#include <tbb/parallel_sort.h>

const int LEN = 100 * 1000 * 1000;

void display(
  std::string name,
  std::chrono::time_point<std::chrono::high_resolution_clock> start,
  std::chrono::time_point<std::chrono::high_resolution_clock> end
) {
  std::cout << name;
  for (auto i = name.size(); i < 30; ++i) {
    std::cout << " ";
  }
  std::cout << std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
  std::cout << " ms" << std::endl;
}

int main() {
  std::vector<uint64_t> v;
  v.resize(LEN);

  auto init = [&] () {
    std::random_device device;
    std::mt19937_64 rnd(device());
    for (int i = 0; i < LEN; ++i) {
      v[i] = rnd();
    }
  };

  {
    init();
    auto start = std::chrono::high_resolution_clock::now();
    std::stable_sort(v.begin(), v.end());
    auto end = std::chrono::high_resolution_clock::now();
    display("std::stable_sort", start, end);
  }
  {
    init();
    auto start = std::chrono::high_resolution_clock::now();
    std::sort(v.begin(), v.end());
    auto end = std::chrono::high_resolution_clock::now();
    display("std::sort", start, end);
  }
  {
    init();
    auto start = std::chrono::high_resolution_clock::now();
    __gnu_parallel::stable_sort(v.begin(), v.end());
    auto end = std::chrono::high_resolution_clock::now();
    display("__gnu_parallel::stable_sort", start, end);
  }
  {
    init();
    auto start = std::chrono::high_resolution_clock::now();
    __gnu_parallel::sort(v.begin(), v.end());
    auto end = std::chrono::high_resolution_clock::now();
    display("__gnu_parallel::sort", start, end);
  }
  {
    init();
    auto start = std::chrono::high_resolution_clock::now();
    tbb::parallel_sort(v.begin(), v.end());
    auto end = std::chrono::high_resolution_clock::now();
    display("tbb::parallel_sort", start, end);
  }

  return 0;
}
