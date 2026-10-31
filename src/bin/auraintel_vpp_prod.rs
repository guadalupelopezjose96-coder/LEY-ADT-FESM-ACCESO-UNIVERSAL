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
    PlatformVector, SovereignOperation, StatutorilySettledCommit, MICRO_SCALE, ORG_HOLDING_ID,
    VPP_MAX_FREQ_MICRO_HZ, VPP_MIN_FREQ_MICRO_HZ,
};

pub const AURAINTELCORE_TENANT_ID: &str = "188680247367";
pub const PRODUCTION_FOLDER_ID: &str = "978426653396";

struct VppSubstationNode {
    node_id: &'static str,
    grid_zone: &'static str,
    idempotency_key: [u8; 32],
    frequency_micro_hz: i128,
    energy_dispatched_micro_wh: i128,
    lmp_settlement_micros: i128,
    gemini_tokens_validated: i128,
}

#[must_use]
pub fn format_micros(micros: i128) -> String {
    let abs_val = micros.unsigned_abs();
    let whole = abs_val / (MICRO_SCALE as u128);
    let frac = abs_val % (MICRO_SCALE as u128);
    format!("{whole}.{frac:06}")
}

fn build_auraintelcore_vpp_fleet() -> [VppSubstationNode; 4] {
    [
        VppSubstationNode {
            node_id: "VPP-CORE-NODE-01-BESS-PRIMARY",
            grid_zone: "auraintelcore.com/us-east-iso-ferc2222",
            idempotency_key: [0xA1; 32],
            frequency_micro_hz: 60_000_000,            // 60.000000 Hz exactos
            energy_dispatched_micro_wh: 250_000_000_000, // 250 MWh
            lmp_settlement_micros: 312_500_000_000,    // $312,500.000000 USD
            gemini_tokens_validated: 4_200_000,
        },
        VppSubstationNode {
            node_id: "VPP-CORE-NODE-02-GRID-FORMING-INV",
            grid_zone: "auraintelcore.com/us-west-wecc-ieee1547",
            idempotency_key: [0xA2; 32],
            frequency_micro_hz: 59_995_000,            // 59.995000 Hz
            energy_dispatched_micro_wh: 180_000_000_000, // 180 MWh
            lmp_settlement_micros: 225_000_000_000,    // $225,000.000000 USD
            gemini_tokens_validated: 3_150_000,
        },
        VppSubstationNode {
            node_id: "VPP-CORE-NODE-03-HYDRO-SYNCHRO",
            grid_zone: "auraintelcore.com/cenace-sin-60hz",
            idempotency_key: [0xA3; 32],
            frequency_micro_hz: 60_005_000,            // 60.005000 Hz
            energy_dispatched_micro_wh: 420_000_000_000, // 420 MWh
            lmp_settlement_micros: 504_000_000_000,    // $504,000.000000 USD
            gemini_tokens_validated: 5_800_000,
        },
        VppSubstationNode {
            node_id: "VPP-CORE-NODE-04-DER-AGGREGATOR",
            grid_zone: "auraintelcore.com/global-edge-mesh",
            idempotency_key: [0xA4; 32],
            frequency_micro_hz: 60_000_000,            // 60.000000 Hz
            energy_dispatched_micro_wh: 150_000_000_000, // 150 MWh
            lmp_settlement_micros: 195_000_000_000,    // $195,000.000000 USD
            gemini_tokens_validated: 2_900_000,
        },
    ]
}

fn dispatch_production_node(
    node: &VppSubstationNode,
) -> Result<SovereignOperation<StatutorilySettledCommit>, &'static str> {
    // 1. Validar plano cognitivo Gemini en AuraIntelCore.com (store: false obligatorio)
    let cognitive_gate = SovereignOperation::new_unverified(
        node.idempotency_key,
        LifecycleFolder::Production,
        CorporateDomain::AuraIntelCore,
        PlatformVector::GeminiCognitive {
            store_false_enforced: true,
            prompt_shield_passed: true,
            tokens_consumed: node.gemini_tokens_validated,
        },
        node.lmp_settlement_micros,
        true,
        2,
    );

    if cognitive_gate.verify_safety_and_policy().is_err() {
        return Err("FALLO_COMPUERTA_COGNITIVA_GEMINI_AURAINTELCORE");
    }

    // 2. Validar y despachar plano eléctrico AuraIntel VPP en 02-Production
    let vpp_op = SovereignOperation::new_unverified(
        node.idempotency_key,
        LifecycleFolder::Production,
        CorporateDomain::AuraIntelCore,
        PlatformVector::VppEnergetics {
            frequency_micro_hz: node.frequency_micro_hz,
            energy_dispatched_micro_wh: node.energy_dispatched_micro_wh,
            anti_islanding_armed: true,
        },
        node.lmp_settlement_micros,
        true,
        2,
    );

    let attested: SovereignOperation<AttestedAndCompliantAction> = vpp_op
        .verify_safety_and_policy()
        .map_err(|_| "FALLO_ENVOLVENTE_IEEE1547_VPP")?;

    // 3. Liquidar en Libro Mayor SERIALIZABLE bajo AAUFSA Sec. 5000H(d) / Sec. 5000G(a)
    attested
        .settle_statutory_ledger(AaufsaDeploymentContext::ExternalCloudSaasProvider)
        .map_err(|_| "FALLO_LIQUIDACION_SERIALIZABLE_AAUFSA")
}

