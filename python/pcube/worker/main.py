import sys
import signal 


from pcube.worker.service_config import ServiceConfig
from pcube.worker.service import Service
from pcube.common.logger import log

service = None

def cancel_callback(signum, frame):
    log(f'Signal {signum} reveived')
    if service:
        service.stop_listener()

signal.signal(signal.SIGTERM, cancel_callback)

if __name__ == "__main__":

    service = Service(ServiceConfig([]))
    exit_code = service.run()
    sys.exit(exit_code.value)
