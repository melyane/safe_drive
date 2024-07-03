// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn composition_interfaces__srv__LoadNode_Request__init(msg: *mut LoadNodeRequest) -> bool;
    fn composition_interfaces__srv__LoadNode_Request__fini(msg: *mut LoadNodeRequest);
    fn composition_interfaces__srv__LoadNode_Request__Sequence__init(msg: *mut LoadNodeRequestSeqRaw, size: usize) -> bool;
    fn composition_interfaces__srv__LoadNode_Request__Sequence__fini(msg: *mut LoadNodeRequestSeqRaw);
    fn composition_interfaces__srv__LoadNode_Response__init(msg: *mut LoadNodeResponse) -> bool;
    fn composition_interfaces__srv__LoadNode_Response__fini(msg: *mut LoadNodeResponse);
    fn composition_interfaces__srv__LoadNode_Response__Sequence__init(msg: *mut LoadNodeResponseSeqRaw, size: usize) -> bool;
    fn composition_interfaces__srv__LoadNode_Response__Sequence__fini(msg: *mut LoadNodeResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__LoadNode() -> *const rcl::rosidl_service_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__LoadNode_Request() -> *const rcl::rosidl_message_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__LoadNode_Response() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct LoadNodeRequest {
    pub package_name: crate::msg::RosString<0>,
    pub plugin_name: crate::msg::RosString<0>,
    pub node_name: crate::msg::RosString<0>,
    pub node_namespace: crate::msg::RosString<0>,
    pub log_level: u8,
    pub remap_rules: crate::msg::RosStringSeq<0, 0>,
    pub parameters: rcl_interfaces::msg::ParameterSeq<0>,
    pub extra_arguments: rcl_interfaces::msg::ParameterSeq<0>,
}

#[repr(C)]
#[derive(Debug)]
pub struct LoadNodeResponse {
    pub success: bool,
    pub error_message: crate::msg::RosString<0>,
    pub full_node_name: crate::msg::RosString<0>,
    pub unique_id: u64,
}

impl LoadNodeRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__LoadNode_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for LoadNodeRequest {
    fn drop(&mut self) {
        unsafe { composition_interfaces__srv__LoadNode_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct LoadNodeRequestSeqRaw {
    data: *mut LoadNodeRequest,
    size: size_t,
    capacity: size_t,
}

/// Sequence of LoadNodeRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct LoadNodeRequestSeq<const N: usize> {
    data: *mut LoadNodeRequest,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> LoadNodeRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: LoadNodeRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__LoadNode_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: LoadNodeRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[LoadNodeRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [LoadNodeRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, LoadNodeRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, LoadNodeRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for LoadNodeRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = LoadNodeRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { composition_interfaces__srv__LoadNode_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for LoadNodeRequestSeq<N> {}
unsafe impl<const N: usize> Sync for LoadNodeRequestSeq<N> {}


impl LoadNodeResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__LoadNode_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for LoadNodeResponse {
    fn drop(&mut self) {
        unsafe { composition_interfaces__srv__LoadNode_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct LoadNodeResponseSeqRaw {
    data: *mut LoadNodeResponse,
    size: size_t,
    capacity: size_t,
}

/// Sequence of LoadNodeResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct LoadNodeResponseSeq<const N: usize> {
    data: *mut LoadNodeResponse,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> LoadNodeResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: LoadNodeResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { composition_interfaces__srv__LoadNode_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: LoadNodeResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[LoadNodeResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [LoadNodeResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, LoadNodeResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, LoadNodeResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for LoadNodeResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = LoadNodeResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { composition_interfaces__srv__LoadNode_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for LoadNodeResponseSeq<N> {}
unsafe impl<const N: usize> Sync for LoadNodeResponseSeq<N> {}


pub struct LoadNode;

impl ServiceMsg for LoadNode {
    type Request = LoadNodeRequest;
    type Response = LoadNodeResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__LoadNode()
        }
    }
}

impl TypeSupport for LoadNodeRequest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__LoadNode_Request()
        }
    }
}

impl TypeSupport for LoadNodeResponse {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__LoadNode_Response()
        }
    }
}

