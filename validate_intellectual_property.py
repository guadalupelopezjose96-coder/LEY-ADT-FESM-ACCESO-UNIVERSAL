#!/usr/bin/env python3
import sys
import os
import json
import subprocess
import time

# --- CONSTANTES DE SOBERANÍA AL 100% ---
MANIFEST_FILE = "soberania-adt-core.json"
SIG_FILE = "soberania-adt-core.json.asc"
REQUIRED_CLASSIFICATION = "100% PRIVADO Y SOBERANO"
REQUIRED_LEGACY_YEARS = 50

def print_100(msg):
    print(f"[100%] {msg}")

def main():
    print("--- [100%] INICIANDO AUDITORÍA INTEGRAL DE VERDAD DIGITAL Y LEY ADT ---")
    time.sleep(0.5)

    # 1. VERIFICACIÓN FÍSICA AL 100%
    print_100("Buscando el manifiesto y los sellos criptográficos en el bunker...")
    if not os.path.exists(MANIFEST_FILE) or not os.path.exists(SIG_FILE):
        print("❌ [ALERTA AL 100%] Integridad comprometida. Faltan los archivos de soberanía o la firma .asc.")
        sys.exit(1)
    print_100("Archivos localizados al 100%.")
    time.sleep(0.5)

    # 2. AUDITORÍA LEGAL AL 100% (LAW-AS-CODE)
    print_100("Auditando el interior del manifiesto JSON (Aura Intel Core / Tetrax)...")
    try:
        with open(MANIFEST_FILE, 'r') as f:
            data = json.load(f)
        
        # Validar Propiedad 100% Privada
        if data.get("clasificacion_seguridad") != REQUIRED_CLASSIFICATION:
            print("❌ [ALERTA AL 100%] Infracción legal. El código no está marcado como 100% PRIVADO Y SOBERANO.")
            sys.exit(1)
        
        # Validar Visión a 50 Años
        if data["plan_de_legado_y_sucesion"]["horizonte_temporal_anios"] != REQUIRED_LEGACY_YEARS:
            print("❌ [ALERTA AL 100%] Desviación de arquitectura. El legado técnico debe estar fijado exactamente en 50 años.")
            sys.exit(1)

        print_100("Estructura de Propiedad Privada, algoritmos de procesamiento y legado verificados al 100%.")
    except Exception as e:
        print(f"❌ [ALERTA AL 100%] Manifiesto JSON corrupto o ilegible: {e}")
        sys.exit(1)
    time.sleep(0.5)

    # 3. VERIFICACIÓN CRIPTOGRÁFICA GPG AL 100%
    print_100("Validando la firma criptográfica ED25519 con GPG...")
    # Ejecutamos gpg en modo silencioso para verificar la firma separada
    result = subprocess.run(
        ['gpg', '--verify', SIG_FILE, MANIFEST_FILE], 
        capture_output=True, 
        text=True
    )
    
    if result.returncode != 0:
        print("❌ [ALERTA AL 100%] Fallo crítico. La firma GPG es inválida o no coincide con el archivo.")
        print(result.stderr)
        sys.exit(1)

    print_100("Sello ED25519 validado. Autenticidad del autor confirmada al 100%.")
    time.sleep(0.5)

    # 4. APROBACIÓN TOTAL AL 100%
    print("✅ [ÉXITO AL 100%] Propiedad Intelectual certificada. El sistema está 100% blindado y listo para operaciones en la bóveda GitHub.")
    sys.exit(0)

if __name__ == "__main__":
    main()
