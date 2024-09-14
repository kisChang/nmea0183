use crate::datetime::Time;
use crate::{common, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct HPR {
    pub time: Time,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub qf: u8,     // 解状态
    pub no_sat: u8, // 卫星号
    pub age: f32,   // 差分龄期 dd.dd

    pub stn_id: u8, // 基准站 ID xxxx (暂不支持)
}

impl HPR {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let time = Time::parse_from_hhmmss(fields.next())?;
        let heading = common::parse_f32(fields.next())?;
        let pitch = common::parse_f32(fields.next())?;
        let roll = common::parse_f32(fields.next())?;
        let qf = common::parse_u8(fields.next())?;
        let no_sat = common::parse_u8(fields.next())?;
        let age = common::parse_f32(fields.next())?;

        fields.next();

        if let (Some(time), Some(heading), Some(pitch), Some(roll), Some(qf), Some(no_sat), Some(age)) = (time, heading, pitch, roll, qf, no_sat, age) {
            Ok(Some(HPR {
                time,
                heading,
                pitch,
                roll,
                qf,
                no_sat,
                age,
                stn_id: 0,
            }))
        } else {
            Ok(None)
        }
    }
}