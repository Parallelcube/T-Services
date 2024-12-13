#include "common/SMHandler.h"

#include "common/Logger.h"

#include <sys/mman.h>
#include <sys/shm.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <cstring>
#include <fcntl.h>
#include <unistd.h>
#include <cmath>

using namespace pcube;

constexpr int SM_ERROR = -1;

SMHandler::SMHandler(): _sm_name(""),
                        _sm_segment(SM_ERROR),
                        _sm_ptr(nullptr),
                        _system_page_size(sysconf(_SC_PAGESIZE)),
                        _mapped_size(0)
{
}

SMHandler::~SMHandler()
{
}

int SMHandler::connect(const std::string& name)
{
    _sm_name = name;
    _sm_segment = shm_open(_sm_name.c_str(), O_CREAT | O_RDWR, 0600);
    if (_sm_segment == SM_ERROR)
    {
        log(std::string("Error shm_open with (") + _sm_name + ") " + strerror(errno) + "'");
        return EXIT_FAILURE;
    }
    return update_map();
}


int SMHandler::disconnect(const bool& unlink)
{
    int exit_code = EXIT_SUCCESS;
    if (_sm_ptr)
    {
        if(_mapped_size > 0)
        {
            int error_code = munmap(_sm_ptr, _mapped_size);
            _sm_ptr = nullptr;
            _mapped_size = 0;
            if (error_code)
            {
                log(std::string("Error munmap with ") + strerror(errno));
            }
        }
    }

    if (_sm_segment != SM_ERROR)
    {
        if (unlink)
        {
            shm_unlink(_sm_name.c_str());
        }

        if (close(_sm_segment) == SM_ERROR)
        {
            log(std::string("Error close with ") + strerror(errno));
            exit_code = EXIT_FAILURE;
        }
        _sm_segment  = SM_ERROR;
    }

    return exit_code;
}

int SMHandler::update_map()
{
    int exit_code = EXIT_SUCCESS;
    size_t segment_size = SMHandler::get_current_size();
    if(_mapped_size != segment_size)
    {
        if (segment_size > 0)
        {
            log(std::string("Shared memory update map ") + std::to_string(segment_size) + " bytes");
            _sm_ptr = mmap((void*)0, segment_size, PROT_READ | PROT_WRITE, MAP_SHARED, _sm_segment, 0);
            _mapped_size = segment_size;
            if (_sm_ptr == MAP_FAILED)
            {
                log(std::string("Error mapping shared memory ") + strerror(errno));
                exit_code = EXIT_FAILURE;
            }
        }
        else
        {
            log(std::string("Invalid segment size ") + strerror(errno));
            exit_code = EXIT_FAILURE;
        }
    }
    return exit_code;
}

int SMHandler::read(std::string& buffer)
{
    if (update_map() != EXIT_SUCCESS)
    {
        return EXIT_FAILURE;
    }

    if (_sm_ptr)
    {
        buffer.assign((char*)_sm_ptr, buffer.size());
        log(std::string("Shared memory read '") + buffer + "' " + std::to_string(buffer.size()) + " bytes");
    }
    return EXIT_SUCCESS;
}

int SMHandler::write(const std::string& buffer)
{
    int exit_code = EXIT_SUCCESS;
    
    size_t segment_size = get_current_size();
    log(std::string("Shared memory write '") + buffer + "' " + std::to_string(buffer.size()) +" bytes");
    size_t new_size = calculate_best_size(buffer.size());
    if (segment_size != new_size)
    {
        log(std::string("Shared memory resize '") + std::to_string(segment_size) + "->" + std::to_string(new_size) + "'");
        if (!ftruncate(_sm_segment, new_size))
        {
            exit_code = update_map();
        }
        else
        {
            log(std::string("Error resizing shared memory") + strerror(errno));
            exit_code = EXIT_FAILURE;
        }
    }

    if (exit_code == EXIT_SUCCESS)
    {
        memcpy(_sm_ptr, buffer.data(), buffer.size());
    }
    
    return exit_code;
}

size_t SMHandler::calculate_best_size(size_t minimal_size) const
{
    return std::ceil(minimal_size / (float)_system_page_size) * _system_page_size;
}

size_t SMHandler::get_current_size() const
{
    struct stat fileinfo;
    if(_sm_segment != SM_ERROR && 0 == fstat(_sm_segment, &fileinfo))
    {
        return fileinfo.st_size;
    }
    return 0;
}