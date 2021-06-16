/* automatically generated by rust-bindgen 0.58.1 */

pub const vpiIntegerVar: u32 = 25;
pub const vpiMemoryWord: u32 = 30;
pub const vpiModule: u32 = 32;
pub const vpiNet: u32 = 36;
pub const vpiNetBit: u32 = 37;
pub const vpiPort: u32 = 44;
pub const vpiPortBit: u32 = 45;
pub const vpiRealVar: u32 = 47;
pub const vpiReg: u32 = 48;
pub const vpiRegBit: u32 = 49;
pub const vpiType: u32 = 1;
pub const vpiName: u32 = 2;
pub const vpiFullName: u32 = 3;
pub const vpiSize: u32 = 4;
pub const vpiDefName: u32 = 9;
pub const vpiTimePrecision: u32 = 12;
pub const vpiArray: u32 = 28;
pub const vpiSigned: u32 = 65;
pub const vpiSimTime: u32 = 2;
pub const vpiSuppressTime: u32 = 3;
pub const vpiBinStrVal: u32 = 1;
pub const vpiIntVal: u32 = 6;
pub const vpiStringVal: u32 = 8;
pub const vpiSuppressVal: u32 = 13;
pub const vpiInertialDelay: u32 = 2;
pub const vpiForceFlag: u32 = 5;
pub const vpiReleaseFlag: u32 = 6;
pub const cbValueChange: u32 = 1;
pub const cbReadWriteSynch: u32 = 6;
pub const cbReadOnlySynch: u32 = 7;
pub const cbAfterDelay: u32 = 9;
pub const cbStartOfSimulation: u32 = 11;
pub const cbEndOfSimulation: u32 = 12;
pub type PLI_INT32 = ::std::os::raw::c_int;
pub type PLI_UINT32 = ::std::os::raw::c_uint;
pub type PLI_BYTE8 = ::std::os::raw::c_char;
#[doc = " TYPEDEFS"]
pub type vpiHandle = *mut PLI_UINT32;
#[doc = " time structure"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct t_vpi_time {
    pub type_: PLI_INT32,
    pub high: PLI_UINT32,
    pub low: PLI_UINT32,
    pub real: f64,
}
pub type p_vpi_time = *mut t_vpi_time;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct t_vpi_vecval {
    pub aval: PLI_INT32,
    pub bval: PLI_INT32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct t_vpi_strengthval {
    pub logic: PLI_INT32,
    pub s0: PLI_INT32,
    pub s1: PLI_INT32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t_vpi_value {
    pub format: PLI_INT32,
    pub value: t_vpi_value__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union t_vpi_value__bindgen_ty_1 {
    pub str_: *mut PLI_BYTE8,
    pub scalar: PLI_INT32,
    pub integer: PLI_INT32,
    pub real: f64,
    pub time: *mut t_vpi_time,
    pub vector: *mut t_vpi_vecval,
    pub strength: *mut t_vpi_strengthval,
    pub misc: *mut PLI_BYTE8,
}
impl Default for t_vpi_value__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for t_vpi_value {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type p_vpi_value = *mut t_vpi_value;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct t_cb_data {
    pub reason: PLI_INT32,
    pub cb_rtn: ::std::option::Option<unsafe extern "C" fn(arg1: *mut t_cb_data) -> PLI_INT32>,
    pub obj: vpiHandle,
    pub time: p_vpi_time,
    pub value: p_vpi_value,
    pub index: PLI_INT32,
    pub user_data: *mut PLI_BYTE8,
}
impl Default for t_cb_data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type p_cb_data = *mut t_cb_data;
extern "C" {
    #[doc = " FUNCTION DECLARATIONS"]
    pub fn vpi_register_cb(cb_data_p: p_cb_data) -> vpiHandle;
}
extern "C" {
    pub fn vpi_remove_cb(cb_obj: vpiHandle) -> PLI_INT32;
}
extern "C" {
    pub fn vpi_handle_by_name(name: *mut PLI_BYTE8, scope: vpiHandle) -> vpiHandle;
}
extern "C" {
    pub fn vpi_iterate(type_: PLI_INT32, refHandle: vpiHandle) -> vpiHandle;
}
extern "C" {
    pub fn vpi_scan(iterator: vpiHandle) -> vpiHandle;
}
extern "C" {
    pub fn vpi_get(property: PLI_INT32, object: vpiHandle) -> PLI_INT32;
}
extern "C" {
    pub fn vpi_get_str(property: PLI_INT32, object: vpiHandle) -> *mut PLI_BYTE8;
}
extern "C" {
    pub fn vpi_get_value(expr: vpiHandle, value_p: p_vpi_value);
}
extern "C" {
    pub fn vpi_put_value(
        object: vpiHandle,
        value_p: p_vpi_value,
        time_p: p_vpi_time,
        flags: PLI_INT32,
    ) -> vpiHandle;
}
extern "C" {
    pub fn vpi_get_time(object: vpiHandle, time_p: p_vpi_time);
}
extern "C" {
    pub fn vpi_printf(format: *const PLI_BYTE8, ...) -> PLI_INT32;
}
extern "C" {
    pub fn vpi_free_object(object: vpiHandle) -> PLI_INT32;
}
