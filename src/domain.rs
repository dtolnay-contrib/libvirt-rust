/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <http://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

#![allow(improper_ctypes)]

extern crate libc;

use std::ffi::CStr;
use std::{str, ptr, mem};

use connect::sys::virConnectPtr;
use stream::sys::virStreamPtr;
use typedparam::sys::{virTypedParameterPtr, virTypedParameter};

use connect::Connect;
use error::Error;
use stream::Stream;

pub mod sys {
    extern crate libc;

    use typedparam::sys::virTypedParameterPtr;

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virDomain {}

    #[allow(non_camel_case_types)]
    pub type virDomainPtr = *mut virDomain;

    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[repr(C)]
    pub struct virDomainInfo {
        pub state: libc::c_ulong,
        pub maxMem: libc::c_ulong,
        pub memory: libc::c_ulong,
        pub nrVirtCpu: libc::c_uint,
        pub cpuTime: libc::c_ulong,
    }

    impl virDomainInfo {
        pub fn new() -> virDomainInfo {
            virDomainInfo {
                state: 0,
                maxMem: 0,
                memory: 0,
                nrVirtCpu: 0,
                cpuTime: 0,
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub type virDomainInfoPtr = *mut virDomainInfo;

    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[repr(C)]
    pub struct virDomainStatsRecord {
        pub dom: virDomainPtr,
        pub params: virTypedParameterPtr,
        pub nparams: libc::c_uint,
    }

    #[allow(non_camel_case_types)]
    pub type virDomainStatsRecordPtr = *mut virDomainStatsRecord;

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virDomainBlockInfo {
        pub capacity: libc::c_ulonglong,
        pub allocation: libc::c_ulonglong,
        pub physical: libc::c_ulonglong,
    }

    impl virDomainBlockInfo {
        pub fn new() -> virDomainBlockInfo {
            virDomainBlockInfo {
                capacity: 0,
                allocation: 0,
                physical: 0,
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub type virDomainBlockInfoPtr = *mut virDomainBlockInfo;

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virDomainInterfaceStats {
        pub rx_bytes: libc::c_longlong,
        pub rx_packets: libc::c_longlong,
        pub rx_errs: libc::c_longlong,
        pub rx_drop: libc::c_longlong,
        pub tx_bytes: libc::c_longlong,
        pub tx_packets: libc::c_longlong,
        pub tx_errs: libc::c_longlong,
        pub tx_drop: libc::c_longlong,
    }

    impl virDomainInterfaceStats {
        pub fn new() -> virDomainInterfaceStats {
            virDomainInterfaceStats {
                rx_bytes: 0,
                rx_packets: 0,
                rx_errs: 0,
                rx_drop: 0,
                tx_bytes: 0,
                tx_packets: 0,
                tx_errs: 0,
                tx_drop: 0,
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub type virDomainInterfaceStatsPtr = *mut virDomainInterfaceStats;

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct virDomainMemoryStats {
        pub tag: libc::c_int,
        pub val: libc::c_ulonglong,
    }

    impl virDomainMemoryStats {
        pub fn new() -> virDomainMemoryStats {
            virDomainMemoryStats { tag: 0, val: 0 }
        }
    }

    #[allow(non_camel_case_types)]
    pub type virDomainMemoryStatsPtr = *mut virDomainMemoryStats;
}

#[link(name = "virt")]
extern "C" {
    fn virDomainLookupByID(c: virConnectPtr, id: libc::c_int) -> sys::virDomainPtr;
    fn virDomainLookupByName(c: virConnectPtr, id: *const libc::c_char) -> sys::virDomainPtr;
    fn virDomainLookupByUUIDString(c: virConnectPtr,
                                   uuid: *const libc::c_char)
                                   -> sys::virDomainPtr;
    fn virDomainCreate(c: virConnectPtr) -> sys::virDomainPtr;
    fn virDomainCreateXML(c: virConnectPtr,
                          xml: *const libc::c_char,
                          flags: libc::c_uint)
                          -> sys::virDomainPtr;
    fn virDomainDefineXML(c: virConnectPtr, xml: *const libc::c_char) -> sys::virDomainPtr;
    fn virDomainDefineXMLFlags(c: virConnectPtr,
                               xml: *const libc::c_char,
                               flags: libc::c_uint)
                               -> sys::virDomainPtr;
    fn virDomainDestroy(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainDestroyFlags(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainUndefine(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainFree(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainShutdown(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainReboot(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainSuspend(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainResume(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainIsActive(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainIsUpdated(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainGetName(ptr: sys::virDomainPtr) -> *const libc::c_char;
    fn virDomainGetOSType(ptr: sys::virDomainPtr) -> *const libc::c_char;
    fn virDomainGetHostname(ptr: sys::virDomainPtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virDomainGetUUIDString(ptr: sys::virDomainPtr, uuid: *mut libc::c_char) -> libc::c_int;
    fn virDomainGetXMLDesc(ptr: sys::virDomainPtr, flags: libc::c_uint) -> *const libc::c_char;
    fn virDomainGetAutostart(ptr: sys::virDomainPtr) -> libc::c_int;
    fn virDomainSetAutostart(ptr: sys::virDomainPtr, autostart: libc::c_uint) -> libc::c_int;
    fn virDomainGetID(ptr: sys::virDomainPtr) -> libc::c_uint;
    fn virDomainSetMaxMemory(ptr: sys::virDomainPtr, memory: libc::c_ulong) -> libc::c_int;
    fn virDomainGetMaxMemory(ptr: sys::virDomainPtr) -> libc::c_ulong;
    fn virDomainGetMaxVcpus(ptr: sys::virDomainPtr) -> libc::c_ulong;
    fn virDomainSetMemory(ptr: sys::virDomainPtr, memory: libc::c_ulong) -> libc::c_int;
    fn virDomainSetMemoryFlags(ptr: sys::virDomainPtr,
                               memory: libc::c_ulong,
                               flags: libc::c_uint)
                               -> libc::c_int;
    fn virDomainSetMemoryStatsPeriod(ptr: sys::virDomainPtr,
                                     period: libc::c_int,
                                     flags: libc::c_uint)
                                     -> libc::c_int;
    fn virDomainSetVcpus(ptr: sys::virDomainPtr, vcpus: libc::c_uint) -> libc::c_int;
    fn virDomainSetVcpusFlags(ptr: sys::virDomainPtr,
                              vcpus: libc::c_uint,
                              flags: libc::c_uint)
                              -> libc::c_int;
    fn virDomainGetVcpusFlags(ptr: sys::virDomainPtr, vcpus: libc::c_uint) -> libc::c_int;
    fn virDomainRestore(c: virConnectPtr, source: *const libc::c_char) -> libc::c_int;
    fn virDomainRestoreFlags(c: virConnectPtr,
                             source: *const libc::c_char,
                             flags: libc::c_uint)
                             -> libc::c_int;
    fn virDomainGetConnect(ptr: sys::virDomainPtr) -> virConnectPtr;
    fn virDomainGetInfo(ptr: sys::virDomainPtr, ninfo: sys::virDomainInfoPtr) -> libc::c_int;
    fn virDomainMigrateSetMaxSpeed(ptr: sys::virDomainPtr,
                                   bandwidth: libc::c_ulong,
                                   flags: libc::c_uint)
                                   -> libc::c_int;
    fn virDomainMigrateGetMaxSpeed(ptr: sys::virDomainPtr,
                                   bandwidth: *mut libc::c_ulong,
                                   flags: libc::c_uint)
                                   -> libc::c_int;
    fn virDomainMigrateSetCompressionCache(ptr: sys::virDomainPtr,
                                           size: libc::c_ulong,
                                           flags: libc::c_uint)
                                           -> libc::c_int;
    fn virDomainMigrateGetCompressionCache(ptr: sys::virDomainPtr,
                                           size: *mut libc::c_ulong,
                                           flags: libc::c_uint)
                                           -> libc::c_int;
    fn virDomainMigrateSetMaxDowntime(ptr: sys::virDomainPtr,
                                      downtime: libc::c_ulong,
                                      flags: libc::c_uint)
                                      -> libc::c_int;
    fn virDomainGetTime(ptr: sys::virDomainPtr,
                        seconds: *mut libc::c_long,
                        nseconds: *mut libc::c_int,
                        flags: libc::c_uint)
                        -> libc::c_int;
    fn virDomainSetTime(ptr: sys::virDomainPtr,
                        seconds: libc::c_long,
                        nseconds: libc::c_int,
                        flags: libc::c_uint)
                        -> libc::c_int;
    fn virDomainGetBlockInfo(ptr: sys::virDomainPtr,
                             disk: *const libc::c_char,
                             info: sys::virDomainBlockInfoPtr,
                             flags: libc::c_uint)
                             -> libc::c_int;
    fn virDomainPinVcpu(ptr: sys::virDomainPtr,
                        vcpu: libc::c_uint,
                        vcpumap: *const libc::c_uchar,
                        maplen: libc::c_uint)
                        -> libc::c_int;
    fn virDomainPinVcpuFlags(ptr: sys::virDomainPtr,
                             vcpu: libc::c_uint,
                             vcpumap: *const libc::c_uchar,
                             maplen: libc::c_uint,
                             flags: libc::c_uint)
                             -> libc::c_int;
    fn virDomainPinEmulator(ptr: sys::virDomainPtr,
                            vcpumap: *const libc::c_uchar,
                            maplen: libc::c_uint,
                            flags: libc::c_uint)
                            -> libc::c_int;
    fn virDomainRename(ptr: sys::virDomainPtr,
                       new_name: *const libc::c_char,
                       flags: libc::c_uint)
                       -> libc::c_int;
    fn virDomainSetUserPassword(ptr: sys::virDomainPtr,
                                user: *const libc::c_char,
                                pass: *const libc::c_char,
                                flags: libc::c_uint)
                                -> libc::c_int;
    fn virDomainSetBlockThreshold(ptr: sys::virDomainPtr,
                                  dev: *const libc::c_char,
                                  threshold: libc::c_ulonglong,
                                  flags: libc::c_uint)
                                  -> libc::c_int;
    fn virDomainOpenGraphics(ptr: sys::virDomainPtr,
                             idx: libc::c_uint,
                             fd: libc::c_int,
                             flags: libc::c_uint)
                             -> libc::c_int;
    fn virDomainOpenGraphicsFD(ptr: sys::virDomainPtr,
                               idx: libc::c_uint,
                               flags: libc::c_uint)
                               -> libc::c_int;
    fn virDomainOpenChannel(ptr: sys::virDomainPtr,
                            name: *const libc::c_char,
                            st: virStreamPtr,
                            flags: libc::c_uint)
                            -> libc::c_int;
    fn virDomainOpenConsole(ptr: sys::virDomainPtr,
                            dev_name: *const libc::c_char,
                            st: virStreamPtr,
                            flags: libc::c_uint)
                            -> libc::c_int;
    fn virDomainInterfaceStats(ptr: sys::virDomainPtr,
                               path: *const libc::c_char,
                               stats: sys::virDomainInterfaceStatsPtr,
                               size: libc::c_uint)
                               -> libc::c_int;
    fn virDomainMemoryStats(ptr: sys::virDomainPtr,
                            stats: sys::virDomainMemoryStatsPtr,
                            nr_stats: libc::c_uint,
                            flags: libc::c_uint)
                            -> libc::c_int;
    fn virDomainSaveImageGetXMLDesc(ptr: virConnectPtr,
                                    file: *const libc::c_char,
                                    flags: libc::c_uint)
                                    -> *const libc::c_char;
    fn virDomainSaveImageDefineXML(ptr: virConnectPtr,
                                   file: *const libc::c_char,
                                   dxml: *const libc::c_char,
                                   flags: libc::c_uint)
                                   -> libc::c_int;
    fn virDomainAttachDevice(ptr: sys::virDomainPtr, xml: *const libc::c_char) -> libc::c_int;
    fn virDomainAttachDeviceFlags(ptr: sys::virDomainPtr,
                                  xml: *const libc::c_char,
                                  flags: libc::c_uint)
                                  -> libc::c_int;
    fn virDomainDetachDevice(ptr: sys::virDomainPtr, xml: *const libc::c_char) -> libc::c_int;
    fn virDomainDetachDeviceFlags(ptr: sys::virDomainPtr,
                                  xml: *const libc::c_char,
                                  flags: libc::c_uint)
                                  -> libc::c_int;
    fn virDomainUpdateDeviceFlags(ptr: sys::virDomainPtr,
                                  xml: *const libc::c_char,
                                  flags: libc::c_uint)
                                  -> libc::c_int;
    fn virDomainManagedSave(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainHasManagedSaveImage(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainManagedSaveRemove(ptr: sys::virDomainPtr, flags: libc::c_uint) -> libc::c_int;
    fn virDomainCoreDump(ptr: sys::virDomainPtr,
                         to: *const libc::c_char,
                         flags: libc::c_uint)
                         -> libc::c_int;
    fn virDomainCoreDumpWithFormat(ptr: sys::virDomainPtr,
                                   to: *const libc::c_char,
                                   format: libc::c_uint,
                                   flags: libc::c_uint)
                                   -> libc::c_int;
    fn virDomainSetMetadata(ptr: sys::virDomainPtr,
                            kind: libc::c_int,
                            meta: *const libc::c_char,
                            key: *const libc::c_char,
                            uri: *const libc::c_char,
                            flags: libc::c_uint)
                            -> libc::c_int;
    fn virDomainGetMetadata(ptr: sys::virDomainPtr,
                            kind: libc::c_int,
                            uri: *const libc::c_char,
                            flags: libc::c_uint)
                            -> *mut libc::c_char;
    fn virDomainBlockResize(ptr: sys::virDomainPtr,
                            disk: *const libc::c_char,
                            size: libc::c_ulonglong,
                            flags: libc::c_uint)
                            -> libc::c_int;
    fn virDomainGetMemoryParameters(ptr: sys::virDomainPtr,
                                    params: virTypedParameterPtr,
                                    nparams: *mut libc::c_int,
                                    flags: libc::c_uint)
                                    -> libc::c_int;
    fn virDomainSetMemoryParameters(ptr: sys::virDomainPtr,
                                    params: virTypedParameterPtr,
                                    nparams: libc::c_int,
                                    flags: libc::c_uint)
                                    -> libc::c_int;
    fn virDomainMigrate(ptr: sys::virDomainPtr,
                        dconn: virConnectPtr,
                        flags: libc::c_uint,
                        dname: *const libc::c_char,
                        uri: *const libc::c_char,
                        bandwidth: libc::c_ulong)
                        -> sys::virDomainPtr;
    fn virDomainMigrate2(ptr: sys::virDomainPtr,
                         dconn: virConnectPtr,
                         dxml: *const libc::c_char,
                         flags: libc::c_uint,
                         dname: *const libc::c_char,
                         uri: *const libc::c_char,
                         bandwidth: libc::c_ulong)
                         -> sys::virDomainPtr;
    fn virDomainMigrateToURI(ptr: sys::virDomainPtr,
                             duri: *const libc::c_char,
                             flags: libc::c_uint,
                             dname: *const libc::c_char,
                             bandwidth: libc::c_ulong)
                             -> sys::virDomainPtr;
    fn virDomainMigrateToURI2(ptr: sys::virDomainPtr,
                              dconnuri: *const libc::c_char,
                              miguri: *const libc::c_char,
                              dxml: *const libc::c_char,
                              flags: libc::c_uint,
                              dname: *const libc::c_char,
                              bandwidth: libc::c_ulong)
                              -> sys::virDomainPtr;
}

pub type DomainXMLFlags = self::libc::c_uint;
pub const VIR_DOMAIN_XML_SECURE: DomainXMLFlags = 1 << 0;
pub const VIR_DOMAIN_XML_INACTIVE: DomainXMLFlags = 1 << 1;
pub const VIR_DOMAIN_XML_UPDATE_CPU: DomainXMLFlags = 1 << 2;
pub const VIR_DOMAIN_XML_MIGRATABLE: DomainXMLFlags = 1 << 3;

pub type DomainCreateFlags = self::libc::c_uint;
pub const VIR_DOMAIN_NONE: DomainCreateFlags = 0;
pub const VIR_DOMAIN_START_PAUSED: DomainCreateFlags = 1 << 0;
pub const VIR_DOMAIN_START_AUTODESTROY: DomainCreateFlags = 1 << 1;
pub const VIR_DOMAIN_START_BYPASS_CACHE: DomainCreateFlags = 1 << 2;
pub const VIR_DOMAIN_START_FORCE_BOOT: DomainCreateFlags = 1 << 3;
pub const VIR_DOMAIN_START_VALIDATE: DomainCreateFlags = 1 << 4;

pub type DomainDestroyFlags = self::libc::c_uint;
pub const VIR_DOMAIN_DESTROY_DEFAULT: DomainDestroyFlags = 0;
pub const VIR_DOMAIN_DESTROY_GRACEFUL: DomainDestroyFlags = 1 << 0;

pub type DomainModImpactFlags = self::libc::c_uint;
pub const VIR_DOMAIN_AFFECT_CURRENT: DomainModImpactFlags = 0;
pub const VIR_DOMAIN_AFFECT_LIVE: DomainModImpactFlags = 1 << 0;
pub const VIR_DOMAIN_AFFECT_CONFIG: DomainModImpactFlags = 1 << 1;

pub type DomainMemoryModFlags = self::libc::c_uint;
pub const VIR_DOMAIN_MEM_CURRENT: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_CURRENT;
pub const VIR_DOMAIN_MEM_LIVE: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_LIVE;
pub const VIR_DOMAIN_MEM_CONFIG: DomainMemoryModFlags = VIR_DOMAIN_AFFECT_CONFIG;
pub const VIR_DOMAIN_MEM_MAXIMUM: DomainMemoryModFlags = 1 << 2;

pub type DomainVcpuFlags = self::libc::c_uint;
pub const VIR_DOMAIN_VCPU_CURRENT: DomainVcpuFlags = VIR_DOMAIN_AFFECT_CURRENT;
pub const VIR_DOMAIN_VCPU_LIVE: DomainVcpuFlags = VIR_DOMAIN_AFFECT_LIVE;
pub const VIR_DOMAIN_VCPU_CONFIG: DomainVcpuFlags = VIR_DOMAIN_AFFECT_CONFIG;
pub const VIR_DOMAIN_VCPU_MAXIMUM: DomainVcpuFlags = 1 << 2;
pub const VIR_DOMAIN_VCPU_GUEST: DomainVcpuFlags = 1 << 3;
pub const VIR_DOMAIN_VCPU_HOTPLUGGABLE: DomainVcpuFlags = 1 << 4;

pub type DomainDefineFlags = self::libc::c_uint;
pub const VIR_DOMAIN_DEFINE_VALIDATE: DomainDefineFlags = 1 << 0;

pub type DomainSaveRestoreFlags = self::libc::c_uint;
pub const VIR_DOMAIN_SAVE_BYPASS_CACHE: DomainSaveRestoreFlags = 1 << 0;
pub const VIR_DOMAIN_SAVE_RUNNING: DomainSaveRestoreFlags = 1 << 1;
pub const VIR_DOMAIN_SAVE_PAUSED: DomainSaveRestoreFlags = 1 << 2;

pub type DomainNumatuneMemMode = self::libc::c_uint;
pub const VIR_DOMAIN_NUMATUNE_MEM_STRICT: DomainNumatuneMemMode = 0;
pub const VIR_DOMAIN_NUMATUNE_MEM_PREFERRED: DomainNumatuneMemMode = 1;
pub const VIR_DOMAIN_NUMATUNE_MEM_INTERLEAVE: DomainNumatuneMemMode = 2;

pub type DomainState = self::libc::c_uint;
pub const VIR_DOMAIN_NOSTATE: DomainState = 0;
pub const VIR_DOMAIN_RUNNING: DomainState = 1;
pub const VIR_DOMAIN_BLOCKED: DomainState = 2;
pub const VIR_DOMAIN_PAUSED: DomainState = 3;
pub const VIR_DOMAIN_SHUTDOWN: DomainState = 4;
pub const VIR_DOMAIN_SHUTOFF: DomainState = 5;
pub const VIR_DOMAIN_CRASHED: DomainState = 6;
pub const VIR_DOMAIN_PMSUSPENDED: DomainState = 7;

pub struct DomainInfo {
    pub state: DomainState,
    pub max_mem: u64,
    pub memory: u64,
    pub nr_virt_cpu: u32,
    pub cpu_time: u64,
}

impl DomainInfo {
    pub fn from_ptr(ptr: sys::virDomainInfoPtr) -> DomainInfo {
        unsafe {
            DomainInfo {
                state: (*ptr).state as DomainState,
                max_mem: (*ptr).maxMem as u64,
                memory: (*ptr).memory as u64,
                nr_virt_cpu: (*ptr).nrVirtCpu as u32,
                cpu_time: (*ptr).cpuTime as u64,
            }
        }
    }
}

pub struct DomainStatsRecord {
    // TODO(sahid): needs to be implemented
    pub ptr: sys::virDomainStatsRecordPtr,
}

pub struct BlockInfo {
    pub capacity: u64,
    pub allocation: u64,
    pub physical: u64,
}

impl BlockInfo {
    pub fn from_ptr(ptr: sys::virDomainBlockInfoPtr) -> BlockInfo {
        unsafe {
            BlockInfo {
                capacity: (*ptr).capacity as u64,
                allocation: (*ptr).capacity as u64,
                physical: (*ptr).capacity as u64,
            }
        }
    }
}

pub struct MemoryParameters {
    pub hard_limit: Option<u64>,
    pub soft_limit: Option<u64>,
    pub min_guarantee: Option<u64>,
    pub swap_hard_limit: Option<u64>,
}

impl MemoryParameters {
    pub fn new() -> MemoryParameters {
        MemoryParameters {
            hard_limit: None,
            soft_limit: None,
            min_guarantee: None,
            swap_hard_limit: None,
        }
    }
    pub fn from_vec(vec: Vec<virTypedParameter>) -> MemoryParameters {
        unsafe {
            let mut ret = MemoryParameters::new();
            for param in vec {
                match str::from_utf8(CStr::from_ptr(param.field.as_ptr()).to_bytes()).unwrap() {
                    "hard_limit" => ret.hard_limit = Some(param.value as u64),
                    "soft_limit" => ret.soft_limit = Some(param.value as u64),
                    "min_guarantee" => ret.min_guarantee = Some(param.value as u64),
                    "swap_hard_limit" => ret.swap_hard_limit = Some(param.value as u64),
                    _ => panic!("Field not implemented"),
                }
            }
            ret
        }
    }
}

pub struct InterfaceStats {
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
}

impl InterfaceStats {
    pub fn from_ptr(ptr: sys::virDomainInterfaceStatsPtr) -> InterfaceStats {
        unsafe {
            InterfaceStats {
                rx_bytes: (*ptr).rx_bytes as i64,
                rx_packets: (*ptr).rx_packets as i64,
                rx_errs: (*ptr).rx_errs as i64,
                rx_drop: (*ptr).rx_drop as i64,
                tx_bytes: (*ptr).tx_bytes as i64,
                tx_packets: (*ptr).tx_packets as i64,
                tx_errs: (*ptr).tx_errs as i64,
                tx_drop: (*ptr).tx_drop as i64,
            }
        }
    }
}

pub struct MemoryStats {
    pub tag: i32,
    pub val: u64,
}

impl MemoryStats {
    pub fn from_ptr(ptr: sys::virDomainMemoryStatsPtr) -> MemoryStats {
        unsafe {
            MemoryStats {
                tag: (*ptr).tag as i32,
                val: (*ptr).val as u64,
            }
        }
    }
}

pub struct Domain {
    ptr: sys::virDomainPtr,
}

impl Drop for Domain {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            if self.free().is_err() {
                panic!("Unable to drop memory for Domain")
            }
            return;
        }
    }
}

impl Domain {
    pub fn new(ptr: sys::virDomainPtr) -> Domain {
        return Domain { ptr: ptr };
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = virDomainGetConnect(self.ptr);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetName(self.ptr);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_os_type(&self) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetOSType(self.ptr);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_hostname(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetHostname(self.ptr, flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if virDomainGetUUIDString(self.ptr, uuid.as_mut_ptr()) == -1 {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr()));
        }
    }

    pub fn get_id(&self) -> Option<u32> {
        unsafe {
            let ret = virDomainGetID(self.ptr);
            if ret as i32 == -1 {
                return None;
            }
            Some(ret)
        }
    }

    pub fn get_xml_desc(&self, flags: DomainCreateFlags) -> Result<String, Error> {
        unsafe {
            let xml = virDomainGetXMLDesc(self.ptr, flags);
            if xml.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create(conn: &Connect) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainCreate(conn.as_ptr());
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn get_info(&self) -> Result<DomainInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainInfo::new();
            let res = virDomainGetInfo(self.ptr, pinfo);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(DomainInfo::from_ptr(pinfo));
        }
    }

    pub fn create_xml(conn: &Connect,
                      xml: &str,
                      flags: DomainCreateFlags)
                      -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainCreateXML(conn.as_ptr(),
                                         string_to_c_chars!(xml),
                                         flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }


    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainDefineXML(conn.as_ptr(), string_to_c_chars!(xml));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn define_xml_flags(conn: &Connect,
                            xml: &str,
                            flags: DomainDefineFlags)
                            -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainDefineXMLFlags(conn.as_ptr(),
                                              string_to_c_chars!(xml),
                                              flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if virDomainDestroy(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn destroy_flags(&self, flags: DomainDestroyFlags) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainDestroyFlags(self.ptr, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn shutdown(&self) -> Result<(), Error> {
        unsafe {
            if virDomainShutdown(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn reboot(&self) -> Result<(), Error> {
        unsafe {
            if virDomainReboot(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn suspend(&self) -> Result<(), Error> {
        unsafe {
            if virDomainSuspend(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn resume(&self) -> Result<(), Error> {
        unsafe {
            if virDomainResume(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainIsActive(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if virDomainUndefine(self.ptr) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if virDomainFree(self.ptr) == -1 {
                return Err(Error::new());
            }
            self.ptr = ptr::null_mut();
            return Ok(());
        }
    }

    pub fn is_updated(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainIsUpdated(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainGetAutostart(self.ptr);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetAutostart(self.ptr, autostart as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_max_memory(&self, memory: u64) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMaxMemory(self.ptr, memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_max_memory(&self) -> Result<u64, Error> {
        unsafe {
            let ret = virDomainGetMaxMemory(self.ptr);
            if ret == 0 {
                return Err(Error::new());
            }
            return Ok(ret as u64);
        }
    }

    pub fn get_max_vcpus(&self) -> Result<u64, Error> {
        unsafe {
            let ret = virDomainGetMaxVcpus(self.ptr);
            if ret == 0 {
                return Err(Error::new());
            }
            return Ok(ret as u64);
        }
    }

    pub fn set_memory(&self, memory: u64) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMemory(self.ptr, memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_memory_flags(&self,
                            memory: u64,
                            flags: DomainMemoryModFlags)
                            -> Result<bool, Error> {
        unsafe {
            let ret =
                virDomainSetMemoryFlags(self.ptr, memory as libc::c_ulong, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_memory_stats_period(&self,
                                   period: i32,
                                   flags: DomainMemoryModFlags)
                                   -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetMemoryStatsPeriod(self.ptr,
                                                    period as libc::c_int,
                                                    flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_vcpus(&self, vcpus: u32) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainSetVcpus(self.ptr, vcpus as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_vcpus_flags(&self, vcpus: u32, flags: DomainVcpuFlags) -> Result<bool, Error> {
        unsafe {
            let ret =
                virDomainSetVcpusFlags(self.ptr, vcpus as libc::c_uint, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn domain_restore(conn: &Connect, path: &str) -> Result<(), Error> {
        unsafe {
            if virDomainRestore(conn.as_ptr(), string_to_c_chars!(path)) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn domain_restore_flags(conn: &Connect,
                                path: &str,
                                flags: DomainSaveRestoreFlags)
                                -> Result<(), Error> {
        unsafe {
            if virDomainRestoreFlags(conn.as_ptr(),
                                     string_to_c_chars!(path),
                                     flags as libc::c_uint) == -1 {
                return Err(Error::new());
            }
            return Ok(());
        }
    }

    pub fn get_vcpus_flags(&self, flags: DomainVcpuFlags) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainGetVcpusFlags(self.ptr, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn migrate_set_max_speed(&self, bandwidth: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainMigrateSetMaxSpeed(self.ptr,
                                                  bandwidth as libc::c_ulong,
                                                  flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn migrate_get_max_speed(&self, flags: u32) -> Result<u64, Error> {
        unsafe {
            let mut bandwidth: libc::c_ulong = 0;
            let ret = virDomainMigrateGetMaxSpeed(self.ptr, &mut bandwidth, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(bandwidth as u64);
        }
    }

    pub fn migrate_set_compression_cache(&self, size: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainMigrateSetCompressionCache(self.ptr,
                                                          size as libc::c_ulong,
                                                          flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn migrate_get_compression_cache(&self, flags: u32) -> Result<u64, Error> {
        unsafe {
            let mut size: libc::c_ulong = 0;
            let ret =
                virDomainMigrateGetCompressionCache(self.ptr, &mut size, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(size as u64);
        }
    }

    pub fn migrate_set_max_downtime(&self, downtime: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainMigrateSetMaxDowntime(self.ptr,
                                                     downtime as libc::c_ulong,
                                                     flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn set_time(&self, seconds: i64, nseconds: i32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSetTime(self.ptr,
                                       seconds as libc::c_long,
                                       nseconds as libc::c_int,
                                       flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_time(&self, flags: u32) -> Result<(i64, i32), Error> {
        unsafe {
            let mut seconds: libc::c_long = 0;
            let mut nseconds: libc::c_int = 0;
            let ret =
                virDomainGetTime(self.ptr, &mut seconds, &mut nseconds, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok((seconds as i64, nseconds as i32));
        }
    }

    pub fn get_block_info(&self, disk: &str, flags: u32) -> Result<BlockInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainBlockInfo::new();
            let ret = virDomainGetBlockInfo(self.ptr,
                                            string_to_c_chars!(disk),
                                            pinfo,
                                            flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(BlockInfo::from_ptr(pinfo));
        }
    }

    pub fn pin_vcpu(&self, vcpu: u32, cpumap: &[u8]) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainPinVcpu(self.ptr,
                                       vcpu as libc::c_uint,
                                       cpumap.as_ptr(),
                                       cpumap.len() as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn pin_vcpu_flags(&self, vcpu: u32, cpumap: &[u8], flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainPinVcpuFlags(self.ptr,
                                            vcpu as libc::c_uint,
                                            cpumap.as_ptr(),
                                            cpumap.len() as libc::c_uint,
                                            flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn pin_emulator(&self, cpumap: &[u8], flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainPinEmulator(self.ptr,
                                           cpumap.as_ptr(),
                                           cpumap.len() as libc::c_uint,
                                           flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn rename(&self, new_name: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainRename(self.ptr,
                                      string_to_c_chars!(new_name),
                                      flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn set_user_password(&self, user: &str, password: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSetUserPassword(self.ptr,
                                               string_to_c_chars!(user),
                                               string_to_c_chars!(password),
                                               flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn set_block_threshold(&self, dev: &str, threshold: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSetBlockThreshold(self.ptr,
                                                 string_to_c_chars!(dev),
                                                 threshold as libc::c_ulonglong,
                                                 flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn open_graphics(&self, idx: u32, fd: i32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainOpenGraphics(self.ptr,
                                            idx as libc::c_uint,
                                            fd as libc::c_int,
                                            flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn open_graphics_fd(&self, idx: u32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainOpenGraphicsFD(self.ptr, idx as libc::c_uint, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn open_channel(&self, name: &str, stream: Stream, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainOpenChannel(self.ptr,
                                           string_to_c_chars!(name),
                                           stream.as_ptr(),
                                           flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn open_console(&self, name: &str, stream: Stream, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainOpenConsole(self.ptr,
                                           string_to_c_chars!(name),
                                           stream.as_ptr(),
                                           flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn interface_stats(&self, path: &str) -> Result<InterfaceStats, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainInterfaceStats::new();
            let ret = virDomainInterfaceStats(self.ptr,
                                              string_to_c_chars!(path),
                                              pinfo,
                                              mem::size_of::<sys::virDomainInterfaceStats>() as
                                              libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(InterfaceStats::from_ptr(pinfo));
        }
    }

    pub fn memory_stats(&self, nr_stats: u32, flags: u32) -> Result<MemoryStats, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainMemoryStats::new();
            let ret = virDomainMemoryStats(self.ptr,
                                           pinfo,
                                           nr_stats as libc::c_uint,
                                           flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(MemoryStats::from_ptr(pinfo));
        }
    }

    pub fn save_image_get_xml_desc(conn: &Connect,
                                   file: &str,
                                   flags: u32)
                                   -> Result<String, Error> {
        unsafe {
            let ptr = virDomainSaveImageGetXMLDesc(conn.as_ptr(),
                                                   string_to_c_chars!(file),
                                                   flags as libc::c_uint);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(ptr));
        }
    }

    pub fn save_image_define_xml(conn: &Connect,
                                 file: &str,
                                 dxml: &str,
                                 flags: u32)
                                 -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSaveImageDefineXML(conn.as_ptr(),
                                                  string_to_c_chars!(file),
                                                  string_to_c_chars!(dxml),
                                                  flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn attach_device(&self, xml: &str) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainAttachDevice(self.ptr, string_to_c_chars!(xml));
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn attach_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainAttachDeviceFlags(self.ptr,
                                                 string_to_c_chars!(xml),
                                                 flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn detach_device(&self, xml: &str) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainDetachDevice(self.ptr, string_to_c_chars!(xml));
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn detach_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainDetachDeviceFlags(self.ptr,
                                                 string_to_c_chars!(xml),
                                                 flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn update_device_flags(&self, xml: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainUpdateDeviceFlags(self.ptr,
                                                 string_to_c_chars!(xml),
                                                 flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn managed_save(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainManagedSave(self.ptr, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn has_managed_save(&self, flags: u32) -> Result<bool, Error> {
        unsafe {
            let ret = virDomainHasManagedSaveImage(self.ptr, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret == 1);
        }
    }

    pub fn managed_save_remove(&self, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainManagedSaveRemove(self.ptr, flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn core_dump(&self, to: &str, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainCoreDump(self.ptr, string_to_c_chars!(to), flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn core_dump_with_format(&self, to: &str, format: u32, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainCoreDumpWithFormat(self.ptr,
                                                  string_to_c_chars!(to),
                                                  format as libc::c_uint,
                                                  flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn set_metadata(&self,
                        kind: i32,
                        metadata: &str,
                        key: &str,
                        uri: &str,
                        flags: u32)
                        -> Result<u32, Error> {
        unsafe {
            let ret = virDomainSetMetadata(self.ptr,
                                           kind as libc::c_int,
                                           string_to_c_chars!(metadata),
                                           string_to_c_chars!(key),
                                           string_to_c_chars!(uri),
                                           flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_metadata(&self, kind: i32, uri: &str, flags: u32) -> Result<String, Error> {
        unsafe {
            let n = virDomainGetMetadata(self.ptr,
                                         kind as libc::c_int,
                                         string_to_c_chars!(uri),
                                         flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn block_resize(&self, disk: &str, size: u64, flags: u32) -> Result<u32, Error> {
        unsafe {
            let ret = virDomainBlockResize(self.ptr,
                                           string_to_c_chars!(disk),
                                           size as libc::c_ulonglong,
                                           flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_memory_parameters(&self, flags: u32) -> Result<MemoryParameters, Error> {
        unsafe {
            let mut nparams: libc::c_int = 0;
            let ret = virDomainGetMemoryParameters(self.ptr,
                                                   ptr::null_mut(),
                                                   &mut nparams,
                                                   flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            let mut params: Vec<virTypedParameter> = vec![
                virTypedParameter::default(); 3];
            let ret = virDomainGetMemoryParameters(self.ptr,
                                                   &mut params[0],
                                                   &mut nparams,
                                                   flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(MemoryParameters::from_vec(params))
        }
    }

    pub fn set_memory_parameters(&self,
                                 params: MemoryParameters,
                                 flags: u32)
                                 -> Result<u32, Error> {
        unsafe {
            fn to_arr(name: &str) -> [libc::c_char; 80] {
                let mut field: [libc::c_char; 80] = [0; 80];
                for (a, c) in field.iter_mut().zip(name.as_bytes()) {
                    *a = *c as i8
                }
                field
            }

            let mut cparams: Vec<virTypedParameter> = Vec::new();
            if params.hard_limit.is_some() {
                cparams.push(virTypedParameter {
                                 field: to_arr("hard_limit\0"),
                                 typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                                 value: params.hard_limit.unwrap(),
                             })
            }
            if params.soft_limit.is_some() {
                cparams.push(virTypedParameter {
                                 field: to_arr("soft_limit\0"),
                                 typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                                 value: params.soft_limit.unwrap(),
                             })
            }
            if params.min_guarantee.is_some() {
                cparams.push(virTypedParameter {
                                 field: to_arr("min_guarantee\0"),
                                 typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                                 value: params.min_guarantee.unwrap(),
                             })
            }
            if params.swap_hard_limit.is_some() {
                cparams.push(virTypedParameter {
                                 field: to_arr("swap_hard_limit\0"),
                                 typed: ::typedparam::VIR_TYPED_PARAM_ULLONG,
                                 value: params.swap_hard_limit.unwrap(),
                             })
            }

            let ret = virDomainSetMemoryParameters(self.ptr,
                                                   &mut cparams[0],
                                                   cparams.len() as libc::c_int,
                                                   flags as libc::c_uint);
            if ret == -1 {
                return Err(Error::new());
            }
            Ok(ret as u32)
        }
    }

    pub fn migrate(&self,
                   dconn: &Connect,
                   flags: u64,
                   dname: &str,
                   uri: &str,
                   bandwidth: u64)
                   -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainMigrate(self.ptr,
                                       dconn.as_ptr(),
                                       flags as libc::c_uint,
                                       string_to_c_chars!(dname),
                                       string_to_c_chars!(uri),
                                       bandwidth as libc::c_ulong);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn migrate2(&self,
                    dconn: &Connect,
                    dxml: &str,
                    flags: u64,
                    dname: &str,
                    uri: &str,
                    bandwidth: u64)
                    -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainMigrate2(self.ptr,
                                        dconn.as_ptr(),
                                        string_to_c_chars!(dxml),
                                        flags as libc::c_uint,
                                        string_to_c_chars!(dname),
                                        string_to_c_chars!(uri),
                                        bandwidth as libc::c_ulong);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn migrate_to_uri(&self,
                          duri: &str,
                          flags: u64,
                          dname: &str,
                          bandwidth: u64)
                          -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainMigrateToURI(self.ptr,
                                            string_to_c_chars!(duri),
                                            flags as libc::c_uint,
                                            string_to_c_chars!(dname),
                                            bandwidth as libc::c_ulong);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn migrate_to_uri2(&self,
                           dconn_uri: &str,
                           mig_uri: &str,
                           dxml: &str,
                           flags: u64,
                           dname: &str,
                           bandwidth: u64)
                           -> Result<Domain, Error> {
        unsafe {
            let ptr = virDomainMigrateToURI2(self.ptr,
                                             string_to_c_chars!(dconn_uri),
                                             string_to_c_chars!(mig_uri),
                                             string_to_c_chars!(dxml),
                                             flags as libc::c_uint,
                                             string_to_c_chars!(dname),
                                             bandwidth as libc::c_ulong);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }
}
