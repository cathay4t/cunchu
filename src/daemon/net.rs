// Copyright (C) 2019 Gris Ge
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// Author: Gris Ge <cnfourt@gmail.com>
//
use pnet::datalink;
use std::vec::Vec;

fn get_local_nic_ips() -> Vec<String> {
    let mut ips = Vec::new();
    for iface in datalink::interfaces() {
        for ip in iface.ips {
            ips.push(ip.to_string().split('/').next().unwrap().to_string());
        }
    }
    ips
}

pub fn get_peer_node_ips(all_nodes: &Vec<String>) -> Vec<String> {
    let mut ret:Vec<String> = Vec::new();
    let local_ips = get_local_nic_ips();

    for node in all_nodes {
        if ! local_ips.contains(&node) {
            ret.push(node.clone())
        }
    }
    ret
}
