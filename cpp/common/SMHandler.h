#pragma once

#include <string>

namespace pcube
{
    class SMHandler
    {
    public:
        SMHandler();
        virtual ~SMHandler();

        int connect(const std::string& name);
        int disconnect(const bool& unlink);
        int read(std::string& buffer);
        int write(const std::string& buffer);

    private:
        int update_map();
        size_t calculate_best_size(size_t minimal_size) const;
        size_t get_current_size() const;

        std::string _sm_name;
        int         _sm_segment;
        void*       _sm_ptr;
        int         _system_page_size;
        int         _mapped_size;
    };
}