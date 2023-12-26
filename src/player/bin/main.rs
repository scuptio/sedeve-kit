use std::collections::HashMap;
use std::fs::read_to_string;
use std::net::SocketAddr;

use clap::Parser;
use scupt_net::notifier::Notifier;
use scupt_util::node_id::NID;
use scupt_util::res::Res;
use scupt_util::res_of::{res_io, res_parse};
use serde::{Deserialize, Serialize};

use sedeve_kit::player::action_incoming_factory::ActionIncomingFactory;
use sedeve_kit::player::dtm_player::DTMPlayer;
use sedeve_kit::trace_gen::trace_db::TraceDB;

/// action definition to Rust code template
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// server node id
    #[arg(short, long)]
    pub conf_path: String
}

#[derive(
    Clone,
    Serialize,
    Debug,
    Deserialize,
)]
pub struct PlayerConfig {
    pub db_path:String,
    pub player_id:NID,
    pub player_address:String,
    pub node_peer:HashMap<NID, String>,
}

fn player_gut(db_path:String, player_id:NID, player_address:SocketAddr, peers:HashMap<NID, SocketAddr>) -> Res<()> {
    let db = TraceDB::new(db_path)?;

    let vec = db.read_trace()?;
    for s in vec {
        let notifier = Notifier::new();
        let n = notifier.clone();
        let incoming = ActionIncomingFactory::action_incoming_from_string(s)?;
        let f_done = move || {
            n.task_notify_all();
        };
        DTMPlayer::run_trace(
            player_id,
            player_address.clone(),
            peers.clone(),
            incoming,
            notifier.clone(),
            Default::default(),
            f_done
        )?;
    }
    Ok(())
}

fn player_run(conf:String) -> Res<()> {
    let s =  res_io(read_to_string(conf))?;
    let c :PlayerConfig = res_parse(serde_json::from_str(s.as_str()))?;
    let player_addr :SocketAddr = res_parse(c.player_address.parse())?;
    let mut peers = HashMap::new();
    for (n, s_addr) in c.node_peer.iter() {
        let a:SocketAddr = res_parse(s_addr.parse())?;
        peers.insert(n.clone(), a);
    }
    player_gut(c.db_path, c.player_id, player_addr, peers)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let conf_path: String = args.conf_path.clone();
    player_run(conf_path).unwrap();
}