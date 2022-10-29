use crate::{
    error::Error,
    util::Truncate, 
    packet::{Packet, PacketMethods},
};
use super::super::helpers;

#[derive(
    Clone,
    Debug, 
    PartialEq, Eq,
)]
pub struct Message {
    group: ux::u4,
    song: ux::u7,
}

impl Message {
    const STATUS_CODE: u8 = 0xF3;
}

impl core::convert::TryFrom<Packet> for Message {
    type Error = Error;
    fn try_from(p: Packet) -> Result<Self, Self::Error> {
        super::validate_packet(
            &p,
            Message::STATUS_CODE,
        )?;
        Ok(Message {
            group: helpers::group_from_packet(&p),
            song: p.octet(2).truncate(),
        })
    }
}

impl From<Message> for Packet {
    fn from(m: Message) -> Self {
        let mut p = Packet::new();
        super::write_data_to_packet(
            &mut p, 
            m.group, 
            Message::STATUS_CODE, 
            Some(m.song),
            None
        );
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::message_traits_test;
    
    message_traits_test!(Message);

    #[test]
    fn deserialize() {
        assert_eq!(
            Message::try_from(Packet::from_data(&[0x17F3_3000])),
            Ok(Message {
                group: ux::u4::new(0x7),
                song: ux::u7::new(0x30),
            })
        );
    }

    #[test]
    fn serialize() {
        assert_eq!(
            Packet::from(Message {
                group: ux::u4::new(0x5),
                song: ux::u7::new(0x01),
            }),
            Packet::from_data(&[0x15F3_0100]),
        );
    }
}
