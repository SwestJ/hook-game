use super::*;

pub const PLAYER_IDLING: Circle = Circle { radius: Radius(10.0), color: YELLOW };
pub const PLAYER_MOVING: Circle = Circle { radius: Radius(9.0), color: BLUE };
pub const PLAYER_SHOOTING: Circle = Circle { radius: Radius(10.0), color: BLUE };