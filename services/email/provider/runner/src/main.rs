use std::collections::HashMap;

use clap::{Parser, Subcommand};
use hc_zome_email_notifications_types::{EmailCredentials, SendEmailSignal};
use holochain::prelude::{AppBundle, ExternIO, Signal};
use holochain_client::*;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

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
        sender_email_address: String,

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

    if let Some(Commands::RegisterCredentials {
        sender_email_address,
        password,
        smtp_relay_url,
    }) = args.command
    {
        let email_credentials = EmailCredentials {
            sender_email_address,
            password,
            smtp_relay_url,
        };

        app_agent_ws
            .call_zome(
                "email_notifications_provider".into(),
                "email_notifications_provider".into(),
                "publish_new_email_credentials".into(),
                ExternIO::encode(email_credentials)?,
            )
            .await
            .map_err(|err| anyhow::anyhow!("Failed to publish email credentials: {err:?}"))?;
        println!("Successfully registered new email credentials");

        std::process::exit(0);
    }

    // Listen for signal
    app_agent_ws
        .on_signal(|signal| {
            let Signal::App { signal, .. } = signal else {
                return ();
            };

            let Ok(send_email_signal) = signal.into_inner().decode::<SendEmailSignal>() else {
                return ();
            };

            tokio::spawn(async move {
                if let Err(err) = send_email(send_email_signal).await {
                    println!("Error sending email: {err:#?}");
                }
            });
        })
        .await?;

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");

    Ok(())
}

async fn send_email(send_email_signal: SendEmailSignal) -> anyhow::Result<()> {
    let email = Message::builder()
        .from(
            format!(
                "Sender <{}>",
                send_email_signal.credentials.sender_email_address.clone()
            )
            .parse()
            .unwrap(),
        )
        .to(format!("Receiver <{}>", send_email_signal.email_address)
            .parse()
            .unwrap())
        .subject(send_email_signal.email.subject)
        .body(send_email_signal.email.body)
        .unwrap();

    let creds = Credentials::new(
        send_email_signal.credentials.sender_email_address.clone(),
        send_email_signal.credentials.password.clone(),
    );

    // Open a remote connection to the given smtp relay
    let mailer = SmtpTransport::relay(send_email_signal.credentials.smtp_relay_url.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Could not send email: {:?}", e)),
    }
}
