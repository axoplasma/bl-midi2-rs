use crate::util::BitOps;

pub mod channel_pressure;
pub mod control_change;
pub mod key_pressure;
pub mod note_off;
pub mod note_on;
pub mod pitch_bend;
pub mod program_change;

use channel_pressure::ChannelPressure;
use control_change::ControlChange;
use key_pressure::KeyPressure;
use note_off::NoteOff;
use note_on::NoteOn;
use pitch_bend::PitchBend;
use program_change::ProgramChange;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x2;

#[derive(derive_more::From, midi2_proc::Data, midi2_proc::Channeled, Debug, PartialEq, Eq)]
pub enum Midi1ChannelVoice<B: crate::buffer::Buffer> {
    ChannelPressure(ChannelPressure<B>),
    ControlChange(ControlChange<B>),
    KeyPressure(KeyPressure<B>),
    NoteOff(NoteOff<B>),
    NoteOn(NoteOn<B>),
    PitchBend(PitchBend<B>),
    ProgramChange(ProgramChange<B>),
}

impl<B: crate::buffer::Ump> crate::traits::Grouped<B> for Midi1ChannelVoice<B> {
    fn group(&self) -> crate::u4 {
        use Midi1ChannelVoice::*;
        match self {
            ChannelPressure(m) => m.group(),
            ControlChange(m) => m.group(),
            KeyPressure(m) => m.group(),
            NoteOff(m) => m.group(),
            NoteOn(m) => m.group(),
            PitchBend(m) => m.group(),
            ProgramChange(m) => m.group(),
        }
    }
    fn set_group(&mut self, group: crate::u4)
    where
        B: crate::buffer::BufferMut,
    {
        use Midi1ChannelVoice::*;
        match self {
            ChannelPressure(m) => m.set_group(group),
            ControlChange(m) => m.set_group(group),
            KeyPressure(m) => m.set_group(group),
            NoteOff(m) => m.set_group(group),
            NoteOn(m) => m.set_group(group),
            PitchBend(m) => m.set_group(group),
            ProgramChange(m) => m.set_group(group),
        }
    }
}

impl<'a, U: crate::buffer::Unit> core::convert::TryFrom<&'a [U]> for Midi1ChannelVoice<&'a [U]> {
    type Error = crate::error::Error;
    fn try_from(buffer: &'a [U]) -> Result<Self, Self::Error> {
        if buffer.len() < 1 {
            return Err(crate::error::Error::InvalidData("Slice is too short"));
        };
        let status: u8 = match <U as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                <U as crate::buffer::UnitPrivate>::specialise_buffer_u8(buffer)[0].nibble(0)
            }
            crate::buffer::UNIT_ID_U32 => {
                <U as crate::buffer::UnitPrivate>::specialise_buffer_u32(buffer)[0].nibble(2)
            }
            _ => unreachable!(),
        }
        .into();
        Ok(match status {
            channel_pressure::STATUS => ChannelPressure::try_from(buffer)?.into(),
            control_change::STATUS => ControlChange::try_from(buffer)?.into(),
            key_pressure::STATUS => KeyPressure::try_from(buffer)?.into(),
            note_off::STATUS => NoteOff::try_from(buffer)?.into(),
            note_on::STATUS => NoteOn::try_from(buffer)?.into(),
            pitch_bend::STATUS => PitchBend::try_from(buffer)?.into(),
            program_change::STATUS => ProgramChange::try_from(buffer)?.into(),
            _ => Err(crate::error::Error::InvalidData(
                "Unknown channel voice status",
            ))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{numeric_types::*, traits::Channeled};
    use pretty_assertions::assert_eq;

    #[test]
    fn channel() {
        assert_eq!(
            Midi1ChannelVoice::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }
}
