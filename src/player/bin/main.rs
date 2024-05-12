use std::collections::HashMap;
use std::fs::read_to_string;
use std::net::SocketAddr;

use clap::Parser;
use scupt_net::notifier::Notifier;
use scupt_util::logger::logger_setup;
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use scupt_util::res_of::{res_io, res_parse};
use toml;
use tracing::debug;

use player_conf::PlayerConf;
use sedeve_kit::dtm::action_incoming_factory::ActionIncomingFactory;
use sedeve_kit::dtm::dtm_player::DTMPlayer;
use sedeve_kit::trace::trace_db::TraceDB;

mod player_conf;

/// action definition to Rust code template
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// player config path
    #[arg(short, long)]
    pub conf_path: String,
}

fn player_gut(db_path: String, player_id: NID, player_address: SocketAddr, peers: HashMap<NID, SocketAddr>) -> Res<()> {
    let db = TraceDB::new(db_path)?;

    let vec = db.read_trace()?;
    for (i, s) in vec.iter().enumerate() {
        let notifier = Notifier::new();
        let n = notifier.clone();
        let incoming = ActionIncomingFactory::action_incoming_from_string(s.clone())?;
        let f_done = move || {
            n.task_notify_all();
        };
        debug!("DTM player run trace {} {}", i + 1, s);
        DTMPlayer::run_trace(
            player_id,
            player_address.clone(),
            peers.clone(),
            incoming,
            notifier.clone(),
            Default::default(),
            f_done,
        )?;
    }
    Ok(())
}

fn player_run(conf: String) -> Res<()> {
    let s = res_io(read_to_string(conf))?;
    let c: PlayerConf = toml::from_str(s.as_str()).unwrap();
    let player_addr: SocketAddr = res_parse(c.player_addr.addr.parse())?;
    let mut peers = HashMap::new();


    logger_setup(c.log_level.as_str());

    for addr in c.peer_addr.iter() {
        let a: SocketAddr = res_parse(addr.addr.parse())?;
        peers.insert(addr.nid.clone(), a);
    }


    player_gut(c.trace_db_path, c.player_addr.nid, player_addr, peers)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let conf_path: String = args.conf_path.clone();
    player_run(conf_path).unwrap();
}