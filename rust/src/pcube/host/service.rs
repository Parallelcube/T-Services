use crate::pcube::common::logger::log;
use crate::pcube::common::enums::EExitCode;
use crate::pcube::common::service_config::ServiceConfig;
use crate::pcube::common::mq_handler::MQHandler;
use crate::pcube::common::sm_handler::SMHandler;

pub struct Service 
{
    config: ServiceConfig,
    listening: bool,
    mq_handler: MQHandler,
    sm_handler: SMHandler
}

impl Service 
{
    pub fn new(service_config: ServiceConfig) -> Self 
    {
        Self {config: service_config, listening: false, mq_handler: MQHandler::new(), sm_handler: SMHandler::new()}
    }

    pub fn start_listener(&mut self) -> bool 
    {
        self.listening = true;

        match self.sm_handler.connect(<Option<String> as Clone>::clone(&self.config.sm_name).unwrap().as_str())
        {
            EExitCode::FAIL => 
            {
                return false;
            },
            _ => {}
        }

        match self.mq_handler.connect(<Option<String> as Clone>::clone(&self.config.q_name_host).unwrap().as_str(), <Option<String> as Clone>::clone(&self.config.q_name_worker).unwrap().as_str())
        {
            EExitCode::FAIL => 
            {
                return false;
            },
            _ => {}
        }

        log(&format!("Service start listening : host({})", self.config.is_host));
        return true;
    }

    pub fn stop_listener(&mut self) -> EExitCode 
    {
        self.listening = false;
        log("Service stop listening");
        self.mq_handler.disconnect(self.config.is_host);
        return self.sm_handler.disconnect(self.config.is_host);
    }

    pub fn handle_run_error(&mut self) -> EExitCode 
    {
        self.stop_listener();
        return EExitCode::FAIL
    }

    pub fn run(&mut self) -> EExitCode 
    {
        if !self.start_listener()
        {
            log("Unable to init listener");
            return self.stop_listener();
        }

        let payload_request = "payload of task-1";

        let status  = self.sm_handler.write(payload_request);
        if status != EExitCode::SUCCESS
        {
            return self.handle_run_error()
        }

        let status = self.mq_handler.send_wait(payload_request.len().to_string().as_str());
        if status != EExitCode::SUCCESS
        {
            return self.handle_run_error()
        }


        while self.listening
        {
            let (response_message, status) = self.mq_handler.receive_wait();
            if status != EExitCode::SUCCESS
            {
                return self.handle_run_error()
            }

            let (_payload_response, status) = self.sm_handler.read(response_message.parse::<usize>().unwrap());
            if status != EExitCode::SUCCESS
            {
                return self.handle_run_error()
            }

            self.stop_listener();
        }

        return EExitCode::SUCCESS;
    }
}