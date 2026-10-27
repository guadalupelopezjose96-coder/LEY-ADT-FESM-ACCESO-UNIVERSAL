#!/bin/bash
# startup_fesm.sh - Secuencia de Arranque Soberano FESM 1979

echo "[+] Verificando Integridad del Nodo..."

# 1. Verificación de hash contra el registro
if ! ./check_integrity.sh; then
    echo "[CRITICAL] Violación detectada en la estructura del nodo."
    echo "[!] ALERTA CRÍTICA: Integridad comprometida. Iniciando Protocolo de Purga..."
    ./v_purgas_automaticas.sh # Llama a tus módulos de limpieza
    echo "[!] Nodo saneado. Autoinmunidad activada."
    exit 1
fi

echo "[*] Ejecutando validación de soberanía digital..."

# 2. Carga de validadores críticos (con manejo de errores)
VALIDADOR="./validadores/v_ley_adt_sentencia_pro.py"

if [ -f "$VALIDADOR" ]; then
    python3 "$VALIDADOR" --verify-mode
    if [ $? -ne 0 ]; then
        echo "[!] Error: El validador de sentencia falló la prueba de seguridad."
        exit 1
    fi
else
    echo "[!] Error Crítico: Validador no encontrado en $VALIDADOR"
    exit 1
fi

# 3. Lanzamiento del Motor de 88 núcleos
echo "[+] Integridad validada. Iniciando clúster FESM..."
./bunker/start_jegl_core.sh --cores 88 --audit-mode on

echo "[+] Sistema FESM 1979 operando al 100%."
