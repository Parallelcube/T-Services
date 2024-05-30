#include "common/Logger.h"

#include <iostream>

void pcube::log(const std::string& message)
{
    std::cout << "cpp: "<< message << std::endl;
}