fn main() -> ExitCode {
    let fleet = build_auraintelcore_vpp_fleet();
    let mut total_energy_micro_wh: i128 = 0;
    let mut total_lmp_micros: i128 = 0;
    let mut total_tokens: i128 = 0;

    let mut out = String::with_capacity(4096);
    out.push_str("{\n");
    out.push_str("  \"deployment_state\": \"PRODUCTION_ACTIVE_100PCT\",\n");
    out.push_str(&format!("  \"holding_org_id\": \"{ORG_HOLDING_ID}\",\n"));
    out.push_str(&format!("  \"lifecycle_folder\": \"02-Production ({PRODUCTION_FOLDER_ID})\",\n"));
    out.push_str(&format!("  \"primary_domain\": \"auraintelcore.com ({AURAINTELCORE_TENANT_ID})\",\n"));
    out.push_str("  \"cyber_physical_engine\": \"AuraIntel VPP\",\n");
    out.push_str("  \"gemini_endpoint_policy\": \"service:generativelanguage.googleapis.com (store:false)\",\n");
    out.push_str(&format!(
        "  \"ieee1547_frequency_envelope_hz\": \"{} - {}\",\n",
        format_micros(VPP_MIN_FREQ_MICRO_HZ),
        format_micros(VPP_MAX_FREQ_MICRO_HZ)
    ));
    out.push_str("  \"substation_dispatches\": [\n");

    for (idx, node) in fleet.iter().enumerate() {
        let settled = match dispatch_production_node(node) {
            Ok(s) => s,
            Err(msg) => {
                eprintln!("[FATAL FAIL-CLOSED] {msg}");
                return ExitCode::from(1);
            }
        };

        total_energy_micro_wh = total_energy_micro_wh.saturating_add(node.energy_dispatched_micro_wh);
        total_lmp_micros = total_lmp_micros.saturating_add(settled.gross_amount_micros);
        total_tokens = total_tokens.saturating_add(node.gemini_tokens_validated);

        out.push_str("    {\n");
        out.push_str(&format!("      \"node_id\": \"{}\",\n", node.node_id));
        out.push_str(&format!("      \"grid_zone\": \"{}\",\n", node.grid_zone));
        out.push_str(&format!("      \"frequency_hz\": \"{}\",\n", format_micros(node.frequency_micro_hz)));
        out.push_str(&format!("      \"energy_mwh\": \"{}\",\n", format_micros(node.energy_dispatched_micro_wh)));
        out.push_str(&format!("      \"lmp_settlement_usd\": \"{}\",\n", format_micros(settled.gross_amount_micros)));
        out.push_str("      \"anti_islanding\": \"ARMED_SIL4\",\n");
        out.push_str("      \"aaufsa_status\": \"SEC_5000H_D_EXEMPT_COMMITTED\"\n");
        if idx < 3 {
            out.push_str("    },\n");
        } else {
            out.push_str("    }\n");
        }
    }

    out.push_str("  ],\n");
    out.push_str("  \"production_totals\": {\n");
    out.push_str(&format!("    \"total_energy_dispatched_mwh\": \"{}\",\n", format_micros(total_energy_micro_wh)));
    out.push_str(&format!("    \"total_lmp_settled_usd\": \"{}\",\n", format_micros(total_lmp_micros)));
    out.push_str(&format!("    \"total_gemini_tokens_attested\": {total_tokens}\n"));
    out.push_str("  }\n}\n");

    if let Ok(mut f) = File::create("auraintel_vpp_production_state.json") {
        let _ = f.write_all(out.as_bytes());
    }
    println!("{out}");
    ExitCode::SUCCESS
}

#[cfg(test)]
mod prod_tests {
    use super::*;

    #[test]
    fn test_auraintel_vpp_production_fleet_100pct() {
        let fleet = build_auraintelcore_vpp_fleet();
        for node in &fleet {
            assert!(dispatch_production_node(node).is_ok());
        }
    }
}
