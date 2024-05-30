
const SYSTEM_HOST_QUEUE_NAME: &str = "/mq_queue_host";
const SYSTEM_WORKER_QUEUE_NAME: &str = "/mq_queue_worker";

pub struct ServiceConfig 
{
    pub is_host: bool,
    pub q_name_host: Option<String>,
    pub q_name_worker: Option<String>
}

impl ServiceConfig
{
    pub fn new(args: &mut Vec<String>) -> Self 
    {
        let mut config = Self {is_host: false, q_name_host: None, q_name_worker: None};
        config.match_is_host(args);
        if config.is_host
        {
            config.q_name_host = Some(SYSTEM_HOST_QUEUE_NAME.to_string());
            config.q_name_worker = Some(SYSTEM_WORKER_QUEUE_NAME.to_string());
        }
        else
        {
            config.q_name_host = Some(SYSTEM_WORKER_QUEUE_NAME.to_string());
            config.q_name_worker = Some(SYSTEM_HOST_QUEUE_NAME.to_string());
        }
        return config;
    }

    pub fn match_is_host(&mut self, args: &mut Vec<String>)
    {
        match args.iter().position(|n| n == "--host")
        {
            Some(position) => 
            {
                self.is_host=true;
                args.remove(position);
            },
            None => {}
        };
    }
}