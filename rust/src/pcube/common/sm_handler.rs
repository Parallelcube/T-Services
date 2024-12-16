extern crate nix;

use super::logger::log;
use super::enums::EExitCode;
use nix::fcntl::OFlag;
use nix::sys::mman::{shm_open, shm_unlink, mmap, munmap, MapFlags};
use nix::sys::stat::{fstat, Mode};
use nix::unistd::ftruncate;
use nix::libc::{sysconf, _SC_PAGESIZE};
use std::{str, ptr, ptr::NonNull, num::NonZero};
use std::os::unix::io::{OwnedFd, AsRawFd};
use std::os::raw::c_void;


pub struct SMHandler 
{
    sm_name: Option<String>,
    sm_segment: Option<OwnedFd>,
    map_file: Option<NonNull<c_void>>,
    mapped_size: usize,
    system_page_size: usize
}

impl SMHandler 
{
    pub fn new() -> Self 
    {
        Self {sm_name: None, sm_segment: None, map_file: None, mapped_size: 0, system_page_size: unsafe { sysconf(_SC_PAGESIZE) } as usize}
    }

    fn calculate_best_size(&self, minimal_size: usize) -> usize
    {
        return ((minimal_size as f64 / self.system_page_size as f64).ceil()) as usize * self.system_page_size;
    }

    fn get_current_size(&self) -> usize
    {
        if let Some(sm_fd) = &self.sm_segment
        {
            match fstat(sm_fd.as_raw_fd())
            {
                Ok(file_stats) => { return file_stats.st_size as usize; }
                Err(_err) => {}
            }
        };
        return 0
    }

    pub fn update_map(&mut self) -> EExitCode 
    {
        let segment_size = self.get_current_size();

        if let Some(sm_fd) = &self.sm_segment
        {
            if <usize as Into<usize>>::into(self.mapped_size) != segment_size
            {
                if segment_size > 0
                {
                    log(&format!("Shared memory update map {} bytes", segment_size.to_string()));
                    match unsafe {
                        mmap(
                            None,
                            NonZero::new(segment_size).unwrap(),
                            nix::sys::mman::ProtFlags::PROT_READ | nix::sys::mman::ProtFlags::PROT_WRITE,
                            MapFlags::MAP_SHARED,
                            sm_fd,
                            0,
                        )
                    } 
                    {
                        Ok(file_ptr) => 
                        {
                            self.map_file = Some(file_ptr);
                            self.mapped_size = segment_size
                        },
                        Err(error) => 
                        {
                            log(&format!("Error mapping shared memory {}", error));
                            return EExitCode::FAIL
                        }
                    };
                }
                else 
                {
                    log("Invalid segment size");
                    return EExitCode::FAIL
                }
            }
        }
        return EExitCode::SUCCESS
    }

    pub fn connect(&mut self, sm_name: &str) -> EExitCode 
    {
        let mode = Mode::S_IWUSR | Mode::S_IRUSR;
        self.sm_name = Some(sm_name.to_string());
        match shm_open(sm_name, OFlag::O_CREAT | OFlag::O_RDWR, mode)
        {
            Ok(sm_fd) => 
            {
                self.sm_segment = Some(sm_fd)
            }
            Err(error) => 
            {
                log(&format!("Error opening shared memory {}", error));
                return EExitCode::FAIL
            }
        };
        return self.update_map();
    }

    pub fn disconnect(&mut self, unlink: bool) -> EExitCode 
    {
        if let Some(file_ptr) = &self.map_file
        {
            unsafe { 
                match munmap(*file_ptr, self.mapped_size)
                {
                    Ok(_) => 
                    {
                        self.map_file = None;
                        self.mapped_size = 0;
                    }
                    Err(error) => 
                    {
                        log(&format!("Error munmap with {}", error));
                    }
                }
            }
        };

        if unlink
        { 
            let _ = shm_unlink(<Option<String> as Clone>::clone(&self.sm_name).unwrap().as_str());
        }

        // OwnedFd will be closed in the drop
        self.sm_segment = None;

        return EExitCode::SUCCESS;
    }

    pub fn write(&mut self, payload: &str) -> EExitCode 
    {
        let mut exit_code = EExitCode::SUCCESS;
        let segment_size = self.get_current_size();
        log(&format!("Shared memory write '{}' {} bytes", payload, payload.len()));
        let new_size = self.calculate_best_size(payload.len());
        if segment_size != new_size
        {
            log(&format!("Shared memory resize '{}->{}'", segment_size, new_size));
            if let Some(sm_fd) = &self.sm_segment
            {
                match ftruncate(sm_fd, new_size as i64)
                {
                    Ok(_) => { exit_code = self.update_map()}
                    Err(error) => 
                    {
                        log(&format!("Error resizing shared memory {} ", error));
                        exit_code = EExitCode::FAIL
                    }
                }
            };
        }

        if exit_code == EExitCode::SUCCESS
        {
            if let Some(sm_ptr) = &self.map_file
            {
                unsafe {
                    ptr::copy_nonoverlapping(payload.as_ptr(), sm_ptr.as_ptr() as *mut u8, payload.len());
                }
            }
        }

        return exit_code
    }

    pub fn read(&mut self, payload_size: usize) -> (String, EExitCode)
    {
        if self.update_map() == EExitCode::FAIL
        {
            return ("".to_string(), EExitCode::FAIL)
        }

        if let Some(sm_ptr) = &self.map_file
        {
            let buffer = 
            unsafe {
                std::slice::from_raw_parts(sm_ptr.as_ptr() as *const u8, payload_size)
            };

            match String::from_utf8((&buffer[..payload_size]).to_vec())
            {
                Ok(message) => 
                {
                    log(&format!("Shared memory read '{}' {} bytes", message, message.len()));
                    return (message, EExitCode::SUCCESS)
                }
                Err(_) => {}
            }
        };

        return ("".to_string(), EExitCode::FAIL)
    }
}
