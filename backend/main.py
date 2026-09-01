"""Phase 1 backend placeholder. It intentionally starts no services."""
from services.contracts import ServiceRegistry

if __name__ == "__main__":
    print("VEX backend services are not enabled until their respective phase.")
    print(ServiceRegistry.phase_one_status())
