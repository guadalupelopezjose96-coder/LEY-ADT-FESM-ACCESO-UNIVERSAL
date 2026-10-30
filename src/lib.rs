#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_arithmetic,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const MICRO_SCALE: i128 = 1_000_000;
pub const UWH_PER_MWH: i128 = 1_000_000_000_000;

pub const NOMINAL_FREQ_60HZ_UHZ: i128 = 60_000_000;
pub const MIN_SYNC_FREQ_UHZ: i128 = 59_950_000;
pub const MAX_SYNC_FREQ_UHZ: i128 = 60_050_000;
pub const MIN_VOLTAGE_MICRO_PU: i128 = 950_000;
pub const MAX_VOLTAGE_MICRO_PU: i128 = 1_050_000;
pub const MAX_SYNC_PHASE_DIFF_UDEG: i128 = 10_000_000;
pub const MAX_ROCOF_UHZ_PER_SEC: i128 = 500_000;
pub const MAX_PMU_TIME_SKEW_NS: u64 = 1_000;
pub const MAX_JUNCTION_TEMP_MC: i128 = 115_000;

pub const HOLDING_ORG_ID: &str = "320717657398";
pub const HOLDING_ORG_NAME: &str = "guadalupelopezjose96-org";
pub const AAUFSA_TAX_RATE_BPS: i128 = 1_530;
pub const BPS_DIVISOR: i128 = 10_000;
pub const AAUFSA_DISPLACEMENT_DAYS_THRESHOLD: u32 = 90;
pub const AAUFSA_REASSIGNMENT_SAFE_HARBOR_MONTHS: u32 = 36;
pub const AAUFSA_SMALL_BUSINESS_EXEMPTION_MICRO_USD: i128 = 5_000_000 * MICRO_SCALE;
pub const AAUFSA_TRUST_SPLIT_OASDI_PCT: i128 = 75;
pub const AAUFSA_TRUST_SPLIT_RESKILLING_PCT: i128 = 25;

pub const IRC_45Y_PTC_PER_MWH_MICRO_USD: i128 = 27_500_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorporateDomain {
    AuraTetrax,      // 877771073297
    Fourphonix,      // 964521086569
    AuraIntelCore,   // 188680247367
    AuraIntelGlobal, // 1087754464927
    Standard1979,    // 282230018778
    Prophonix,       // 358233233
}

