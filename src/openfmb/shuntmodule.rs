/// Shunt compensator system
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntSystem {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalSwitchingCapabilityKind {
    #[prost(enumeration="SwitchingCapabilityKind", tag="1")]
    pub value: i32,
}
/// <<abstract>> Enumerated status (ENS)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsSwitchingCapabilityKind {
    /// If true, 'q.operatorBlocked'=true, and the process value is no longer updated.
    #[prost(message, optional, tag="1")]
    pub blk_ena: ::std::option::Option<bool>,
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="2")]
    pub q: ::std::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="SwitchingCapabilityKind", tag="3")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="4")]
    pub t: ::std::option::Option<super::commonmodule::Timestamp>,
}
/// LN: Power shunt   Name: YPSH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntEventAndStatusYpsh {
    /// (controllable) If true, 'close' action has been blocked; can be set from another logical node. 
    /// Operating capability 'ShOpCap' does not reflect the blocked closing.
    #[prost(message, optional, tag="1")]
    pub blk_cls: ::std::option::Option<super::commonmodule::ControlSpc>,
    /// (controllable) If true, 'open' action has been blocked; can be set from another logical node.
    /// Operating capability 'ShOpCap' does not reflect the blocked opening.
    #[prost(message, optional, tag="2")]
    pub blk_opn: ::std::option::Option<super::commonmodule::ControlSpc>,
    /// (controllable) Position of the switch of power shunt.
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Physical capability of the switch of the power shunt to operate.
    #[prost(message, optional, tag="4")]
    pub sh_op_cap: ::std::option::Option<EnsSwitchingCapabilityKind>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntPoint {
    /// Regulator control
    #[prost(message, optional, tag="1")]
    pub control: ::std::option::Option<ShuntEventAndStatusYpsh>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::std::option::Option<super::commonmodule::Timestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<ShuntPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntControlScheduleFsch {
    /// Control value in CSG type
    #[prost(message, optional, tag="1")]
    pub val_csg: ::std::option::Option<ShuntCsg>,
}
/// Using 61850 FSCC for shunt control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub shunt_control_schedule_fsch: ::std::option::Option<ShuntControlScheduleFsch>,
}
/// Regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub shunt_control_fscc: ::std::option::Option<ShuntControlFscc>,
}
/// Shunt control profile.  Instructs an end device (or an end device group) to perform a specified
/// action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub shunt_control: ::std::option::Option<ShuntControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub shunt_system: ::std::option::Option<ShuntSystem>,
}
/// OpenFMB specialization for shunt discrete control:
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntDiscreteControlZcap {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub pos: ::std::option::Option<super::commonmodule::ControlDpc>,
}
/// Shunt discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub shunt_discrete_control_zcap: ::std::option::Option<ShuntDiscreteControlZcap>,
}
/// Shunt discrete control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub shunt_control: ::std::option::Option<ShuntDiscreteControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub shunt_system: ::std::option::Option<ShuntSystem>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntEventAndStatusPoint {
    /// Regulator control
    #[prost(message, optional, tag="1")]
    pub event_and_status: ::std::option::Option<ShuntEventAndStatusYpsh>,
}
/// OpenFMB 61850 specialization for both ShuntEventProfile and ShuntStatusProfile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntEventAndStatusZcap {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub point_status: ::std::option::Option<ShuntEventAndStatusPoint>,
}
/// Shunt event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub shunt_event_and_status_zcap: ::std::option::Option<ShuntEventAndStatusZcap>,
}
/// Shunt status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub shunt_event: ::std::option::Option<ShuntEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub shunt_system: ::std::option::Option<ShuntSystem>,
}
/// Shunt reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntReading {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment_terminal_reading: ::std::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
/// Shunt reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub shunt_reading: ::std::option::Option<ShuntReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub shunt_system: ::std::option::Option<ShuntSystem>,
}
/// Shunt status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub shunt_event_and_status_zcap: ::std::option::Option<ShuntEventAndStatusZcap>,
}
/// Shunt status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuntStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub shunt_status: ::std::option::Option<ShuntStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub shunt_system: ::std::option::Option<ShuntSystem>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SwitchingCapabilityKind {
    /// MISSING DOCUMENTATION!!!
    None = 0,
    /// Open
    Open = 1,
    /// Close
    Close = 2,
    /// Open and Close
    OpenAndClose = 3,
}
