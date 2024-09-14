use crate::modes::Status;
use crate::{common, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct ROT {
    pub rate: f32,
}

impl ROT {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let rate = common::parse_f32(fields.next())?;
        let status = if let Some(f_status) = fields.next() {
            Status::from_str(f_status)?
        } else {
            return Err("Status field is mandatory for ROT sentence!");
        };

        if let (Some(rate)) = rate {
            Ok(Some(ROT {
                rate,
            }))
        } else {
            Ok(None)
        }
    }
}