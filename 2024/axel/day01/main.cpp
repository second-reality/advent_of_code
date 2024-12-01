#include <iostream>
#include <ranges>
#include <fstream>
#include <string>
#include <cmath>
#include <vector>
#include <algorithm>

int main(int argc, char* argv[])
{
  if(argc != 2)
    return 1;

  std::ifstream data_file(argv[1]);

  int a, b;
  int total_dist = 0;

  std::vector<int> v1, v2;
  while(data_file >> a >> b)
  {
    v1.push_back(a);
    v2.push_back(b);
  }

  std::sort(v1.begin(), v1.end());
  std::sort(v2.begin(), v2.end());

  for(auto elem : std::views::zip(v1, v2))
  {
    total_dist += std::abs(std::get<0>(elem) - std::get<1>(elem));
  }


  std::cout << std::abs(total_dist) << std::endl;

  return 0;
}
