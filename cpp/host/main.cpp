#include "host/Service.h"
#include "common/ServiceConfig.h"

#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <vector>

int main(int argc, char *argv[])
{
    std::vector<std::string> args;
    args.push_back("--host");
    pcube::ServiceConfig service_config(args);
    pcube::Service service(service_config);
    int exit_code = service.run();
    return exit_code;
}