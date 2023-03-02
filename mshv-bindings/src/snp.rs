// Copyright © 2022, Microsoft Corporation
//
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause
//

use crate::bindings::*;

pub fn get_default_snp_guest_policy() -> hv_snp_guest_policy {
    let mut snp_policy = hv_snp_guest_policy { as_uint64: 0_u64 };

    unsafe {
        snp_policy.__bindgen_anon_1.set_minor_version(0x1f);
        snp_policy.__bindgen_anon_1.set_major_version(0x00);
        snp_policy.__bindgen_anon_1.set_smt_allowed(1);
        snp_policy.__bindgen_anon_1.set_vmpls_required(1);
        snp_policy.__bindgen_anon_1.set_migration_agent_allowed(0);
        snp_policy.__bindgen_anon_1.set_debug_allowed(0);
    }

    snp_policy
}

pub fn get_default_isolation_state() -> u64 {
        hv_partition_isolation_state_HV_PARTITION_ISOLATION_SECURE as u64
}

pub fn get_sev_control_register(vmsa_pfn: u64) -> u64 {
    let mut sev_control = hv_x64_register_sev_control { as_uint64: 0_u64 };

    unsafe {
        sev_control.__bindgen_anon_1.set_enable_encrypted_state(1);
        sev_control.__bindgen_anon_1.set_vmsa_gpa_page_number(vmsa_pfn);
        sev_control.as_uint64
    }
}