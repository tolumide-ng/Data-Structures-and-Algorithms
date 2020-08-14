#[derive(Debug, Clone, Copy)]
pub struct PacketInfo {
    arrival_time: usize,
    processing_time: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct NetworkInfo {
    buffer_size: usize,
    exepected_packets: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct EngagedPacket {
    arrival_time: usize,
    start_time: i8,
    finish_time: i8,
}

fn network_packets(
    network_info: NetworkInfo,
    packet_info: Option<Vec<PacketInfo>>,
) -> Option<Vec<i8>> {
    let mut output_store: Vec<EngagedPacket> = vec![];

    if packet_info.clone().unwrap().len() < 1 {
        return None;
    }

    for packet in packet_info.unwrap().iter() {
        if output_store.len() == 0 {
            output_store.push(EngagedPacket {
                arrival_time: packet.arrival_time,
                start_time: packet.arrival_time as i8,
                finish_time: packet.processing_time as i8,
            });
        } else {
            if packet.arrival_time == (output_store[output_store.len() - 1].finish_time) as usize {
                let start = packet.arrival_time
                    + (output_store[output_store.len() - 1].finish_time) as usize;
                output_store.push(EngagedPacket {
                    arrival_time: packet.arrival_time,
                    start_time: packet.arrival_time as i8,
                    finish_time: (packet.processing_time + start) as i8,
                });
            } else if output_store.len() + 1 > network_info.buffer_size {
                output_store.push(EngagedPacket {
                    arrival_time: packet.arrival_time,
                    start_time: -1,
                    finish_time: -1,
                })
            } else {
                output_store.push(EngagedPacket {
                    arrival_time: packet.arrival_time,
                    start_time: packet.arrival_time as i8,
                    finish_time: (packet.arrival_time + packet.processing_time) as i8,
                })
            }
        }
    }
    let values: Vec<i8> = output_store.iter().map(|item| item.start_time).collect();
    return Some(values);
}

pub fn execute() {
    let net_pack = NetworkInfo {
        buffer_size: 1,
        exepected_packets: 2,
    };
    let all_packets = Some(vec![
        PacketInfo {
            arrival_time: 0,
            processing_time: 1,
        },
        PacketInfo {
            arrival_time: 1,
            processing_time: 1,
        },
    ]);
    let results = network_packets(net_pack, all_packets);
    println!("Result >>>>> {:?}", results);
}
