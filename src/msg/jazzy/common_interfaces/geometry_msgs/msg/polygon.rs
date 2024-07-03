// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Polygon__init(msg: *mut Polygon) -> bool;
    fn geometry_msgs__msg__Polygon__fini(msg: *mut Polygon);
    fn geometry_msgs__msg__Polygon__are_equal(lhs: *const Polygon, rhs: *const Polygon) -> bool;
    fn geometry_msgs__msg__Polygon__Sequence__init(msg: *mut PolygonSeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__Polygon__Sequence__fini(msg: *mut PolygonSeqRaw);
    fn geometry_msgs__msg__Polygon__Sequence__are_equal(lhs: *const PolygonSeqRaw, rhs: *const PolygonSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Polygon() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Polygon {
    pub points: Point32Seq<0>,
}

impl Polygon {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Polygon__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Polygon {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Polygon__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct PolygonSeqRaw {
    data: *mut Polygon,
    size: size_t,
    capacity: size_t,
}

/// Sequence of Polygon.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PolygonSeq<const N: usize> {
    data: *mut Polygon,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> PolygonSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PolygonSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Polygon__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: PolygonSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[Polygon] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Polygon] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Polygon> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Polygon> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for PolygonSeq<N> {
    fn drop(&mut self) {
        let mut msg = PolygonSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { geometry_msgs__msg__Polygon__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PolygonSeq<N> {}
unsafe impl<const N: usize> Sync for PolygonSeq<N> {}


impl TypeSupport for Polygon {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Polygon()
        }
    }
}

impl PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            geometry_msgs__msg__Polygon__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for PolygonSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = PolygonSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = PolygonSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            geometry_msgs__msg__Polygon__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

