from posix_ipc import MessageQueue, O_RDONLY, O_WRONLY, O_CREAT

from pcube.common.enums import EExitCode
from pcube.common.logger import log

class MQHandler:
    MAX_MESSAGE_SIZE = 512

    def __init__(self):
        self.mq_request = None
        self.mq_response = None

    def connect(self, mq_request_name: str, mq_response_name: str) -> EExitCode:
        exit_code = EExitCode.SUCCESS
        self.mq_request = MessageQueue(mq_request_name, 
                                        O_CREAT | O_WRONLY, 
                                        max_message_size=MQHandler.MAX_MESSAGE_SIZE, 
                                        max_messages=1, 
                                        read = False, 
                                        write = True)
        if self.mq_request is None:
            log(f"Error opening with mq_request")
            exit_code = EExitCode.FAIL

        self.mq_response = MessageQueue(mq_response_name, 
                                        O_CREAT | O_RDONLY, 
                                        max_message_size=MQHandler.MAX_MESSAGE_SIZE, 
                                        max_messages=1, 
                                        read = True, 
                                        write = False)
        if self.mq_response is None:
            log(f"Error opening with mq_response")
            exit_code = EExitCode.FAIL
        return exit_code

    def disconnect(self, unlink: bool) -> EExitCode: 
        exit_code = EExitCode.SUCCESS
        if self.mq_request:
            self.mq_request.close()
            if unlink:
                self.mq_request.unlink()
            self.mq_request = None
        if self.mq_response:
            self.mq_response.close()
            if unlink:
                self.mq_response.unlink()
            self.mq_response = None
        return exit_code
    
    def send_wait(self, message: str) -> EExitCode:
        try:
            log(f"Sending message '{message}'")
            self.mq_request.send(message=message, timeout=None)
            return EExitCode.SUCCESS
        except KeyboardInterrupt as ex:
            log(f"Safe KeyboardInterrupt")
            return EExitCode.FAIL
        except Exception as ex:
            log(f"Error mq_send {ex}")
            return EExitCode.FAIL

    def receive_wait(self) -> tuple[str, EExitCode]:
        try:
            message, _ = self.mq_response.receive(timeout=None)
            decoded_message = message.decode()
            log(f"Received message '{decoded_message}'")
            return decoded_message, EExitCode.SUCCESS
        except KeyboardInterrupt as ex:
            log(f"Safe KeyboardInterrupt")
            return f"{ex}", EExitCode.FAIL
        except Exception as ex:
            log(f"Error mq_receive {ex}")
            return f"{ex}", EExitCode.FAIL
        