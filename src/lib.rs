
struct Group {
}
impl Group {
    pub fn merge(&mut self, sequence_number: u16, payload: &[u8]) {
        unimplemented!()
    }

    pub fn is_complete(&self, sequence_number: u16) -> bool {
        unimplemented!()
    }

    pub fn sequence_number(&self) -> u16 {
        unimplemented!()
    }

    pub fn into_payload(&self) -> Vec<u8> {
        unimplemented!()
    }
}

struct RtpFlow {
}
impl RtpFlow {
    fn emit(&self, sequence_number:u16, payload: Vec<u8>) {

    }
}


pub struct FecSession {
    rows: usize,
    cols: usize,
    media_flow: RtpFlow,
    col_fec_flow: RtpFlow,
}
impl FecSession {
    pub fn new(rows: usize, cols: usize) -> FecSession {
        FecSession {
            rows,
            cols,
            media_flow: RtpFlow {},
            col_fec_flow: RtpFlow {},
        }
    }

    fn push(&mut self, sequence_number: u16, payload: Vec<u8>) {
        {
            let group = self.get_col_group_mut(sequence_number);
            group.merge(sequence_number, &payload[..]);
        }
        self.media_flow.emit(sequence_number, payload);
        let group = self.get_col_group(sequence_number);
        if group.is_complete(sequence_number) {
            self.col_fec_flow.emit(group.sequence_number(), group.into_payload())
        }
    }

    fn get_col_group_mut(&mut self, sequence_number: u16) -> &mut Group {
        let num = self.group_num(sequence_number as usize);
        unimplemented!()
    }

    fn get_col_group(&self, sequence_number: u16) -> &Group {
        let num = self.group_num(sequence_number as usize);
        unimplemented!()
    }

    fn group_num(&self, sequence_number: usize) -> usize {
        sequence_number % self.cols + (sequence_number / (self.cols * self.rows)) * self.cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_num() {
        let sess = FecSession::new(5, 4);
        assert_eq!(sess.group_num(0), 0);
        assert_eq!(sess.group_num(19), 3);
        assert_eq!(sess.group_num(20), 4);
    }
}
