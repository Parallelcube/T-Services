from pcube.common.logger import log
from pcube.common.enums import EExitCode
from pcube.common.mq_handler import MQHandler
from pcube.common.service_config import ServiceConfig

class Service:
    def __init__(self, config: ServiceConfig):
        self._config: ServiceConfig = config
        self._listening = False
        self._mq_handler = MQHandler()

    def start_listener(self) -> bool:
        self._listening = True
        exit_code = self._mq_handler.connect(self._config.q_name_host, self._config.q_name_worker)
        if exit_code == EExitCode.SUCCESS:
            log(f"Service start listening : host({self._config.is_host})")
            return True
        return False

    def stop_listener(self):
        self._listening = False
        log("Service stop listening")
        self._mq_handler.disconnect(self._config.is_host)

    def run(self) -> EExitCode:
        exit_code = EExitCode.SUCCESS
        if self.start_listener():

            while self._listening:
                message, status = self._mq_handler.receive_wait()
                if status == EExitCode.SUCCESS:
                    self._mq_handler.send_wait(f"{message} processed")
                    self.stop_listener()
                else:
                    exit_code = EExitCode.FAIL
                    self.stop_listener()
        else:
            log("Unable to init listener")
            exit_code = EExitCode.FAIL
        return exit_code
        