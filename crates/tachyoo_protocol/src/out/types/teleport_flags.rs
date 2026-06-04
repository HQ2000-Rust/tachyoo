use crate::out::types::Int;

bitflags::bitflags! {
    pub struct TeleportFlags: Int {
        const RELATIVE_X                = 0x0001;
        const RELATIVE_Y                = 0x0002;
        const RELATIVE_Z                = 0x0004;
        const RELATIVE_YAW              = 0x0008;
        const RELATIVE_PITCH            = 0x0010;
        const RELATIVE_VELOCITY_X       = 0x0020;
        const RELATIVE_VELOCITY_Y       = 0x0040;
        const RELATIVE_VELOCITY_Z       = 0x0080;
        const ROT_VEL_BEFORE_VEL_CHANGE = 0x0100;
    }
}
