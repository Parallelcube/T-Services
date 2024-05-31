use std::process::ExitCode;

use rsservices::pcube::common::enums::EExitCode;
use rsservices::pcube::common::service_config::ServiceConfig;
use rsservices::pcube::worker::service::Service;

fn main() -> ExitCode
{
    let mut args= vec![];
    let service_config = ServiceConfig::new(&mut args);
    let mut service = Service::new(service_config);
    let exit_code = match service.run() {
        EExitCode::SUCCESS => ExitCode::SUCCESS,
        EExitCode::FAIL => ExitCode::FAILURE,
    };
    ExitCode::from(exit_code)
}
