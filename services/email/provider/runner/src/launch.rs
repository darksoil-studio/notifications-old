use std::{collections::HashMap, path::PathBuf, sync::Arc};

use holochain::{
    conductor::{
        config::{AdminInterfaceConfig, ConductorConfig, KeystoreConfig},
        interface::InterfaceDriver,
        state::AppInterfaceId,
        Conductor,
    },
    prelude::{
        dependencies::kitsune_p2p_types::{
            config::{KitsuneP2pConfig, TransportConfig},
            dependencies::lair_keystore_api::{
                dependencies::sodoken::{BufRead, BufWrite},
                LairClient,
            },
        },
        AppBundle, AppBundleSource, MembraneProof, NetworkSeed, RoleName,
    },
};
use holochain_client::{AdminWebsocket, AppInfo, InstallAppPayload};
use holochain_keystore::{lair_keystore::spawn_lair_keystore_in_proc, MetaLairClient};

pub struct HolochainInfo {
    pub app_port: u16,
    pub admin_port: u16,
    pub lair_client: LairClient,
}

pub fn vec_to_locked(mut pass_tmp: Vec<u8>) -> std::io::Result<BufRead> {
    match BufWrite::new_mem_locked(pass_tmp.len()) {
        Err(e) => {
            pass_tmp.fill(0);
            Err(e.into())
        }
        Ok(p) => {
            {
                let mut lock = p.write_lock();
                lock.copy_from_slice(&pass_tmp);
                pass_tmp.fill(0);
            }
            Ok(p.to_read())
        }
    }
}

pub async fn launch_holochain() -> anyhow::Result<HolochainInfo> {
    let data_dir = dirs::data_local_dir().ok_or(anyhow::anyhow!("Could not get data local dir"))?;

    let keystore_dir = data_dir.join("keystore");
    let conductor_dir = data_dir.join("conductor");
    let passphrase = vec_to_locked(vec![]).expect("Can't build passphrase");

    let keystore = spawn_lair_keystore_in_proc(&keystore_dir, passphrase).await?;
    build_conductor(conductor_dir, keystore).await
}

pub async fn install_app(
    admin_ws: &mut AdminWebsocket,
    app_id: String,
    bundle: AppBundle,
    membrane_proofs: HashMap<RoleName, MembraneProof>,
    network_seed: Option<NetworkSeed>,
) -> anyhow::Result<AppInfo> {
    println!("Installing app {}", app_id);

    let agent_key = admin_ws
        .generate_agent_pub_key()
        .await
        .map_err(|err| anyhow::anyhow!("Could not generate pub key: {err:?}"))?;

    let app_info = admin_ws
        .install_app(InstallAppPayload {
            agent_key,
            membrane_proofs,
            network_seed,
            source: AppBundleSource::Bundle(bundle),
            installed_app_id: Some(app_id.clone()),
        })
        .await
        .map_err(|err| anyhow::anyhow!("Could not install app: {err:?}"))?;
    println!("Installed app {app_info:?}");

    let response = admin_ws
        .enable_app(app_id.clone())
        .await
        .map_err(|err| anyhow::anyhow!("Could not enable app: {err:?}"))?;

    println!("Enabled app {app_id:?}");

    Ok(response.app)
}

pub fn conductor_config(conductor_dir: &PathBuf, admin_port: u16) -> ConductorConfig {
    let mut config = ConductorConfig::default();
    config.data_root_path = Some(conductor_dir.clone().into());
    // config.keystore = KeystoreConfig::LairServer { connection_url };

    let mut network_config = KitsuneP2pConfig::default();

    network_config.bootstrap_service = Some(url2::url2!("https://bootstrap.holo.host"));

    network_config.transport_pool.push(TransportConfig::WebRTC {
        signal_url: String::from("wss://signal.holo.host"),
    });

    config.network = network_config;

    config.admin_interfaces = Some(vec![AdminInterfaceConfig {
        driver: InterfaceDriver::Websocket { port: admin_port },
    }]);

    config
}

async fn build_conductor(
    conductor_dir: PathBuf,
    keystore: MetaLairClient,
) -> anyhow::Result<HolochainInfo> {
    let admin_port = portpicker::pick_unused_port().expect("No ports free");
    let app_port = portpicker::pick_unused_port().expect("No ports free");

    let config = conductor_config(&conductor_dir, admin_port);

    let lair_client = keystore.lair_client();

    let conductor = Conductor::builder()
        .config(config)
        // .passphrase(Some(passphrase))
        .with_keystore(keystore)
        .build()
        .await?;

    let p: either::Either<u16, AppInterfaceId> = either::Either::Left(app_port);
    conductor.clone().add_app_interface(p).await?;

    Ok(HolochainInfo {
        app_port,
        admin_port,
        lair_client,
    })
}
