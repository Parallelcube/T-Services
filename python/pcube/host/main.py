import sys
import signal 

from python.pcube.common.service_config import ServiceConfig
from pcube.host.service import Service
from pcube.common.logger import log

service = None

def cancel_callback(signum, frame):
    log(f'Signal {signum} reveived')
    if service:
        service.stop_listener()

signal.signal(signal.SIGTERM, cancel_callback)

if __name__ == "__main__":
    cli_args = ["--host"]
    service = Service(ServiceConfig(cli_args))
    exit_code = service.run()
    sys.exit(exit_code.value)