impl CorporateDomain {
    #[must_use]
    pub const fn domain_id_str(&self) -> &'static str {
        match self {
            Self::AuraTetrax => "877771073297",
            Self::Fourphonix => "964521086569",
            Self::AuraIntelCore => "188680247367",
            Self::AuraIntelGlobal => "1087754464927",
            Self::Standard1979 => "282230018778",
            Self::Prophonix => "358233233",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleFolder {
    SharedServices, // 938478392755
    Production,     // 978426653396
    NonProduction,  // 467686749558
    Sandbox,        // 10470744625
}

impl LifecycleFolder {
    #[must_use]
    pub const fn folder_id_str(&self) -> &'static str {
        match self {
            Self::SharedServices => "938478392755",
            Self::Production => "978426653396",
            Self::NonProduction => "467686749558",
            Self::Sandbox => "10470744625",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergeticsError {
    ArithmeticOverflow,
    FrequencyOutOfEnvelope { measured_uhz: i128 },
    VoltageOutOfEnvelope { measured_micro_pu: i128 },
    OutOfPhaseReclosingBlocked { phase_diff_udeg: i128 },
    RocofLimitExceeded { rocof_uhz_per_s: i128 },
    PtpClockSkewViolation { skew_ns: u64 },
    ThermalOverload { temp_mc: i128 },
    BatterySocViolation { soc_micro_pct: i128 },
    TwoManRuleSignatureMissing,
    DuplicateSignaturesInTwoManRule,
    InvalidSettlementDirection,
    DoubleEntryImbalance { debit_micros: i128, credit_micros: i128 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynchrophasorMeasurement {
    pub grid_freq_uhz: i128,
    pub inverter_freq_uhz: i128,
    pub grid_voltage_micro_pu: i128,
    pub inverter_voltage_micro_pu: i128,
    pub phase_angle_diff_udeg: i128,
    pub rocof_uhz_per_sec: i128,
    pub ptp_time_skew_ns: u64,
    pub igbt_junction_temp_mc: i128,
    pub bess_soc_micro_pct: i128,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorAuthorization {
    pub primary_operator_id: String,
    pub primary_ml_dsa_sig_hash: String,
    pub secondary_engineer_id: String,
    pub secondary_ml_dsa_sig_hash: String,
}

impl OperatorAuthorization {
    pub fn validate(&self) -> Result<(), EnergeticsError> {
        if self.primary_operator_id.trim().is_empty()
            || self.secondary_engineer_id.trim().is_empty()
            || self.primary_ml_dsa_sig_hash.trim().is_empty()
            || self.secondary_ml_dsa_sig_hash.trim().is_empty()
        {
            return Err(EnergeticsError::TwoManRuleSignatureMissing);
        }
        if self.primary_operator_id == self.secondary_engineer_id
            || self.primary_ml_dsa_sig_hash == self.secondary_ml_dsa_sig_hash
        {
            return Err(EnergeticsError::DuplicateSignaturesInTwoManRule);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerDispatchCommand {
    pub active_power_uw: i128,
    pub reactive_power_uvar: i128,
    pub synthetic_inertia_constant_ms: u32,
    pub duration_hours_micro: i128,
    pub dispatched_energy_uwh: i128,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WholesaleMarketSettlement {
    pub idempotency_key: String,
    pub iso_rto_node_id: String,
    pub dispatched_energy_uwh: i128,
    pub lmp_price_per_mwh_micro_usd: i128,
    pub ancillary_ffr_revenue_micro_usd: i128,
    pub rec_clean_certificate_micro_usd: i128,
    pub gross_energy_settlement_micro_usd: i128,
    pub irc_45y_ptc_credit_micro_usd: i128,
    pub total_receivable_micro_usd: i128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesenergizadoAislado;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerificandoSincronismoFasorial;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SincronizadoEnRed;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DespachandoPotenciaActivaReactiva;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiquidandoMercadoLMP;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransicionModoIslaSegura;

#[derive(Debug, Clone)]
pub struct VppGridGovernor<State> {
    pub domain: CorporateDomain,
    pub folder: LifecycleFolder,
    pub vpp_resource_id: String,
    pub telemetry: Option<SynchrophasorMeasurement>,
    pub authorization: Option<OperatorAuthorization>,
    pub active_dispatch: Option<PowerDispatchCommand>,
    pub settlement: Option<WholesaleMarketSettlement>,
    _state: PhantomData<State>,
}

impl VppGridGovernor<DesenergizadoAislado> {
    #[must_use]
    pub const fn new(
        domain: CorporateDomain,
        folder: LifecycleFolder,
        vpp_resource_id: String,
    ) -> Self {
        Self {
            domain,
            folder,
            vpp_resource_id,
            telemetry: None,
            authorization: None,
            active_dispatch: None,
            settlement: None,
            _state: PhantomData,
        }
    }

    pub fn ingest_synchrophasor_telemetry(
        mut self,
        measurement: SynchrophasorMeasurement,
    ) -> Result<VppGridGovernor<VerificandoSincronismoFasorial>, EnergeticsError> {
        if measurement.ptp_time_skew_ns > MAX_PMU_TIME_SKEW_NS {
            return Err(EnergeticsError::PtpClockSkewViolation {
                skew_ns: measurement.ptp_time_skew_ns,
            });
        }
        if measurement.igbt_junction_temp_mc > MAX_JUNCTION_TEMP_MC {
            return Err(EnergeticsError::ThermalOverload {
                temp_mc: measurement.igbt_junction_temp_mc,
            });
        }
        if measurement.bess_soc_micro_pct < 5_000_000
            || measurement.bess_soc_micro_pct > 98_000_000
        {
            return Err(EnergeticsError::BatterySocViolation {
                soc_micro_pct: measurement.bess_soc_micro_pct,
            });
        }
        self.telemetry = Some(measurement);
        Ok(VppGridGovernor {
            domain: self.domain,
            folder: self.folder,
            vpp_resource_id: self.vpp_resource_id,
            telemetry: self.telemetry,
            authorization: None,
            active_dispatch: None,
            settlement: None,
            _state: PhantomData,
        })
    }
}

impl VppGridGovernor<VerificandoSincronismoFasorial> {
    pub fn close_breaker_and_synchronize(
        mut self,
        auth: OperatorAuthorization,
    ) -> Result<VppGridGovernor<SincronizadoEnRed>, EnergeticsError> {
        auth.validate()?;
        let pmu = self.telemetry.ok_or(EnergeticsError::ArithmeticOverflow)?;

        if pmu.grid_freq_uhz < MIN_SYNC_FREQ_UHZ || pmu.grid_freq_uhz > MAX_SYNC_FREQ_UHZ {
            return Err(EnergeticsError::FrequencyOutOfEnvelope {
                measured_uhz: pmu.grid_freq_uhz,
            });
        }
        if pmu.inverter_freq_uhz < MIN_SYNC_FREQ_UHZ || pmu.inverter_freq_uhz > MAX_SYNC_FREQ_UHZ {
            return Err(EnergeticsError::FrequencyOutOfEnvelope {
                measured_uhz: pmu.inverter_freq_uhz,
            });
        }
        if pmu.grid_voltage_micro_pu < MIN_VOLTAGE_MICRO_PU
            || pmu.grid_voltage_micro_pu > MAX_VOLTAGE_MICRO_PU
        {
            return Err(EnergeticsError::VoltageOutOfEnvelope {
                measured_micro_pu: pmu.grid_voltage_micro_pu,
            });
        }
        if pmu.phase_angle_diff_udeg.abs() > MAX_SYNC_PHASE_DIFF_UDEG {
            return Err(EnergeticsError::OutOfPhaseReclosingBlocked {
                phase_diff_udeg: pmu.phase_angle_diff_udeg,
            });
        }
        if pmu.rocof_uhz_per_sec.abs() > MAX_ROCOF_UHZ_PER_SEC {
            return Err(EnergeticsError::RocofLimitExceeded {
                rocof_uhz_per_s: pmu.rocof_uhz_per_sec,
            });
        }

        self.authorization = Some(auth);
        Ok(VppGridGovernor {
            domain: self.domain,
            folder: self.folder,
            vpp_resource_id: self.vpp_resource_id,
            telemetry: self.telemetry,
            authorization: self.authorization,
            active_dispatch: None,
            settlement: None,
            _state: PhantomData,
        })
    }
}

impl VppGridGovernor<SincronizadoEnRed> {
    pub fn execute_grid_forming_dispatch(
        mut self,
        base_active_power_uw: i128,
        base_reactive_power_uvar: i128,
        droop_gain_uw_per_uhz: i128,
        synthetic_inertia_constant_ms: u32,
        duration_hours_micro: i128,
    ) -> Result<VppGridGovernor<DespachandoPotenciaActivaReactiva>, EnergeticsError> {
        let pmu = self.telemetry.ok_or(EnergeticsError::ArithmeticOverflow)?;

        let freq_error_uhz = NOMINAL_FREQ_60HZ_UHZ
            .checked_sub(pmu.grid_freq_uhz)
            .ok_or(EnergeticsError::ArithmeticOverflow)?;

        let ffr_adjustment_uw = freq_error_uhz
            .checked_mul(droop_gain_uw_per_uhz)
            .ok_or(EnergeticsError::ArithmeticOverflow)?;

        let total_active_power_uw = base_active_power_uw
            .checked_add(ffr_adjustment_uw)
            .ok_or(EnergeticsError::ArithmeticOverflow)?;

        if total_active_power_uw <= 0 || duration_hours_micro <= 0 {
            return Err(EnergeticsError::InvalidSettlementDirection);
        }

        let dispatched_energy_uwh = total_active_power_uw
            .checked_mul(duration_hours_micro)
            .ok_or(EnergeticsError::ArithmeticOverflow)?
            .checked_div(MICRO_SCALE)
            .ok_or(EnergeticsError::ArithmeticOverflow)?;

        self.active_dispatch = Some(PowerDispatchCommand {
            active_power_uw: total_active_power_uw,
            reactive_power_uvar: base_reactive_power_uvar,
            synthetic_inertia_constant_ms,
            duration_hours_micro,
            dispatched_energy_uwh,
        });

        Ok(VppGridGovernor {
            domain: self.domain,
            folder: self.folder,
            vpp_resource_id: self.vpp_resource_id,
            telemetry: self.telemetry,
            authorization: self.authorization,
            active_dispatch: self.active_dispatch,
            settlement: None,
            _state: PhantomData,
        })
    }
}

impl VppGridGovernor<DespachandoPotenciaActivaReactiva> {
    pub fn settle_wholesale_market_lmp(
        mut self,
        idempotency_key: String,
        iso_rto_node_id: String,
        lmp_price_per_mwh_micro_usd: i128,
        ancillary_ffr_revenue_micro_usd: i128,
        rec_clean_certificate_micro_usd: i128,
        qualifies_irc_45y_prevailing_wage: bool,
    ) -> Result<VppGridGovernor<LiquidandoMercadoLMP>, EnergeticsError> {
        let dispatch = self
            .active_dispatch
            .ok_or(EnergeticsError::ArithmeticOverflow)?;

        if lmp_price_per_mwh_micro_usd < 0
            || ancillary_ffr_revenue_micro_usd < 0
            || rec_clean_certificate_micro_usd < 0
        {
            return Err(EnergeticsError::InvalidSettlementDirection);
        }

        let gross_energy_settlement_micro_usd = dispatch
            .dispatched_energy_uwh
            .checked_mul(lmp_price_per_mwh_micro_usd)
            .ok_or(EnergeticsError::ArithmeticOverflow)?
            .checked_div(UWH_PER_MWH)
            .ok_or(EnergeticsError::ArithmeticOverflow)?;

        let irc_45y_ptc_credit_micro_usd = if qualifies_irc_45y_prevailing_wage {
            dispatch
                .dispatched_energy_uwh
                .checked_mul(IRC_45Y_PTC_PER_MWH_MICRO_USD)
                .ok_or(EnergeticsError::ArithmeticOverflow)?
                .checked_div(UWH_PER_MWH)
                .ok_or(EnergeticsError::ArithmeticOverflow)?
        } else {
            0
        };

        let total_receivable_micro_usd = gross_energy_settlement_micro_usd
            .checked_add(ancillary_ffr_revenue_micro_usd)
            .ok_or(EnergeticsError::ArithmeticOverflow)?
            .checked_add(rec_clean_certificate_micro_usd)
            .ok_or(EnergeticsError::ArithmeticOverflow)?
            .checked_add(irc_45y_ptc_credit_micro_usd)
            .ok_or(EnergeticsError::ArithmeticOverflow)?;

        self.settlement = Some(WholesaleMarketSettlement {
            idempotency_key,
            iso_rto_node_id,
            dispatched_energy_uwh: dispatch.dispatched_energy_uwh,
            lmp_price_per_mwh_micro_usd,
            ancillary_ffr_revenue_micro_usd,
            rec_clean_certificate_micro_usd,
            gross_energy_settlement_micro_usd,
            irc_45y_ptc_credit_micro_usd,
            total_receivable_micro_usd,
        });

        Ok(VppGridGovernor {
            domain: self.domain,
            folder: self.folder,
            vpp_resource_id: self.vpp_resource_id,
            telemetry: self.telemetry,
            authorization: self.authorization,
            active_dispatch: self.active_dispatch,
            settlement: self.settlement,
            _state: PhantomData,
        })
    }

    #[must_use]
    pub fn trigger_protective_island_mode(self) -> VppGridGovernor<TransicionModoIslaSegura> {
        VppGridGovernor {
            domain: self.domain,
            folder: self.folder,
            vpp_resource_id: self.vpp_resource_id,
            telemetry: self.telemetry,
            authorization: self.authorization,
            active_dispatch: self.active_dispatch,
            settlement: None,
            _state: PhantomData,
        }
    }
}

impl VppGridGovernor<LiquidandoMercadoLMP> {
    #[must_use]
    pub fn transition_to_safe_island_mode(self) -> VppGridGovernor<TransicionModoIslaSegura> {
        VppGridGovernor {
            domain: self.domain,
            folder: self.folder,
            vpp_resource_id: self.vpp_resource_id,
            telemetry: self.telemetry,
            authorization: self.authorization,
            active_dispatch: self.active_dispatch,
            settlement: self.settlement,
            _state: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergeticsDeploymentModel {
    ThirdPartyUtilitySaasLicense,
    ProprietaryAutonomousGridOps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AaufsaStatutoryAuditInput {
    pub deployment_model: EnergeticsDeploymentModel,
    pub controlled_group_revenues_micro_usd: [i128; 6],
    pub displaced_grid_operators_count: u32,
    pub displacement_duration_days: u32,
    pub bls_median_annual_wage_micro_usd: i128,
    pub reassigned_to_grid_resilience_role: bool,
    pub continuous_reassignment_months: u32,
    pub wage_maintained_at_100_percent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AaufsaTaxDetermination {
    pub aggregated_group_revenue_micro_usd: i128,
    pub cloud_saas_provider_immunity_sec_5000h_d: bool,
    pub small_business_exemption_sec_5000g_b: bool,
    pub reassignment_safe_harbor_sec_5000g_a: bool,
    pub clawback_triggered_sec_5000g_c: bool,
    pub total_tax_liability_micro_usd: i128,
    pub oasdi_trust_fund_75_pct_micro_usd: i128,
    pub workforce_reskilling_25_pct_micro_usd: i128,
    pub statutory_citation: String,
    pub cryptographic_seal_sha256: String,
}

pub fn evaluate_aaufsa_chapter_56_compliance(
    input: &AaufsaStatutoryAuditInput,
) -> Result<AaufsaTaxDetermination, EnergeticsError> {
    let mut aggregated_group_revenue_micro_usd: i128 = 0;
    for rev in &input.controlled_group_revenues_micro_usd {
        if *rev < 0 {
            return Err(EnergeticsError::InvalidSettlementDirection);
        }
        aggregated_group_revenue_micro_usd = aggregated_group_revenue_micro_usd
            .checked_add(*rev)
            .ok_or(EnergeticsError::ArithmeticOverflow)?;
    }

    if input.deployment_model == EnergeticsDeploymentModel::ThirdPartyUtilitySaasLicense {
        let citation = String::from(
            "IRC Sec. 5000H(d) Cloud/SaaS Provider Immunity: 0% Tax Liability for Holding/Vendor",
        );
        let seal = compute_audit_seal(&citation, aggregated_group_revenue_micro_usd, 0);
        return Ok(AaufsaTaxDetermination {
            aggregated_group_revenue_micro_usd,
            cloud_saas_provider_immunity_sec_5000h_d: true,
            small_business_exemption_sec_5000g_b: false,
            reassignment_safe_harbor_sec_5000g_a: false,
            clawback_triggered_sec_5000g_c: false,
            total_tax_liability_micro_usd: 0,
            oasdi_trust_fund_75_pct_micro_usd: 0,
            workforce_reskilling_25_pct_micro_usd: 0,
            statutory_citation: citation,
            cryptographic_seal_sha256: seal,
        });
    }

    if aggregated_group_revenue_micro_usd < AAUFSA_SMALL_BUSINESS_EXEMPTION_MICRO_USD {
        let citation = String::from(
            "IRC Sec. 5000G(b) Controlled Group Small Business Exemption (< $5,000,000 USD)",
        );
        let seal = compute_audit_seal(&citation, aggregated_group_revenue_micro_usd, 0);
        return Ok(AaufsaTaxDetermination {
            aggregated_group_revenue_micro_usd,
            cloud_saas_provider_immunity_sec_5000h_d: false,
            small_business_exemption_sec_5000g_b: true,
            reassignment_safe_harbor_sec_5000g_a: false,
            clawback_triggered_sec_5000g_c: false,
            total_tax_liability_micro_usd: 0,
            oasdi_trust_fund_75_pct_micro_usd: 0,
            workforce_reskilling_25_pct_micro_usd: 0,
            statutory_citation: citation,
            cryptographic_seal_sha256: seal,
        });
    }

    if input.displaced_grid_operators_count == 0
        || input.displacement_duration_days <= AAUFSA_DISPLACEMENT_DAYS_THRESHOLD
    {
        let citation = String::from(
            "IRC Sec. 5000H(c) No Permanent Human Displacement (> 90 Days Threshold Not Met)",
        );
        let seal = compute_audit_seal(&citation, aggregated_group_revenue_micro_usd, 0);
        return Ok(AaufsaTaxDetermination {
            aggregated_group_revenue_micro_usd,
            cloud_saas_provider_immunity_sec_5000h_d: false,
            small_business_exemption_sec_5000g_b: false,
            reassignment_safe_harbor_sec_5000g_a: false,
            clawback_triggered_sec_5000g_c: false,
            total_tax_liability_micro_usd: 0,
            oasdi_trust_fund_75_pct_micro_usd: 0,
            workforce_reskilling_25_pct_micro_usd: 0,
            statutory_citation: citation,
            cryptographic_seal_sha256: seal,
        });
    }

    if input.reassigned_to_grid_resilience_role
        && input.continuous_reassignment_months >= AAUFSA_REASSIGNMENT_SAFE_HARBOR_MONTHS
        && input.wage_maintained_at_100_percent
    {
        let citation = String::from(
            "IRC Sec. 5000G(a) Employee Reassignment Safe Harbor Verified (>= 36 Months, 100% Wage)",
        );
        let seal = compute_audit_seal(&citation, aggregated_group_revenue_micro_usd, 0);
        return Ok(AaufsaTaxDetermination {
            aggregated_group_revenue_micro_usd,
            cloud_saas_provider_immunity_sec_5000h_d: false,
            small_business_exemption_sec_5000g_b: false,
            reassignment_safe_harbor_sec_5000g_a: true,
            clawback_triggered_sec_5000g_c: false,
            total_tax_liability_micro_usd: 0,
            oasdi_trust_fund_75_pct_micro_usd: 0,
            workforce_reskilling_25_pct_micro_usd: 0,
            statutory_citation: citation,
            cryptographic_seal_sha256: seal,
        });
    }

    let clawback_triggered = input.reassigned_to_grid_resilience_role
        && (input.continuous_reassignment_months < AAUFSA_REASSIGNMENT_SAFE_HARBOR_MONTHS
            || !input.wage_maintained_at_100_percent);

    let displaced_count_i128 = i128::from(input.displaced_grid_operators_count);
    let total_displaced_wage_base = input
        .bls_median_annual_wage_micro_usd
        .checked_mul(displaced_count_i128)
        .ok_or(EnergeticsError::ArithmeticOverflow)?;

    let total_tax_liability_micro_usd = total_displaced_wage_base
        .checked_mul(AAUFSA_TAX_RATE_BPS)
        .ok_or(EnergeticsError::ArithmeticOverflow)?
        .checked_div(BPS_DIVISOR)
        .ok_or(EnergeticsError::ArithmeticOverflow)?;

    let oasdi_trust_fund_75_pct_micro_usd = total_tax_liability_micro_usd
        .checked_mul(AAUFSA_TRUST_SPLIT_OASDI_PCT)
        .ok_or(EnergeticsError::ArithmeticOverflow)?
        .checked_div(100)
        .ok_or(EnergeticsError::ArithmeticOverflow)?;

    let workforce_reskilling_25_pct_micro_usd = total_tax_liability_micro_usd
        .checked_sub(oasdi_trust_fund_75_pct_micro_usd)
        .ok_or(EnergeticsError::ArithmeticOverflow)?;

    let citation = if clawback_triggered {
        String::from("IRC Sec. 5000F & Sec. 5000G(c) Clawback Triggered (< 36 Months Retention): 15.3% Excise Tax Assessed")
    } else {
        String::from("IRC Sec. 5000F Automated Asset Competitive Neutrality Tax: 15.3% Assessed (75% OASDI / 25% Reskilling)")
    };

    let seal = compute_audit_seal(
        &citation,
        aggregated_group_revenue_micro_usd,
        total_tax_liability_micro_usd,
    );

    Ok(AaufsaTaxDetermination {
        aggregated_group_revenue_micro_usd,
        cloud_saas_provider_immunity_sec_5000h_d: false,
        small_business_exemption_sec_5000g_b: false,
        reassignment_safe_harbor_sec_5000g_a: false,
        clawback_triggered_sec_5000g_c: clawback_triggered,
        total_tax_liability_micro_usd,
        oasdi_trust_fund_75_pct_micro_usd,
        workforce_reskilling_25_pct_micro_usd,
        statutory_citation: citation,
        cryptographic_seal_sha256: seal,
    })
}

fn compute_audit_seal(citation: &str, group_rev: i128, tax_liability: i128) -> String {
    let mut hasher = Sha256::new();
    hasher.update(HOLDING_ORG_ID.as_bytes());
    hasher.update(citation.as_bytes());
    hasher.update(group_rev.to_le_bytes());
    hasher.update(tax_liability.to_le_bytes());
    let digest = hasher.finalize();
    let mut hex_out = String::with_capacity(64);
    for byte in digest {
        use core::fmt::Write;
        let _ = write!(&mut hex_out, "{byte:02x}");
    }
    hex_out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_valid_pmu() -> SynchrophasorMeasurement {
        SynchrophasorMeasurement {
            grid_freq_uhz: 59_980_000,
            inverter_freq_uhz: 59_980_000,
            grid_voltage_micro_pu: 1_000_000,
            inverter_voltage_micro_pu: 1_000_000,
            phase_angle_diff_udeg: 1_500_000,
            rocof_uhz_per_sec: 120_000,
            ptp_time_skew_ns: 450,
            igbt_junction_temp_mc: 72_000,
            bess_soc_micro_pct: 80_000_000,
        }
    }

    fn sample_valid_auth() -> OperatorAuthorization {
        OperatorAuthorization {
            primary_operator_id: String::from("OP-GRID-001"),
            primary_ml_dsa_sig_hash: String::from("mldsa87_sig_alpha_9f8e7d6c"),
            secondary_engineer_id: String::from("ENG-PROT-002"),
            secondary_ml_dsa_sig_hash: String::from("mldsa87_sig_beta_1a2b3c4d"),
        }
    }

    #[test]
    fn test_typestate_full_grid_energization_and_lmp_settlement() {
        let gov = VppGridGovernor::<DesenergizadoAislado>::new(
            CorporateDomain::AuraIntelCore,
            LifecycleFolder::Production,
            String::from("VPP-BESS-NODE-01"),
        );

        let gov_sync_check = gov
            .ingest_synchrophasor_telemetry(sample_valid_pmu())
            .expect("Valid PMU telemetry must succeed");

        let gov_synced = gov_sync_check
            .close_breaker_and_synchronize(sample_valid_auth())
            .expect("Valid synchronization within IEEE 1547 envelope must succeed");

        let gov_dispatching = gov_synced
            .execute_grid_forming_dispatch(
                10_000_000_000_000,
                2_000_000_000_000,
                50_000,
                5_000,
                1_000_000,
            )
            .expect("Dispatch calculation must succeed");

        let gov_settled = gov_dispatching
            .settle_wholesale_market_lmp(
                String::from("IDEMP-ISO-20261030-001"),
                String::from("PJM-WEST-HUB-NODE-04"),
                45_000_000,
                15_000_000,
                10_000_000,
                true,
            )
            .expect("LMP market settlement must succeed");

        let settlement = gov_settled
            .settlement
            .as_ref()
            .expect("Settlement must be present");

        assert_eq!(settlement.gross_energy_settlement_micro_usd, 450_045_000);
        assert_eq!(settlement.irc_45y_ptc_credit_micro_usd, 275_027_500);
        assert_eq!(settlement.total_receivable_micro_usd, 750_072_500);

        let _gov_island = gov_settled.transition_to_safe_island_mode();
    }

    #[test]
    fn test_out_of_phase_reclosing_blocked() {
        let gov = VppGridGovernor::<DesenergizadoAislado>::new(
            CorporateDomain::Prophonix,
            LifecycleFolder::Production,
            String::from("SUB-BREAKER-52A"),
        );
        let mut bad_pmu = sample_valid_pmu();
        bad_pmu.phase_angle_diff_udeg = 25_000_000;

        let gov_check = gov
            .ingest_synchrophasor_telemetry(bad_pmu)
            .expect("Telemetry ingestion valid");
        let res = gov_check.close_breaker_and_synchronize(sample_valid_auth());
        assert_eq!(
            res.unwrap_err(),
            EnergeticsError::OutOfPhaseReclosingBlocked {
                phase_diff_udeg: 25_000_000
            }
        );
    }

    #[test]
    fn test_aaufsa_sec_5000h_d_saas_immunity_and_sec_5000f_tax_split() {
        let saas_input = AaufsaStatutoryAuditInput {
            deployment_model: EnergeticsDeploymentModel::ThirdPartyUtilitySaasLicense,
            controlled_group_revenues_micro_usd: [
                2_000_000 * MICRO_SCALE,
                2_000_000 * MICRO_SCALE,
                3_000_000 * MICRO_SCALE,
                1_500_000 * MICRO_SCALE,
                1_000_000 * MICRO_SCALE,
                500_000 * MICRO_SCALE,
            ],
            displaced_grid_operators_count: 10,
            displacement_duration_days: 180,
            bls_median_annual_wage_micro_usd: 100_000 * MICRO_SCALE,
            reassigned_to_grid_resilience_role: false,
            continuous_reassignment_months: 0,
            wage_maintained_at_100_percent: false,
        };

        let saas_res =
            evaluate_aaufsa_chapter_56_compliance(&saas_input).expect("Audit must succeed");
        assert!(saas_res.cloud_saas_provider_immunity_sec_5000h_d);
        assert_eq!(saas_res.total_tax_liability_micro_usd, 0);

        let internal_input = AaufsaStatutoryAuditInput {
            deployment_model: EnergeticsDeploymentModel::ProprietaryAutonomousGridOps,
            ..saas_input
        };
        let tax_res =
            evaluate_aaufsa_chapter_56_compliance(&internal_input).expect("Audit must succeed");
        assert_eq!(
            tax_res.total_tax_liability_micro_usd,
            153_000 * MICRO_SCALE
        );
        assert_eq!(
            tax_res.oasdi_trust_fund_75_pct_micro_usd,
            114_750 * MICRO_SCALE
        );
        assert_eq!(
            tax_res.workforce_reskilling_25_pct_micro_usd,
            38_250 * MICRO_SCALE
        );
    }
}
