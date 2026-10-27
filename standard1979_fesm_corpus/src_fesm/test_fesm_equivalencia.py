from decimal import Decimal
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import rsa, padding
from core.fesm_engine import ejecutar_transaccion_mercado, TransaccionResultado


class Entidad:
    def __init__(self, id_entidad, saldo, clave_privada=None, clave_publica=None):
        self.id = id_entidad
        self.saldo = Decimal(saldo)
        self.clave_privada = clave_privada
        self.clave_publica = clave_publica


def firmar_transaccion(clave_privada, mensaje: bytes) -> bytes:
    """Genera la firma digital usando RSA-PSS con SHA256."""
    return clave_privada.sign(
        mensaje,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.MAX_LENGTH,
        ),
        hashes.SHA256(),
    )


def test_ley_equivalencia_fiscal_transaccion_basica():
    # 1. Generar par de claves RSA válido de 2048 bits para las pruebas
    priv_emisor = rsa.generate_private_key(
        public_exponent=65537,
        key_size=2048,
    )
    pub_emisor = priv_emisor.public_key()

    # 2. Configuración de Entidades con sus propiedades id y clave_publica
    emisor = Entidad(
        "USR001", "1000.00", clave_privada=priv_emisor, clave_publica=pub_emisor
    )
    receptor = Entidad("USR002", "500.00")
    tesoreria = Entidad("TES001", "0.00")

    monto = Decimal("250.00")
    costo = Decimal("2.00")

    # 3. Preparar mensaje binario exacto
    mensaje = f"{emisor.id}:{receptor.id}:{monto}".encode()

    # 4. Generar firma criptográfica
    firma = firmar_transaccion(emisor.clave_privada, mensaje)

    # 5. Ejecutar la transacción
    resultado = ejecutar_transaccion_mercado(
        emisor, receptor, tesoreria, monto, costo, firma
    )

    # 6. Aserciones de estado y balance
    assert resultado.estado == "EXITO"
    assert emisor.saldo == Decimal("748.00")
    assert receptor.saldo == Decimal("750.00")
    assert tesoreria.saldo == Decimal("2.00")
