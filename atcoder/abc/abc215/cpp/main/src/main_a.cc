#include <iostream>
#include <string>

int main() {
  std::string str;
  std::cin >> str;

  if (str == static_cast<std::string>("Hello,World!")){
    std::cout << "AC" << std::endl;
  }
  else{
    std::cout << "WA" << std::endl;
  }
  return EXIT_SUCCESS;
}
