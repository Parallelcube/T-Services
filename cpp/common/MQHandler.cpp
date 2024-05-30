#include "common/MQHandler.h"

#include "common/Logger.h"

#include <fcntl.h>
#include <string.h>

using namespace pcube;

constexpr mqd_t MQ_ERROR = (mqd_t)-1;
constexpr long MAX_MESSAGE_SIZE = 512;

MQHandler::MQHandler():_mq_request(MQ_ERROR),_mq_response(MQ_ERROR),_mq_request_name(""),_mq_response_name("")
{
}

MQHandler::~MQHandler()
{
}

int MQHandler::connect(const std::string& mq_request_name, const std::string& mq_response_name)
{
    int exit_code = EXIT_SUCCESS;
    mq_attr mq_attributes;
    mq_attributes.mq_maxmsg = 1;
    mq_attributes.mq_msgsize = MAX_MESSAGE_SIZE;
    _mq_request_name = mq_request_name;
    _mq_request = mq_open(_mq_request_name.c_str(), O_CREAT | O_WRONLY, 0600, &mq_attributes);
    if (_mq_request == MQ_ERROR)
    {
        log("Error mq_open with mq_request ("+_mq_request_name+")");
        exit_code = EXIT_FAILURE;
    }

    if (exit_code == EXIT_SUCCESS)
    {
        _mq_response_name = mq_response_name;
        _mq_response = mq_open(_mq_response_name.c_str(), O_CREAT | O_RDONLY, 0600, &mq_attributes);
        if (_mq_response == MQ_ERROR)
        {
            log("Error mq_open with mq_response ("+_mq_response_name+")");
            exit_code = EXIT_FAILURE;
        }
    }
    return exit_code;
}

int MQHandler::disconnect(const bool& unlink)
{
    int exit_code = EXIT_SUCCESS;
    if (_mq_request != MQ_ERROR)
    {
        if(mq_close(_mq_request) == MQ_ERROR)
        {
            log("Error mq_close with _mq_request");
            exit_code = EXIT_FAILURE;
        }

        if(unlink)
        {
            mq_unlink(_mq_request_name.c_str());
        }
    }

    if (_mq_response != MQ_ERROR)
    {
        if(mq_close(_mq_response) == MQ_ERROR)
        {
            log("Error mq_close with _mq_response");
            exit_code = EXIT_FAILURE;
        }

        if(unlink)
        {
            mq_unlink(_mq_response_name.c_str());
        }
    }

    return exit_code;
}

int MQHandler::send_wait(const std::string& message) const
{
    int exit_code = EXIT_SUCCESS;
    log(std::string("Sending message '") + message + "'");
    int return_code = mq_send(_mq_request, message.data(), message.size(), 0);
    if (return_code == -1)
    {
        log(std::string("Error mq_send") + strerror(errno));
        exit_code = EXIT_FAILURE;
    }
    return exit_code;
}

int MQHandler::receive_wait(std::string& message) const
{
    int exit_code = EXIT_SUCCESS;
    message.resize(MAX_MESSAGE_SIZE);
    ssize_t num_bytes = mq_receive(_mq_response, message.data(), message.size(), 0);
    if (num_bytes != -1)
    {
        message.resize(num_bytes);
        log(std::string("Received message '") + message + "'");
    }
    else
    {
        std::string error_message(strerror(errno));
        log(std::string("Error mq_receive") + error_message);
        message = error_message;
        exit_code = EXIT_FAILURE;
    }
    return exit_code;
}