mod cli;
mod icore;
mod logger;
use clap::{load_yaml, App};
use cli::{parser::parse_input, typer};
use icore::arg::{Arg, SendArg, RecvArg};
use icore::{message::send_msg, message::Message, receiver, sender};

#[async_std::main]
async fn main() {
    // Init communication between UI and model.
    typer::launch();

    // Init logger. No need to terminate process if error happens.
    // TODO: logger should be turned off in release.
    if let Err(e) = logger::init_log() {
        //eprintln!("Cannot init logger: {}", e);
        send_msg(Message::Error(format!("cannot init logger: {}", e)));
    }

    let yaml = load_yaml!("cli/cli.yaml");
    let m = App::from(yaml).get_matches();

    match parse_input(&m) {
        Ok(Arg::R(r)) => start_receiver(r).await,
        Ok(Arg::S(s)) => start_sender(s).await,
        Err(e) => {
            send_msg(Message::Fatal(format!("cannot parse input: {}", e)));
        },
    }
}

// TODO: identify fatal errors from normal ones.
async fn start_sender(s: SendArg) {
    log::debug!("Get sender arg:\n{:?}", &s);

    if let Err(e) = sender::launch(s).await {
        //eprintln!("Error in sender: {}", e);
        send_msg(Message::Fatal(format!("in sender: {}", e)));
    }
}

async fn start_receiver(r: RecvArg) {
    log::debug!("Get receiver arg:\n{:?}", &r);

    if let Err(e) = receiver::launch(r).await {
        //eprintln!("Error in receiver: {}", e);
        send_msg(Message::Fatal(format!("in receiver: {}", e)));
    }
}