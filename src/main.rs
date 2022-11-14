#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]
// We allow expect since it forces good error messages at the least.
#![allow(clippy::expect_used)]

use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::*;
use tracing::{trace, info, error};


#[derive(Debug, clap::Parser)]
#[clap(about = "Pam Testing Tool")]
/// Test a pam service and how it behaves.
pub struct Opt {
    /// Optionally pre-enter the password for the account to test.
    #[clap(short='W')]
    pub password: Option<String>,

    /// The name of the pam service to interact with. This is generally linked to the
    /// name of a file in /etc/pam.d/. If you want to test the behaviour of /etc/pam.d/sudo
    /// then service would be "sudo".
    pub service: String,

    /// The name of the account you want to pass into pam to test against.
    pub name: String,
}


fn main() {
    let opt = Opt::parse();

    let fmt_layer = fmt::layer()
    .with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    trace!(?opt);

    let Opt {
        password, service, name
    } = opt;

    let password = password.unwrap_or_else(|| {
        rpassword::prompt_password("Enter password: ").unwrap()
    });

    let mut auth = pam::Authenticator::with_password(&service).unwrap();

    auth.get_handler().set_credentials(name, password);

    if let Err(e) = auth.authenticate() {
        error!(?e, "Authentication failed");
    } else {
        info!("Successfully authenticated!");
    }

    if let Err(e) = auth.open_session() {
        error!(?e, "Session failed");
    } else {
        info!("Successfully opened session!");
    }
}
