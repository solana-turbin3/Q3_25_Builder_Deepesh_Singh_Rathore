use anchor_lang::prelude::*;

#[error_code]
pub enum JoinRoomError {
    #[msg("Participant already joined this room.")]
    PlayerAlreadyJoined,
    #[msg("Room Full")]
    RoomFull
}
