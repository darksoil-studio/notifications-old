use std::collections::HashMap;

use clap::{Parser, Subcommand};
use holochain::{prelude::{AppBundle, ExternIO}, HOLOCHAIN_VERSION};
use holochain_client::*;

mod launch;

use launch::{install_app, launch_holochain};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Simple program to greet a person
#[derive(Subcommand, Debug)]
enum Commands {
    RegisterCredentials {
        /// Username for the sender email account
        #[arg(long)]
        username: String,

        /// Password for the sender email account
        #[arg(short, long)]
        password: String,

        /// Url of the SMTP relay server
        #[arg(short, long)]
        smtp_relay_url: String,
    },
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        println!("Error running holochain: {err:?}");
    }
}

const PROVIDER_APP_ID: &'static str = "email_notifications_provider";

fn provider_app_bundle() -> AppBundle {
    let bytes = include_bytes!("../../app/email_notifications_provider.happ");
    AppBundle::decode(bytes).unwrap()
}

async fn run() -> anyhow::Result<()> {
    let holochain_info = launch_holochain().await?;

    let mut admin_ws =
        AdminWebsocket::connect(format!("ws://localhost:{}", holochain_info.admin_port)).await?;

    let apps = admin_ws
        .list_apps(None)
        .await
        .map_err(|err| anyhow::anyhow!("Could not connect to admin ws: {err:?}"))?;

    if apps.len() == 0 {
        install_app(
            &mut admin_ws,
            String::from(PROVIDER_APP_ID),
            provider_app_bundle(),
            HashMap::new(),
            None,
        )
        .await?;
    }

    let mut app_agent_ws = AppAgentWebsocket::connect(
        format!("ws://localhost:{}", holochain_info.app_port),
        PROVIDER_APP_ID.into(),
        holochain_info.lair_client,
    )
    .await?;

    let args = Args::parse();

    if let Some(commands) = args.command {
        let email_credentials = EmailCredentials {
            username: commands.username,
            password: commands.password,
            smtp_relay_url: commands.smtp_relay_url,
        };
        
        app_agent_ws.call_zome("email_notifications_provider".into(), "publish_new_email_credentials", ExternIO::encode(email_credentials)).await?;
        println!("Successfully registered new email credentials");

        std::process::exit(0);
    }

    // Listen for signal
}
