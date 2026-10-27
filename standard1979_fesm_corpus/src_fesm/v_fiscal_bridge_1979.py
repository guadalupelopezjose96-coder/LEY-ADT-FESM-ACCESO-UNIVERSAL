import hashlib
import os

# Definición de la ruta del archivo crítico
FILE_PATH = "v_fiscal_bridge_1979.py"

# Hash esperado (debes actualizar este valor tras cada commit validado)
EXPECTED_HASH = "b8191d47a9ba716551b23caee66375cc881a920d0ce5e84004f7f2779c824829"

def verify_integrity():
    if not os.path.exists(FILE_PATH):
        print("ERROR: Archivo no encontrado.")
        return False
    
    sha256_hash = hashlib.sha256()
    with open(FILE_PATH, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
            
    if sha256_hash.hexdigest() == EXPECTED_HASH:
        print("VALIDACIÓN EXITOSA: Justicia Maximisisisima garantizada.")
        return True
    else:
        print("ALERTA DE SEGURIDAD: Inconsistencia detectada en el bridge fiscal.")
        return False

if __name__ == "__main__":
    verify_integrity()
