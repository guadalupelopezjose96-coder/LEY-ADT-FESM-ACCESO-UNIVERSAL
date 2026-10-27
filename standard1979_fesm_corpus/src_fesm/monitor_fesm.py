import time
import subprocess
import sys

def obtener_carga():
    # 'uptime' devuelve una cadena como: "12:44:14 up 1 day,  2:10,  load average: 3.45, 3.20, 3.15"
    result = subprocess.run(['uptime'], capture_output=True, text=True)
    uptime_str = result.stdout
    # Extraemos la parte de load average
    load_part = uptime_str.split('load average:')[1].strip()
    return load_part

def monitor_fesm():
    print("[!] INICIANDO MONITOR DE RENDIMIENTO FESM (Modo Subprocess)")
    print("[!] Nodo: Termux/Android (Compatible con kernel limitado)")
    print("-" * 55)
    
    try:
        while True:
            carga = obtener_carga()
            # Mostramos la carga directamente
            sys.stdout.write(f"\rEstado del Nodo Aura-Tetrax | Carga (1, 5, 15 min): {carga}")
            sys.stdout.flush()
            time.sleep(2)
            
    except KeyboardInterrupt:
        print("\n\n[+] Monitoreo finalizado. Nodo FESM en espera.")

if __name__ == "__main__":
    monitor_fesm()
