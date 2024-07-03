// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn visualization_msgs__msg__InteractiveMarkerInit__init(msg: *mut InteractiveMarkerInit) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerInit__fini(msg: *mut InteractiveMarkerInit);
    fn visualization_msgs__msg__InteractiveMarkerInit__are_equal(lhs: *const InteractiveMarkerInit, rhs: *const InteractiveMarkerInit) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerInit__Sequence__init(msg: *mut InteractiveMarkerInitSeqRaw, size: usize) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerInit__Sequence__fini(msg: *mut InteractiveMarkerInitSeqRaw);
    fn visualization_msgs__msg__InteractiveMarkerInit__Sequence__are_equal(lhs: *const InteractiveMarkerInitSeqRaw, rhs: *const InteractiveMarkerInitSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarkerInit() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerInit {
    pub server_id: crate::msg::RosString<0>,
    pub seq_num: u64,
    pub markers: InteractiveMarkerSeq<0>,
}

impl InteractiveMarkerInit {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarkerInit__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for InteractiveMarkerInit {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__InteractiveMarkerInit__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct InteractiveMarkerInitSeqRaw {
    data: *mut InteractiveMarkerInit,
    size: size_t,
    capacity: size_t,
}

/// Sequence of InteractiveMarkerInit.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerInitSeq<const N: usize> {
    data: *mut InteractiveMarkerInit,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> InteractiveMarkerInitSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: InteractiveMarkerInitSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarkerInit__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: InteractiveMarkerInitSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[InteractiveMarkerInit] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [InteractiveMarkerInit] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, InteractiveMarkerInit> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, InteractiveMarkerInit> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for InteractiveMarkerInitSeq<N> {
    fn drop(&mut self) {
        let mut msg = InteractiveMarkerInitSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { visualization_msgs__msg__InteractiveMarkerInit__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for InteractiveMarkerInitSeq<N> {}
unsafe impl<const N: usize> Sync for InteractiveMarkerInitSeq<N> {}


impl TypeSupport for InteractiveMarkerInit {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarkerInit()
        }
    }
}

impl PartialEq for InteractiveMarkerInit {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            visualization_msgs__msg__InteractiveMarkerInit__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for InteractiveMarkerInitSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = InteractiveMarkerInitSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = InteractiveMarkerInitSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            visualization_msgs__msg__InteractiveMarkerInit__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

