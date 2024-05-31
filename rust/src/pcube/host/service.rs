use crate::pcube::common::logger::log;
use crate::pcube::common::enums::EExitCode;
use crate::pcube::common::service_config::ServiceConfig;
use crate::pcube::common::mq_handler::MQHandler;

pub struct Service 
{
    config: ServiceConfig,
    listening: bool,
    mq_handler: MQHandler
}

impl Service 
{
    pub fn new(service_config: ServiceConfig) -> Self 
    {
        Self {config: service_config, listening: false, mq_handler: MQHandler::new()}
    }

    pub fn start_listener(&mut self) -> bool 
    {
        self.listening = true;
        match self.mq_handler.connect(<Option<String> as Clone>::clone(&self.config.q_name_host).unwrap().as_str(), <Option<String> as Clone>::clone(&self.config.q_name_worker).unwrap().as_str())
        {
            EExitCode::SUCCESS => 
            {
                log(&format!("Service start listening : host({})", self.config.is_host));
                return true;
            }
            EExitCode::FAIL => 
            {
                return false;
            }
        }
    }

    pub fn stop_listener(&mut self) 
    {
        self.listening = false;
        log("Service stop listening");
        self.mq_handler.disconnect(self.config.is_host);
    }

    pub fn run(&mut self) -> EExitCode 
    {
        let mut exit_code = EExitCode::SUCCESS;
        if self.start_listener()
        {
            self.mq_handler.send_wait("task-1");
            while self.listening
            {
                let (_message, status) = self.mq_handler.receive_wait();
                match status
                {
                    EExitCode::SUCCESS => 
                    {
                        self.stop_listener();
                    }
                    EExitCode::FAIL => 
                    {
                        exit_code = EExitCode::FAIL;
                        self.stop_listener();
                    }
                }
            }
        }
        else
        {
            log("Unable to init listener");
            exit_code = EExitCode::FAIL;
        }
        return exit_code
    }
}