use scupt_util::node_id::NID;
use serde::{Deserialize, Serialize};

#[derive(
Clone,
Serialize,
Debug,
Deserialize,
)]
pub struct NodeAddr {
    pub nid: NID,
    pub addr: String,
}

#[derive(
Clone,
Serialize,
Debug,
Deserialize,
)]
pub struct PlayerConf {
    pub log_level:String,
    pub trace_db_path: String,
    pub player_addr: NodeAddr,
    pub peer_addr: Vec<NodeAddr>,
}


#[cfg(test)]
mod test {
    use crate::player_conf::{NodeAddr, PlayerConf};

    #[test]
    fn test_conf() {
        let mut node_peer = vec![];
        for i in 2..=3 {
            node_peer.push(NodeAddr { nid: i, addr: format!("127.0.0.1:{}", 8000 + i) });
        }
        let conf = PlayerConf {
            log_level: "".to_string(),
            trace_db_path: "/tmp/trace.db".to_string(),
            player_addr: NodeAddr {
                nid: 1,
                addr: "127.0.0.1:8000".to_string(),
            },
            peer_addr: node_peer,
        };

        let s = toml::to_string(&conf).unwrap();
        println!("{}", s)
    }
}