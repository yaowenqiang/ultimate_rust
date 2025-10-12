use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DATA_COLLECTOR_ADDRESS: &str = "127.0.0.1:9004";

const MAGIC_NUMBER: u16 = 1234;
const VERSION_NUMBER: u16 = 1;

fn unix_now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("系统时间异常：时间倒流");
    since_the_epoch.as_secs() as u32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CollectorCommandV1 {
    SubmitData {
        collector_id: u128,
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
}

pub fn encode_v1(command: CollectorCommandV1) -> Vec<u8> {
    let json = serde_json::to_string(&command).expect("JSON 序列化失败");
    let json_bytes = json.as_bytes();

    let crc = crc32fast::hash(json_bytes);
    let payload_size = json_bytes.len() as u32;

    let timestamp = unix_now();

    let mut result = Vec::with_capacity(140);

    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&timestamp.to_be_bytes());
    result.extend_from_slice(&payload_size.to_be_bytes());
    result.extend_from_slice(json_bytes);
    result.extend_from_slice(&crc.to_be_bytes());

    result
}

pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version_number = u16::from_be_bytes([bytes[2], bytes[3]]);
    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    let payload_start = 12;
    let payload_end = payload_start + payload_size as usize;
    let payload = &bytes[payload_start..payload_end];

    let crc_start = payload_end;
    let crc = u32::from_be_bytes([
        bytes[crc_start],
        bytes[crc_start + 1],
        bytes[crc_start + 2],
        bytes[crc_start + 3],
    ]);

    assert_eq!(magic_number, MAGIC_NUMBER, "协议魔数不匹配");
    assert_eq!(version_number, VERSION_NUMBER, "协议版本号不匹配");

    let computed_crc = crc32fast::hash(payload);
    assert_eq!(crc, computed_crc, "CRC32 校验失败，数据可能已损坏");

    let command = serde_json::from_slice(payload).expect("JSON 反序列化失败");

    (timestamp, command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let command = CollectorCommandV1::SubmitData {
            collector_id: 1234,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };

        let encoded = encode_v1(command.clone());
        let (timestamp, decoded) = decode_v1(&encoded);

        assert_eq!(decoded, command, "解码后的命令与原始命令不匹配");
        assert!(timestamp > 0, "时间戳应该大于 0");
        assert!(encoded.len() > 20, "编码后的数据长度应该大于头部大小");
    }

    #[test]
    fn test_protocol_integrity() {
        let command = CollectorCommandV1::SubmitData {
            collector_id: 5678,
            total_memory: 1024,
            used_memory: 512,
            average_cpu_usage: 0.75,
        };

        let encoded = encode_v1(command);

        let (timestamp, _) = decode_v1(&encoded);
        assert!(timestamp > 0);

        let mut corrupted = encoded.clone();
        if let Some(last_byte) = corrupted.last_mut() {
            *last_byte = last_byte.wrapping_add(1);
        }

        std::panic::catch_unwind(|| {
            decode_v1(&corrupted);
        })
        .expect_err("应该检测到数据损坏");
    }
}
