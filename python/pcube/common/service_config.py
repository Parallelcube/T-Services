class ServiceConfig:
    SYSTEM_HOST_QUEUE_NAME = "/mq_queue_host"
    SYSTEM_WORKER_QUEUE_NAME = "/mq_queue_worker"
    SYSTEM_SM_QUEUE_NAME = "/sm_services"

    def __init__(self, args: list):
        self.is_host = False
        self._match_is_host(args)
        if self.is_host:
            self.q_name_host = ServiceConfig.SYSTEM_HOST_QUEUE_NAME
            self.q_name_worker = ServiceConfig.SYSTEM_WORKER_QUEUE_NAME
        else:
            self.q_name_host = ServiceConfig.SYSTEM_WORKER_QUEUE_NAME
            self.q_name_worker = ServiceConfig.SYSTEM_HOST_QUEUE_NAME
        self.sm_name = ServiceConfig.SYSTEM_SM_QUEUE_NAME

    def _match_is_host(self, args: list):
        for arg in args:
            if arg == "--host":
                self.is_host = True
                args.remove(arg)
                break
