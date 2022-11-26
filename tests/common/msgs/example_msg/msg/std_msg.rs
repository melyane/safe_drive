// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use safe_drive::msg::*;
use safe_drive::rcl;
use safe_drive::msg::common_interfaces::*;
pub const XX: i32 = 20; // constant
pub const INITIALIZING_VEHICLE: &[u8] = b"InitializingVehicle a\"a\0";
pub const WAITING_FOR_ROUTE: &[u8] = b"WaitingForRoute\0";
pub const PLANNING: &[u8] = b"Planning\0";

extern "C" {
    fn example_msg__msg__StdMsg__init(msg: *mut StdMsg) -> bool;
    fn example_msg__msg__StdMsg__fini(msg: *mut StdMsg);
    fn example_msg__msg__StdMsg__are_equal(lhs: *const StdMsg, rhs: *const StdMsg) -> bool;
    fn example_msg__msg__StdMsg__Sequence__init(msg: *mut StdMsgSeqRaw, size: usize) -> bool;
    fn example_msg__msg__StdMsg__Sequence__fini(msg: *mut StdMsgSeqRaw);
    fn example_msg__msg__StdMsg__Sequence__are_equal(lhs: *const StdMsgSeqRaw, rhs: *const StdMsgSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__msg__StdMsg() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct StdMsg {
    pub a: bool,
    pub b: i8,
    pub c: u8,
    pub d: i16,
    pub e: u16,
    pub f: i32,
    pub g: u32,
    pub h: i64,
    pub i: u64,
    pub j: f32,
    pub k: f64,
    pub l: safe_drive::msg::RosString<0>,
    pub o: safe_drive::msg::I32Seq<0>,
    pub p: [i32; 10],
    pub limited: safe_drive::msg::I32Seq<5>,
    pub q: std_msgs::msg::Bool,
    pub r: std_msgs::msg::Byte,
    pub s: std_msgs::msg::ByteMultiArray,
    pub t: std_msgs::msg::Char,
    pub u: std_msgs::msg::ColorRGBA,
    pub w: std_msgs::msg::Empty,
    pub x: std_msgs::msg::Float32,
    pub y: std_msgs::msg::Float32MultiArray,
    pub z: std_msgs::msg::Float64,
    pub aa: std_msgs::msg::Float64MultiArray,
    pub bb: std_msgs::msg::Header,
    pub cc: std_msgs::msg::Int16,
    pub dd: std_msgs::msg::Int16MultiArray,
    pub ee: std_msgs::msg::Int32,
    pub ff: std_msgs::msg::Int32MultiArray,
    pub gg: std_msgs::msg::Int64,
    pub hh: std_msgs::msg::Int64MultiArray,
    pub ii: std_msgs::msg::Int8,
    pub jj: std_msgs::msg::Int8MultiArray,
    pub kk: std_msgs::msg::MultiArrayDimension,
    pub ll: std_msgs::msg::MultiArrayLayout,
    pub mm: std_msgs::msg::String,
    pub oo: std_msgs::msg::UInt16,
    pub pp: std_msgs::msg::UInt16MultiArray,
    pub qq: std_msgs::msg::UInt32,
    pub rr: std_msgs::msg::UInt32MultiArray,
    pub ss: std_msgs::msg::UInt64,
    pub tt: std_msgs::msg::UInt64MultiArray,
    pub uu: std_msgs::msg::UInt8,
    pub vv: std_msgs::msg::UInt8MultiArray,
    pub ww: i32, // default
}

impl StdMsg {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__msg__StdMsg__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for StdMsg {
    fn drop(&mut self) {
        unsafe { example_msg__msg__StdMsg__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct StdMsgSeqRaw {
    data: *mut StdMsg,
    size: usize,
    capacity: usize,
}

/// Sequence of StdMsg.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct StdMsgSeq<const N: usize> {
    data: *mut StdMsg,
    size: usize,
    capacity: usize,
}

impl<const N: usize> StdMsgSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: StdMsgSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__msg__StdMsg__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: StdMsgSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[StdMsg] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [StdMsg] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, StdMsg> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, StdMsg> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for StdMsgSeq<N> {
    fn drop(&mut self) {
        let mut msg = StdMsgSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { example_msg__msg__StdMsg__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for StdMsgSeq<N> {}
unsafe impl<const N: usize> Sync for StdMsgSeq<N> {}


impl TypeSupport for StdMsg {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__msg__StdMsg()
        }
    }
}

impl PartialEq for StdMsg {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            example_msg__msg__StdMsg__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for StdMsgSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = StdMsgSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = StdMsgSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            example_msg__msg__StdMsg__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

