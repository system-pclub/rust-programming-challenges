pub trait Decode<'a, T> {
    fn decode_from(&'a mut self) -> T;
}

pub struct MQTTFrame<'a> {
    pub payload: &'a Vec<u8>,
}

pub struct MQTTFrameDecoder<'a> {
    pub payload: &'a mut Vec<u8>,
}

impl<'a> Decode<'a, MQTTFrame<'a>> for MQTTFrameDecoder<'a> {
    fn decode_from(&'a mut self) -> MQTTFrame<'a> {
        MQTTFrame {
            payload: self.payload,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v = vec![12, 13];
        let mut decoder = MQTTFrameDecoder { payload: &mut v };
        let frame = decoder.decode_from();
        assert_eq!(format!("{:?}", frame.payload), "[12, 13]");
    }
}