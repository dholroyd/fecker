
struct Group {
}
impl Group {
    pub fn merge(&mut self, sequence_nubmer: u16, payload: &[u8]) {

    }
}

struct RtpFlow {
}
impl RtpFlow {
    fn emit(&mut self, sequence_number:u16, ) {

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
        let group = self.get_col_group(sequence_number);
        group.merge(sequence_number, &payload[..]);
        self.media_flow.emit(sequence_number, payload);
        if group.is_complete(sequence_number) {
            self.col_fec_flow.emit(group.sequence_number(), group.into_payload())
        }
    }

    fn get_col_group(&mut self, sequence_number: u16) -> &mut Group {
        let num = self.group_num(sequence_number as usize);
    }

    fn group_num(&self, sequence_number: usize) -> usize {
        sequence_number % self.cols + sequence_number / self.rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_num() {
        let sess = FecSession::new(4, 5);
        assert_eq!(0, sess.group_num(0));
        assert_eq!(4, sess.group_num(19));
        assert_eq!(5, sess.group_num(19));
    }
}
