use crate::uavcan::file::path::Path;
use canadensis_core::ServiceId;
use canadensis_encoding::{
    DataType, Deserialize, DeserializeError, ReadCursor, Response, Serialize, WriteCursor,
};

/// uavcan.node.ExecuteCommand version 1.0 request
#[derive(Clone)]
pub struct ExecuteCommandRequest {
    pub command: Command,
    pub parameter: heapless::Vec<u8, { Path::MAX_LENGTH as usize }>,
}

impl ExecuteCommandRequest {
    pub const SERVICE: ServiceId = ServiceId::from_truncating(435);
}

impl Default for ExecuteCommandRequest {
    fn default() -> Self {
        ExecuteCommandRequest {
            command: Command::Other(0),
            parameter: Default::default(),
        }
    }
}

#[derive(Clone)]
pub enum Command {
    Restart,
    PowerOff,
    BeginSoftwareUpdate,
    FactoryReset,
    EmergencyStop,
    StorePersistentStates,
    Other(u16),
}

impl From<u16> for Command {
    fn from(bits: u16) -> Self {
        match bits {
            65535 => Command::Restart,
            65534 => Command::PowerOff,
            65533 => Command::BeginSoftwareUpdate,
            65532 => Command::FactoryReset,
            65531 => Command::EmergencyStop,
            65530 => Command::StorePersistentStates,
            other => Command::Other(other),
        }
    }
}

impl From<Command> for u16 {
    fn from(command: Command) -> Self {
        match command {
            Command::Restart => 65535,
            Command::PowerOff => 65534,
            Command::BeginSoftwareUpdate => 65533,
            Command::FactoryReset => 65532,
            Command::EmergencyStop => 65531,
            Command::StorePersistentStates => 65530,
            Command::Other(bits) => bits,
        }
    }
}

impl DataType for ExecuteCommandRequest {
    const EXTENT_BYTES: Option<u32> = Some(300);
}

impl canadensis_encoding::Request for ExecuteCommandRequest {}

impl Serialize for ExecuteCommandRequest {
    fn size_bits(&self) -> usize {
        16 + 8 + 8 * self.parameter.len()
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u16(self.command.clone().into());
        cursor.write_aligned_u8(self.parameter.len() as u8);
        cursor.write_aligned_bytes(&self.parameter);
    }
}

impl Deserialize for ExecuteCommandRequest {
    fn in_bit_length_set(bit_length: usize) -> bool {
        if bit_length < 16 {
            false
        } else {
            let parameter_bytes_length = bit_length - 16;
            parameter_bytes_length % 8 == 0
                && parameter_bytes_length / 8 <= usize::from(Path::MAX_LENGTH)
        }
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.command = cursor.read_aligned_u16().into();
        self.parameter.clear();
        let parameter_length = cursor.read_aligned_u8();
        for _ in 0..parameter_length {
            self.parameter.push(cursor.read_aligned_u8()).unwrap();
        }
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = ExecuteCommandRequest::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}

/// uavcan.node.ExecuteCommand version 1.0 response
#[derive(Clone, Default)]
pub struct ExecuteCommandResponse {
    pub status: Status,
}

impl ExecuteCommandResponse {
    pub const SERVICE: ServiceId = ServiceId::from_truncating(435);
}

#[derive(Clone)]
pub enum Status {
    Success,
    Failure,
    NotAuthorized,
    BadCommand,
    BadParameter,
    BadState,
    InternalError,
    Other(u8),
}

impl Default for Status {
    fn default() -> Self {
        Status::Success
    }
}

impl From<u8> for Status {
    fn from(bits: u8) -> Self {
        match bits {
            0 => Status::Success,
            1 => Status::Failure,
            2 => Status::NotAuthorized,
            3 => Status::BadCommand,
            4 => Status::BadParameter,
            5 => Status::BadState,
            6 => Status::InternalError,
            other => Status::Other(other),
        }
    }
}

impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => 0,
            Status::Failure => 1,
            Status::NotAuthorized => 2,
            Status::BadCommand => 3,
            Status::BadParameter => 4,
            Status::BadState => 5,
            Status::InternalError => 6,
            Status::Other(other) => other,
        }
    }
}

impl DataType for ExecuteCommandResponse {
    const EXTENT_BYTES: Option<u32> = Some(48);
}

impl Response for ExecuteCommandResponse {}

impl Serialize for ExecuteCommandResponse {
    fn size_bits(&self) -> usize {
        8
    }

    fn serialize(&self, cursor: &mut WriteCursor<'_>) {
        cursor.write_aligned_u8(self.status.clone().into())
    }
}

impl Deserialize for ExecuteCommandResponse {
    fn in_bit_length_set(bit_length: usize) -> bool {
        bit_length == 8
    }

    fn deserialize_in_place(
        &mut self,
        cursor: &mut ReadCursor<'_>,
    ) -> Result<(), DeserializeError> {
        self.status = cursor.read_aligned_u8().into();
        Ok(())
    }

    fn deserialize(cursor: &mut ReadCursor<'_>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let mut value = ExecuteCommandResponse::default();
        value.deserialize_in_place(cursor)?;
        Ok(value)
    }
}
