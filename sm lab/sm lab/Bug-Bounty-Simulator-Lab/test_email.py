#!/usr/bin/env python3
"""
Test script to verify TOTP functionality
Run this to test TOTP generation and verification
"""

import pyotp
import qrcode
import time

def test_totp():
    print("🔐 TOTP Test - Generating secret and QR code")
    print("=" * 50)
    
    # Generate secret
    secret = pyotp.random_base32()
    print(f"Secret: {secret}")
    
    # Create TOTP instance
    totp = pyotp.TOTP(secret)
    
    # Generate provisioning URI
    uri = totp.provisioning_uri(
        name="test@example.com",
        issuer_name="Bug Bounty Lab Test"
    )
    
    print(f"\nProvisioning URI: {uri}")
    
    # Generate QR code
    qr = qrcode.QRCode(version=1, box_size=10, border=5)
    qr.add_data(uri)
    qr.make(fit=True)
    
    print("\n📱 QR Code generated (scan with authenticator app):")
    qr.print_ascii()
    
    # Generate current TOTP
    current_otp = totp.now()
    print(f"\n🔢 Current TOTP: {current_otp}")
    
    # Test verification
    is_valid = totp.verify(current_otp)
    print(f"✅ Verification result: {is_valid}")
    
    # Show time remaining
    remaining = 30 - (int(time.time()) % 30)
    print(f"⏰ Code expires in: {remaining} seconds")
    
    return True

if __name__ == "__main__":
    test_totp()