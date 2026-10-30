terraform {
  required_version = ">= 1.7.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.40.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.35.0"
    }
  }
}

variable "holding_org_id" {
  type        = string
  default     = "320717657398"
}

variable "production_folder_id" {
  type        = string
  default     = "978426653396"
}

variable "project_id" {
  type        = string
  default     = "auraintel-vpp-prod-188680247367"
}

variable "region" {
  type    = string
  default = "us-central1"
}

resource "google_compute_network" "nerc_cip_esp_vpc" {
  name                            = "auraintel-vpp-nerc-cip005-esp-vpc"
  project                         = var.project_id
  auto_create_subnetworks         = false
  routing_mode                    = "GLOBAL"
  delete_default_routes_on_create = true
}

resource "google_compute_firewall" "esp_strict_ingress_mtls_wireguard" {
  name      = "allow-only-pqc-wireguard-and-iec62351-mtls"
  project   = var.project_id
  network   = google_compute_network.nerc_cip_esp_vpc.name
  direction = "INGRESS"
  priority  = 100

  allow {
    protocol = "udp"
    ports    = ["51820"]
  }

  allow {
    protocol = "tcp"
    ports    = ["3782", "8443"]
  }

  source_ranges = ["10.240.0.0/24"]
  target_tags   = ["vpp-grid-governor"]
}

resource "google_kms_key_ring" "vpp_grid_hsm_keyring" {
  name     = "auraintel-vpp-fips140-3-keyring"
  project  = var.project_id
  location = var.region
}

resource "google_kms_crypto_key" "iec62351_dispatch_signing_key" {
  name     = "iec62351-dispatch-hsm-key"
  key_ring = google_kms_key_ring.vpp_grid_hsm_keyring.id
  purpose  = "ASYMMETRIC_SIGN"

  version_template {
    algorithm        = "EC_SIGN_P384_SHA384"
    protection_level = "HSM"
  }

  lifecycle {
    prevent_destroy = true
  }
}

resource "google_storage_bucket" "worm_pmu_and_aaufsa_audit_vault" {
  name                        = "auraintel-vpp-worm-vault-188680247367"
  project                     = var.project_id
  location                    = "US"
  storage_class               = "STANDARD"
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"

  retention_policy {
    is_locked        = true
    retention_period = 315576000
  }

  versioning {
    enabled = true
  }
}
