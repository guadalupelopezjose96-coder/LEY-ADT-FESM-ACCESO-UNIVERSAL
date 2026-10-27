#!/bin/bash
# Protocolo Aura-Tetrax: Sellado en Producción - V4 Final
VALIDADOR_PATH="/data/data/com.termux/files/home/validadores/v_ley_adt_sentencia_pro.py"

echo "[+] Verificando Integridad del Nodo..."

if [ -f "$VALIDADOR_PATH" ]; then
    python3 "$VALIDADOR_PATH" --verify-all
    
    if [ $? -eq 0 ]; then
        echo "[!] Integridad confirmada. Lanzando Motor FESM..."
        python3 "$HOME/v_master_trinity_core.py"
        
        mkdir -p "$HOME/AuraIntelCore/vault"
        mv "$HOME/resultado_blindado.dat" "$HOME/AuraIntelCore/vault/res_$(date +%Y%m%d_%H%M%S).dat"
        echo "[#] Nodo sellado con éxito."
    else
        echo "[CRITICAL] Violación detectada. Nodo bloqueado."
    fi
else
    echo "[!] ERROR: Validador no localizado en $VALIDADOR_PATH"
fi

