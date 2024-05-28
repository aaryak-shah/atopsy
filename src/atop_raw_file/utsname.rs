use crate::{constants::UTSNAME_FIELD_SIZE, utils};

#[derive(Debug)]
#[repr(C)]
pub struct UTSName {
    sys_name: [u8; UTSNAME_FIELD_SIZE],
    node_name: [u8; UTSNAME_FIELD_SIZE],
    release: [u8; UTSNAME_FIELD_SIZE],
    version: [u8; UTSNAME_FIELD_SIZE],
    machine: [u8; UTSNAME_FIELD_SIZE],
    domain_name: [u8; UTSNAME_FIELD_SIZE],
}

impl UTSName {
    pub fn get_sys_name_str(&mut self) -> String {
        utils::string_from_bytes(self.sys_name.to_vec())
    }
    pub fn get_node_name_str(&mut self) -> String {
        utils::string_from_bytes(self.node_name.to_vec())
    }
    pub fn get_release_str(&mut self) -> String {
        utils::string_from_bytes(self.release.to_vec())
    }
    pub fn get_version_str(&mut self) -> String {
        utils::string_from_bytes(self.version.to_vec())
    }
    pub fn get_machine_str(&mut self) -> String {
        utils::string_from_bytes(self.machine.to_vec())
    }
    pub fn get_domain_name_str(&mut self) -> String {
        utils::string_from_bytes(self.domain_name.to_vec())
    }
}
