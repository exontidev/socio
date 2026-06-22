#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
#[repr(u16)]
pub enum NotifyCode {
    RoomJoined = 4200,
    MessageRequestSent = 4201,
    OwnMessageSuccessfullyBroadcasted = 4205,
    RoomLeftGracefully = 4202,
    AllRoomsLeftGracefully = 4203,
}

#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[repr(u16)]
pub enum WebSocketError {
    NoRoomFound = 4402,
    InvalidRequest = 4403,
    RoomLimitReached = 4404,
    NonUTF8Request = 4401,
    RoomIsEmpty = 4405,
    UserIsntInAnyRoom = 4406,
    UserIsNotConnectedToGivenRoom = 4408,
    ActionDoesNotExist = 4407,
}
