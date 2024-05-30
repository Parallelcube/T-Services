mod pcube;

use std::process::ExitCode;
use std::env;

use pcube::enums::EExitCode;
use pcube::service_config::ServiceConfig;
use pcube::service::Service;

fn main() -> ExitCode
{
    let mut args = env::args().collect::<Vec<String>>().clone();
    args.remove(0);
    let service_config = ServiceConfig::new(&mut args);
    let mut service = Service::new(service_config);
    let exit_code = match service.run() {
        EExitCode::SUCCESS => ExitCode::SUCCESS,
        EExitCode::FAIL => ExitCode::FAILURE,
    };
    ExitCode::from(exit_code)
}
