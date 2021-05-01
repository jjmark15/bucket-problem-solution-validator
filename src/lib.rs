use crate::application::ApplicationServiceImpl;
use crate::ports::cli::structopt::run_cli;

mod application;
mod ports;
mod domain;

#[derive(Default)]
pub struct Application;

impl Application {
    pub fn new() -> Self {
        Application
    }

    pub fn run(&self) {
        let application_service = ApplicationServiceImpl::new();
        run_cli(&application_service);
    }
}
