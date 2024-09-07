#pragma once

#include <string>
#include <vector>


namespace pcube
{
    class ServiceConfig
    {
    public:
        ServiceConfig(std::vector<std::string>& args);
        virtual ~ServiceConfig();

        bool        is_host;
        std::string q_name_host;
        std::string q_name_worker;
        std::string sm_name;

    private:
        void match_is_host(std::vector<std::string>& args);

    };
}