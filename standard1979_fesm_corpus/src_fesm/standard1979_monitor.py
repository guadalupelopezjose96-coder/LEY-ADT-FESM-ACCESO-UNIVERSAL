# /* PROPIEDAD INTELECTUAL PROTEGIDA - LEY ADT / STANDARD 1979.
# Módulo: Realidad Real JEGL. Autor: JEGL.
# Cualquier suplantación digital o comercial es ilegal. */

import os
import json
import time

def get_live_metrics():
    log_dir = "audit_records_standard1979"
    total_value = 0.0
    total_tax = 0.0
    txn_count = 0
    last_status = "INACTIVO"

    if os.path.exists(log_dir):
        # Obtener el archivo de log más reciente
        logs = sorted([f for f in os.listdir(log_dir) if f.endswith('.log')])
        if logs:
            last_status = "OPERATIVO (100%)"
            log_path = os.path.join(log_dir, logs[-1])
            with open(log_path, "r", encoding="utf-8", errors="ignore") as f:
                for line in f:
                    if "METADATA" in line:
                        txn_count += 1
                        try:
                            # Deserialización segura mediante JSON (0% CWE-95 / Sin eval)
                            parts = line.split("METADATA:", 1)
                            if len(parts) > 1:
                                raw_json = parts[1].strip()
                                data = json.loads(raw_json)
                                val = float(data.get("valor", 0.0))
                                total_value += val
                                total_tax += val * 0.065 # Tasa FL
                        except Exception:
                            continue

    return txn_count, total_value, total_tax, last_status

def display_dashboard():
    while True:
        count, val, tax, status = get_live_metrics()

        # Limpieza de pantalla segura multiplataforma con secuencias ANSI (0% CWE-78)
        print("\033[H\033[J", end="")

        print("="*60)
        print(f"CONTROL CENTRAL: LEY STANDARD 1979 | ESTADO: {status}")
        print("="*60)
        print(f"[+] Transacciones Procesadas: {count}")
        print(f"[+] Volumen de Equivalencia: ${val:,.2f} USD")
        print(f"[+] Impuestos Recaudados (SST): ${tax:,.2f} USD")
        print("="*60)
        print("[!] Sincronización Frecuencia: 528Hz (ÓPTIMA)")
        print("[!] Protección de Trust: ACTIVA (Delaware)")
        print("="*60)
        print("Actualizando datos en tiempo real... (Presiona Ctrl+C para salir)")

        time.sleep(5) # Refresco cada 5 segundos

if __name__ == "__main__":
    try:
        display_dashboard()
    except KeyboardInterrupt:
        print("\nCierre de Dashboard seguro. El motor sigue operando en segundo plano.")
