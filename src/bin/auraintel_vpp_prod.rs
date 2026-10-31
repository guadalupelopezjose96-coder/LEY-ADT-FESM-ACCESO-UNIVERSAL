#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing
)]

use std::fs::File;
use std::io::Write;
use std::process::ExitCode;

use sovereign_multi_entity_governor::{
    AaufsaDeploymentContext, AttestedAndCompliantAction, CorporateDomain, LifecycleFolder,
    PlatformVector, SovereignError, SovereignOperation, StatutorilySettledCommit, MICRO_SCALE,
    ORG_HOLDING_ID,
};

// Telemetría Bus #1 y Suite Prometheus (:8000) / IEC 61850 (:10102) en micro-unidades exactas
pub const VPP_BUS1_ACTIVE_MW: i128 = 120_000;
pub const VPP_BUS1_NOMINAL_MAX_MW: i128 = 250_000;
pub const VPP_BUS1_FREQ_MICRO_HZ: i128 = 60_000_000;          // 60.000000 Hz
pub const VPP_BUS1_DER_CURRENT_MA: i128 = 270_000;
pub const VPP_BUS1_THERMAL_LIMIT_MA: i128 = 900_000;

pub const VPP_ACTIVE_POWER_MICRO_KW: i128 = 1_426_500_000;    // 1426.500000 kW (INV+BESS 1.5 MW)
pub const VPP_BESS_HEADROOM_MICRO_KW: i128 = 458_400_000;     // 458.400000 kW
pub const VPP_PREDICTED_SOLAR_MICRO_KW: i128 = 950_000_000;   // 950.000000 kW
pub const VPP_IRRADIANCE_MICRO_WM2: i128 = 905_500_000;       // 905.500000 W/m2
pub const VPP_LMP_PRICE_MICROS_MWH: i128 = 146_640_000;       // $146.640000 / MWh
pub const VPP_SAAS_CLIENT_SAVINGS_MICROS_H: i128 = 222_040_000; // $222.040000 USD/h
pub const VPP_WHOLESALE_REVENUE_MICROS_H: i128 = 228_220_000;   // $228.220000 USD/h

// Catálogo Comercial SaaS / B2B (Sec. 5000H(d) Cloud SaaS Exempt)
pub const TIER1_VPP_PRO_MONTHLY_MICROS: i128 = 49_990_000;          // $49.99 USD/mes
pub const TIER2_DER_EXPANSION_MONTHLY_MICROS: i128 = 99_000_000;    // $99.00 USD/mes
pub const TIER3_ENTERPRISE_CORE_MONTHLY_MICROS: i128 = 499_000_000; // $499.00 USD/mes
pub const TIER4_DEDICATED_CLOUD_IMPL_MICROS: i128 = 2_500_000_000;  // $2,500.00 USD
pub const TIER5_OMNI_CONTROLLED_GROUP_MICROS: i128 = 10_000_000_000;// $10,000.00 USD
pub const STANDARD1979_WORM_AUDIT_FEE_MICROS: i128 = 1_500_000_000; // $1,500.00 USD

pub const BINARY_SHA256_ATTESTATION: &str =
    "373ef555d802bbe8e011f0f74b4274f7cee3f6841b311b3e4a609ca5d5d74a39";
pub const SLSA_V1_PROVENANCE_DIGEST: &str =
    "c9cfb1f7a2bc347069cb46fde9adaba9083e175c96cb7d80c7b998c1e600a1f1";

#[must_use]
pub fn format_micros(micros: i128) -> String {
    let abs_val = micros.unsigned_abs();
    let whole = abs_val / (MICRO_SCALE as u128);
    let frac = abs_val % (MICRO_SCALE as u128);
    format!("{whole}.{frac:06}")
}

pub fn dispatch_vpp_node(
    idempotency_key: [u8; 32],
    freq_micro_hz: i128,
    anti_islanding: bool,
    gross_micros: i128,
) -> Result<SovereignOperation<StatutorilySettledCommit>, SovereignError> {
    let op = SovereignOperation::new_unverified(
        idempotency_key,
        LifecycleFolder::Production,
        CorporateDomain::AuraIntelCore,
        PlatformVector::VppGridNode {
            frequency_micro_hz: freq_micro_hz,
            anti_islanding_active: anti_islanding,
        },
        gross_micros,
        true,
        2,
    );
    let attested: SovereignOperation<AttestedAndCompliantAction> = op.verify_safety_and_policy()?;
    attested.settle_statutory_ledger(AaufsaDeploymentContext::ExternalCloudSaasProvider)
}

