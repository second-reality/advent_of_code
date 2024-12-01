#include <iostream>
#include <ranges>
#include <fstream>
#include <string>
#include <cmath>
#include <vector>
#include <algorithm>

#include <unordered_map>

void part1(const char* data_filename);
void part2(const char* data_filename);

int main(int argc, char* argv[])
{
  if(argc != 2)
    return 1;

  // part1(argv[1]);
  part2(argv[1]);

  return 0;
}

void part1(const char* data_filename)
{
  std::ifstream data_file(data_filename);

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
}

void part2(const char* data_filename)
{
  std::ifstream data_file(data_filename);

  int a, b;

  std::vector<int> v1, v2;
  while(data_file >> a >> b)
  {
    v1.push_back(a);
    v2.push_back(b);
  }

  std::sort(v1.begin(), v1.end());
  std::sort(v2.begin(), v2.end());

  std::unordered_map<int, int> occurences; // number -> occurence
  for(auto elem: v2)
  {
    occurences[elem] += 1;
  }
  
  for(auto elem: occurences)
  {
    std::cout << "Key:[" << elem.first << "] Value:[" << elem.second << "]\n";

  }

  int similiraty = 0;
  for(auto elem: v1)
  {
    similiraty += elem * occurences[elem];
  }


  std::cout << std::abs(similiraty) << std::endl;
}


