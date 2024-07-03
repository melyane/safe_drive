// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Point__init(msg: *mut Point) -> bool;
    fn geometry_msgs__msg__Point__fini(msg: *mut Point);
    fn geometry_msgs__msg__Point__are_equal(lhs: *const Point, rhs: *const Point) -> bool;
    fn geometry_msgs__msg__Point__Sequence__init(msg: *mut PointSeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__Point__Sequence__fini(msg: *mut PointSeqRaw);
    fn geometry_msgs__msg__Point__Sequence__are_equal(lhs: *const PointSeqRaw, rhs: *const PointSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Point() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Point__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Point__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct PointSeqRaw {
    data: *mut Point,
    size: size_t,
    capacity: size_t,
}

/// Sequence of Point.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PointSeq<const N: usize> {
    data: *mut Point,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> PointSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PointSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Point__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: PointSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[Point] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Point] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Point> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Point> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for PointSeq<N> {
    fn drop(&mut self) {
        let mut msg = PointSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { geometry_msgs__msg__Point__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PointSeq<N> {}
unsafe impl<const N: usize> Sync for PointSeq<N> {}


impl TypeSupport for Point {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Point()
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            geometry_msgs__msg__Point__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for PointSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = PointSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = PointSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            geometry_msgs__msg__Point__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

