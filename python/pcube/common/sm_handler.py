from posix_ipc import SharedMemory, O_CREAT, O_RDWR
import mmap

from pcube.common.enums import EExitCode
from pcube.common.logger import log

class SMHandler:
    INITIAL_SIZE = 512

    def __init__(self):
        self.sm_segment = None
        self.map_file = None
        self.mapped_size = 0

    @staticmethod
    def calculate_best_size(minimal_size: int) -> int:
        return ((minimal_size + mmap.PAGESIZE - 1) // mmap.PAGESIZE) * mmap.PAGESIZE
    
    def update_map(self) -> EExitCode:
        if (self.sm_segment is not None and self.mapped_size < self.sm_segment.size):
            try:
                self.map_file = mmap.mmap(self.sm_segment.fd, self.sm_segment.size)
                self.mapped_size = self.sm_segment.size
                log(f"Shared memory update map {self.map_file.size()} bytes")
            except Exception:
                return EExitCode.FAIL
        return EExitCode.SUCCESS

    def resize(self, size: int=None) -> EExitCode:
        optimal_size = SMHandler.calculate_best_size(size)
        log(f"Shared memory segment resize {optimal_size} bytes")
        self.map_file.resize(optimal_size)
        return EExitCode.SUCCESS

    def connect(self, name: str) -> EExitCode:
        optimal_size = SMHandler.calculate_best_size(SMHandler.INITIAL_SIZE)
        self.sm_segment = SharedMemory(name, O_CREAT | O_RDWR, size=optimal_size)
        if self.sm_segment is None:
            log(f"Error opening shared memory")
            return EExitCode.FAIL
        return self.update_map()

    def disconnect(self, unlink: bool) -> EExitCode:
        if self.map_file is not None:
            try:
                self.map_file.close()
            except Exception as ex:
                log(f"Error map file close {ex}")
                return EExitCode.FAIL
            self.map_file = None

        if self.sm_segment is not None:
            try:
                if unlink:
                    self.sm_segment.unlink()
                self.sm_segment.close_fd()
            except Exception as ex:
                log(f"Error sm segment close {ex}")
                return EExitCode.FAIL
            self.sm_segment = None
        return EExitCode.SUCCESS

    def write(self, payload: str) -> tuple[int, EExitCode]:
        payload_size = len(payload)
        log(f"Shared memory write '{payload}' {payload_size} bytes")
        exit_code = EExitCode.SUCCESS
        if self.sm_segment is None or self.sm_segment.size != SMHandler.calculate_best_size(payload_size):
            self.resize(payload_size)
            exit_code = self.update_map()
        
        if exit_code == EExitCode.SUCCESS:
            try:
                self.map_file.seek(0)
                _ = self.map_file.write(payload.encode())
                return payload_size, exit_code
            except Exception as ex:
                log(f"Error sm write {ex}")
        return None, EExitCode.FAIL
    
    def read(self, payload_size: int) -> tuple[str, EExitCode]:
        if self.update_map() == EExitCode.SUCCESS:
            try:
                self.map_file.seek(0)
                payload = self.map_file.read(payload_size).decode()
                log(f"Shared memory read '{payload}' {payload_size} bytes")
                return payload, EExitCode.SUCCESS
            except Exception as ex:
                log(f"Error sm read {ex}")
        return None, EExitCode.FAIL