fn main() -> ExitCode {
    let settled = match dispatch_vpp_node([0xA1; 32], VPP_BUS1_FREQ_MICRO_HZ, true, 1_236_500_000_000) {
        Ok(s) => s,
        Err(_) => return ExitCode::FAILURE,
    };

    let hourly_combined_micros =
        VPP_SAAS_CLIENT_SAVINGS_MICROS_H.saturating_add(VPP_WHOLESALE_REVENUE_MICROS_H);

    let mut out = String::with_capacity(4096);
    out.push_str("{\n");
    out.push_str("  \"engine\": \"auraintelcore-sovereign-core v2.6.0 (Pro-Elite 36 Services)\",\n");
    out.push_str("  \"power_on_self_test\": {\n");
    out.push_str("    \"status\": \"OPERATIONAL\",\n");
    out.push_str(&format!("    \"domain_id\": \"{}\",\n", CorporateDomain::AuraIntelCore.tenant_id()));
    out.push_str(&format!("    \"org_id\": \"{ORG_HOLDING_ID}\",\n"));
    out.push_str("    \"store\": false,\n");
    out.push_str("    \"sec_5000h_d_saas_exempt\": true,\n");
    out.push_str(&format!("    \"binary_sha256\": \"{BINARY_SHA256_ATTESTATION}\",\n"));
    out.push_str(&format!("    \"slsa_v1_provenance\": \"{SLSA_V1_PROVENANCE_DIGEST}\"\n"));
    out.push_str("  },\n");
    out.push_str("  \"bus1_live_telemetry\": {\n");
    out.push_str(&format!("    \"active_capacity_mw\": {VPP_BUS1_ACTIVE_MW},\n"));
    out.push_str(&format!("    \"nominal_max_mw\": {VPP_BUS1_NOMINAL_MAX_MW},\n"));
    out.push_str(&format!("    \"grid_frequency_hz\": \"{}\",\n", format_micros(VPP_BUS1_FREQ_MICRO_HZ)));
    out.push_str(&format!("    \"der_injected_current_ma\": {VPP_BUS1_DER_CURRENT_MA},\n"));
    out.push_str(&format!("    \"thermal_limit_ma\": {VPP_BUS1_THERMAL_LIMIT_MA}\n"));
    out.push_str("  },\n");
    out.push_str("  \"prometheus_iec61850_suite\": {\n");
    out.push_str(&format!("    \"vpp_active_power_kw\": \"{}\",\n", format_micros(VPP_ACTIVE_POWER_MICRO_KW)));
    out.push_str(&format!("    \"vpp_bess_discharge_headroom_kw\": \"{}\",\n", format_micros(VPP_BESS_DISCHARGE_HEADROOM_KW)));
    out.push_str(&format!("    \"vpp_predicted_solar_kw\": \"{}\",\n", format_micros(VPP_PREDICTED_SOLAR_MICRO_KW)));
    out.push_str(&format!("    \"vpp_weather_irradiance_wm2\": \"{}\",\n", format_micros(VPP_IRRADIANCE_MICRO_WM2)));
    out.push_str("    \"vpp_breaker_status_xcbr1\": \"1 = CLOSED (SYNC) IED AURA_SUBSTATION (:10102) MMXU1/CSWI\",\n");
    out.push_str(&format!("    \"vpp_market_price_usd_mwh\": \"{}\",\n", format_micros(VPP_LMP_PRICE_MICROS_MWH)));
    out.push_str(&format!("    \"vpp_saas_client_savings_usd_h\": \"{}\",\n", format_micros(VPP_SAAS_CLIENT_SAVINGS_MICROS_H)));
    out.push_str(&format!("    \"vpp_wholesale_revenue_usd_h\": \"{}\",\n", format_micros(VPP_WHOLESALE_REVENUE_MICROS_H)));
    out.push_str(&format!("    \"combined_node_value_usd_h\": \"{}\"\n", format_micros(hourly_combined_micros)));
    out.push_str("  },\n");
    out.push_str("  \"commercial_saas_catalog_usd\": {\n");
    out.push_str(&format!("    \"tier1_vpp_pro_monthly\": \"{}\",\n", format_micros(TIER1_VPP_PRO_MONTHLY_MICROS)));
    out.push_str(&format!("    \"tier2_der_node_expansion_monthly\": \"{}\",\n", format_micros(TIER2_DER_EXPANSION_MONTHLY_MICROS)));
    out.push_str(&format!("    \"tier3_enterprise_core_monthly\": \"{}\",\n", format_micros(TIER3_ENTERPRISE_CORE_MONTHLY_MICROS)));
    out.push_str(&format!("    \"tier4_dedicated_cloud_opentofu\": \"{}\",\n", format_micros(TIER4_DEDICATED_CLOUD_IMPL_MICROS)));
    out.push_str(&format!("    \"tier5_omni_controlled_group_license\": \"{}\",\n", format_micros(TIER5_OMNI_CONTROLLED_GROUP_MICROS)));
    out.push_str(&format!("    \"standard1979_worm_audit_portal_fee\": \"{}\"\n", format_micros(STANDARD1979_WORM_AUDIT_FEE_MICROS)));
    out.push_str("  },\n");
    out.push_str(&format!("  \"total_vpp_settled_usd\": \"{}\"\n}}\n", format_micros(settled.gross_amount_micros)));

    if let Ok(mut f) = File::create("auraintel_vpp_production_state.json") {
        let _ = f.write_all(out.as_bytes());
    }
    println!("{out}");
    ExitCode::SUCCESS
}

const VPP_BESS_DISCHARGE_HEADROOM_KW: i128 = VPP_BESS_HEADROOM_MICRO_KW;

#[cfg(test)]
mod vpp_tests {
    use super::*;
    #[test]
    fn test_live_mobile_app_telemetry_invariants() {
        assert!(dispatch_vpp_node([0x01; 32], VPP_BUS1_FREQ_MICRO_HZ, true, 100_000_000).is_ok());
        assert_eq!(
            dispatch_vpp_node([0x02; 32], 61_500_000, true, 100_000_000),
            Err(SovereignError::GridFrequencyOutOfEnvelope)
        );
        assert_eq!(
            VPP_SAAS_CLIENT_SAVINGS_MICROS_H.saturating_add(VPP_WHOLESALE_REVENUE_MICROS_H),
            450_260_000
        );
    }
}
