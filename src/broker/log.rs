use crate::broker::error::BrokerError;

// building block of logs
pub struct Record {
    offset: u64,
    payload: Vec<u8>,
}

impl Record {
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn create_record(next_offset: u64, record: &[u8]) -> Record {
        Record {
            offset: next_offset,
            payload: record.to_vec(),
        }
    }
}

// set of multiple records
#[derive(Default)]
pub struct Log {
    records: Vec<Record>,
    next_offset: u64,
}

impl Log {
    // log instance constructor
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append_record(&mut self, record: &[u8]) -> u64 {
        let offset = self.next_offset;
        let record = Record::create_record(offset, record);
        self.records.push(record);
        self.next_offset += 1;
        offset
    }

    pub fn read_record(&self, offset: u64) -> Result<&Record, BrokerError> {
        let idx = offset as usize;
        self.records.get(idx).ok_or(BrokerError::OffsetOutOfRange)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // new log has next_offset zero default
    fn first_record_gets_offset_zero() {
        let mut log = Log::new();
        assert_eq!(log.append_record(&[1]), 0);
    }

    #[test]
    fn invalid_offset_returns_error() {
        let log = Log::new();
        assert!(matches!(
            log.read_record(0),
            Err(BrokerError::OffsetOutOfRange)
        ));
    }

    #[test]
    fn test_log_roundtrip_offsets() {
        let mut log = Log::new();
        let offset1 = log.append_record(&[1]);
        let offset2 = log.append_record(&[2, 3]);
        assert_eq!(log.read_record(offset1).unwrap().offset(), 0);
        assert_eq!(log.read_record(offset2).unwrap().offset(), 1);
        assert_eq!(log.read_record(offset1).unwrap().payload(), &[1]);
        assert_eq!(log.read_record(offset2).unwrap().payload(), &[2, 3]);
    }
}
