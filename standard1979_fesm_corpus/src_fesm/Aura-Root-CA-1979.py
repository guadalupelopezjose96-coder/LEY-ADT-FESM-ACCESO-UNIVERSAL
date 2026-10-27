import json
import hashlib
from datetime import datetime, timedelta
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.exceptions import InvalidSignature

class SelloEquivalencia1979:
    def __init__(self):
        # 1. GÉNESIS ZERO-TRUST: Generación de la Clave Privada Estatal (100% Inmutable)
        # En producción, esta clave reside físicamente en un Hardware Security Module (HSM).
        self._private_key = ed25519.Ed25519PrivateKey.generate()
        self.public_key = self._private_key.public_key()
        
    def _calcular_hash_ia(self, binario_ia_crudo: bytes) -> str:
        # Extrae la huella digital forense y exacta de la Inteligencia Artificial
        return hashlib.sha256(binario_ia_crudo).hexdigest()

    def emitir_certificado(self, nombre_corp: str, binario_ia: bytes, rho_aut_auditado: float) -> dict:
        """
        Ejecución del Artículo 9: Emisión del Sello tras auditoría matemática 100% exitosa.
        """
        hash_inmutable = self._calcular_hash_ia(binario_ia)
        fecha_emision = datetime.utcnow()
        # Validez determinista estricta de 1 ciclo fiscal exacto (365 días)
        fecha_expiracion = fecha_emision + timedelta(days=365)
        
        # Payload: El Contrato Legal Criptográfico
        payload_legal = {
            "autoridad": "AURA ROOT CA - THE 1979 STANDARD",
            "corporacion": nombre_corp,
            "hash_ia_autorizado": hash_inmutable,
            "rho_aut_aprobado": rho_aut_auditado,
            "emision_iso": fecha_emision.isoformat(),
            "expiracion_iso": fecha_expiracion.isoformat()
        }
        
        # Serialización 100% determinista para garantizar integridad de bytes
        payload_bytes = json.dumps(payload_legal, sort_keys=True).encode('utf-8')
        
        # Firma Criptográfica (El Sello Inquebrantable del Estado)
        firma = self._private_key.sign(payload_bytes)
        
        return {
            "payload": payload_legal,
            "firma_hex": firma.hex()
        }

    def verificar_transaccion_en_borde(self, certificado: dict, binario_ia_ejecucion: bytes) -> bool:
        """
        Ejecución del Artículo 10: Validación Zero-Trust en los Nodos de Borde.
        """
        payload_legal = certificado["payload"]
        firma_bytes = bytes.fromhex(certificado["firma_hex"])
        payload_bytes = json.dumps(payload_legal, sort_keys=True).encode('utf-8')
        
        # 1. Auditoría Legal de Expiración
        expiracion = datetime.fromisoformat(payload_legal["expiracion_iso"])
        if datetime.utcnow() > expiracion:
            print("❌ BARRERA ZERO-TRUST: Certificado Expirado. Multa activada.")
            return False
            
        # 2. Auditoría Forense de Mutación (Verificación del Hash)
        hash_tiempo_real = self._calcular_hash_ia(binario_ia_ejecucion)
        if hash_tiempo_real != payload_legal["hash_ia_autorizado"]:
            print("❌ FRAUDE FORENSE 100%: El código de la IA fue alterado post-certificación. Sello Roto.")
            return False
            
        # 3. Auditoría Criptográfica Asimétrica (Ed25519)
        try:
            self.public_key.verify(firma_bytes, payload_bytes)
            print("✅ CUMPLIMIENTO 100%: Identidad corporativa y Hash validados. Transacción legal autorizada.")
            return True
        except InvalidSignature:
            print("❌ FALSIFICACIÓN: Firma estatal inválida.")
            return False


# =====================================================================
# EJECUCIÓN FORENSE DEL PROTOCOLO AL 100% 
# =====================================================================

# 1. Instanciamos la Autoridad
root_ca = SelloEquivalencia1979()

# 2. La corporación envía su modelo IA (simulado en bytes) y recibe el Sello
codigo_ia_aprobado = b"def ia_logistica(): optimizar_rutas(eficiencia=5.0)"
sello_oficial = root_ca.emitir_certificado("Logistics Corp LLC", codigo_ia_aprobado, rho_aut_auditado=5.0)

print("--- SELLO DE EQUIVALENCIA 100% EMITIDO ---")
print(json.dumps(sello_oficial, indent=2))
print("-" * 40)

# 3. PRUEBA LEGAL: La IA opera sin modificaciones (Pasa al 100%)
print("\n[Nodo de Borde] IA opera con código intacto:")
root_ca.verificar_transaccion_en_borde(sello_oficial, codigo_ia_aprobado)

# 4. PRUEBA DE HACKEO: La corporación modifica 1 solo byte para evadir impuestos (Rechazo al 100%)
codigo_ia_hackeado = b"def ia_logistica(): optimizar_rutas(eficiencia=9.0)"
print("\n[Nodo de Borde] IA intenta operar tras ser modificada en secreto:")
root_ca.verificar_transaccion_en_borde(sello_oficial, codigo_ia_hackeado)
