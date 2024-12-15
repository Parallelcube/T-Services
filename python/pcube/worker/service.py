from pcube.common.logger import log
from pcube.common.enums import EExitCode
from pcube.common.mq_handler import MQHandler
from pcube.common.sm_handler import SMHandler
from pcube.common.service_config import ServiceConfig

class Service:
    def __init__(self, config: ServiceConfig):
        self._config: ServiceConfig = config
        self._listening = False
        self._mq_handler = MQHandler()
        self._sm_handler = SMHandler()

    def start_listener(self) -> bool:
        self._listening = True
        if self._sm_handler.connect(self._config.sm_name) != EExitCode.SUCCESS:
            return False
        if self._mq_handler.connect(self._config.q_name_host, self._config.q_name_worker) != EExitCode.SUCCESS:
            return False
        log(f"Service start listening : host({self._config.is_host})")
        return True

    def stop_listener(self):
        self._listening = False
        log("Service stop listening")
        self._mq_handler.disconnect(self._config.is_host)
        self._sm_handler.disconnect(self._config.is_host)

    def handle_run_error(self) -> EExitCode: 
        self.stop_listener()
        return EExitCode.FAIL
    
    def run(self) -> EExitCode:
        if not self.start_listener():
            log("Unable to init listener")
            return self.stop_listener()

        while self._listening:
            payload_size, status = self._mq_handler.receive_wait()
            if status != EExitCode.SUCCESS:
                return self.handle_run_error()

            payload, status = self._sm_handler.read(payload_size=int(payload_size))
            if status != EExitCode.SUCCESS:
                return self.handle_run_error()
            
            payload = payload + " processed"

            status = self._sm_handler.write(payload)
            if status != EExitCode.SUCCESS:
                return self.handle_run_error()

            self._mq_handler.send_wait(str(len(payload)))
            self.stop_listener()

        return EExitCode.SUCCESS
        