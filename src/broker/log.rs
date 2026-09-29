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
pub struct Log {
    records: Vec<Record>,
    next_offset: u64,
}

impl Log {
    // log instance constructor
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            next_offset: 0,
        }
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
