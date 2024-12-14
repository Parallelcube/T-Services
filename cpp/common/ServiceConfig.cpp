#include "common/ServiceConfig.h"

#include <algorithm>
#include <vector>

using namespace pcube;

constexpr const char* SYSTEM_HOST_QUEUE_NAME = "/mq_queue_host";
constexpr const char* SYSTEM_WORKER_QUEUE_NAME = "/mq_queue_worker";
constexpr const char* SYSTEM_SM_NAME = "/sm_services";

ServiceConfig::ServiceConfig(std::vector<std::string>& args):is_host(false)
{
    match_is_host(args);
    if (is_host)
    {
        q_name_host = SYSTEM_HOST_QUEUE_NAME;
        q_name_worker = SYSTEM_WORKER_QUEUE_NAME;
    }
    else
    {
        q_name_host = SYSTEM_WORKER_QUEUE_NAME;
        q_name_worker = SYSTEM_HOST_QUEUE_NAME;
    }

    sm_name = SYSTEM_SM_NAME;
}

ServiceConfig::~ServiceConfig()
{
}

void ServiceConfig::match_is_host(std::vector<std::string>& args)
{
    auto it = std::find(args.begin(), args.end(), "--host");
    if (it != args.end())
    {
        is_host = true;
        it = args.erase(it);
    }
}
