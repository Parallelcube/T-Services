#include "Service.h"

#include "Logger.h"
#include "MQHandler.h"

#include <iostream>

using namespace pcube;

Service::Service(const ServiceConfig& config):  _config(config),
                                                _listening(false)
{
}

Service::~Service()
{
}

std::string bool_to_string(bool value) 
{
    return value ? "true" : "false";
}

bool Service::start_listener()
{
    _listening = true;
    if (_sm_handler.connect(_config.sm_name) != EXIT_SUCCESS)
    {
        return false;
    }

    if (_mq_handler.connect(_config.q_name_host, _config.q_name_worker) != EXIT_SUCCESS)
    {
        return false;
    }

    log("Service start listening : host("+bool_to_string(_config.is_host)+")");
    return true;
}

void Service::stop_listener()
{
    _listening = false;
    log("Service stop listening");
    _mq_handler.disconnect(_config.is_host);
    _sm_handler.disconnect(_config.is_host);
}

int Service::run()
{
    if (!start_listener())
    {
        log("Unable to init listener");
        return handle_run_error();
    }

    std::string message;
    std::string payload;
    while (_listening)
    {
        if (_mq_handler.receive_wait(message) != EXIT_SUCCESS)
        {
            return handle_run_error();
        }

        payload.resize(stoi(message));
        if (_sm_handler.read(payload) != EXIT_SUCCESS)
        {
            return handle_run_error();
        }

        payload = payload + " processed";
        
        if (_sm_handler.write(payload) != EXIT_SUCCESS)
        {
            return handle_run_error();
        }

        if (_mq_handler.send_wait(std::to_string(payload.size())) != EXIT_SUCCESS)
        {
            return handle_run_error();
        }
        
        stop_listener();
    }
    return EXIT_SUCCESS;
}

int Service::handle_run_error()
{
    stop_listener();
    return EXIT_FAILURE;
}


