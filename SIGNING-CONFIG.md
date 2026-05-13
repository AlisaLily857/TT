# Code Signing Configuration for OmniBox

## GitHub Secrets Setup

Add the following secrets to your GitHub repository (`Settings > Secrets and variables > Actions > New repository secret`):

### Windows
| Secret Name | Value | Source |
|-------------|-------|--------|
| `WINDOWS_CERTIFICATE` | Base64 of `signing/windows.pfx` | `base64 -w 0 signing/windows.pfx` |
| `WINDOWS_CERTIFICATE_PASSWORD` | `omnibox` | Generated |

### macOS
| Secret Name | Value | Source |
|-------------|-------|--------|
| `APPLE_CERTIFICATE` | Base64 of `signing/macos.p12` | `base64 -w 0 signing/macos.p12` |
| `APPLE_CERTIFICATE_PASSWORD` | `omnibox` | Generated |

> Note: macOS code signing also requires an Apple Developer account for notarization. Self-signed certificates allow signing but users will see a security warning. For distribution, apply for an Apple Developer account (`$99/year`) and use `APPLE_ID`, `APPLE_PASSWORD`, and `APPLE_TEAM_ID` for notarization.

### Linux (Flatpak - Optional)
For Flatpak distribution with GPG signing:

| Secret Name | Value | Source |
|-------------|-------|--------|
| `GPG_PRIVATE_KEY` | Base64 of `signing/omnibox.gpg.pub` (private) | `gpg --armor --export-secret-keys "OmniBox Release" \| base64 -w 0` |
| `GPG_PASSPHRASE` | *(no passphrase)* | Generated |

---

## Quick Copy: Base64 Values

Run these commands in the `signing/` directory:

```bash
# Windows certificate
echo "WINDOWS_CERTIFICATE:"
base64 -w 0 windows.pfx
echo

# macOS certificate
echo "APPLE_CERTIFICATE:"
base64 -w 0 macos.p12
echo

# GPG private key
echo "GPG_PRIVATE_KEY:"
gpg --armor --export-secret-keys "OmniBox Release" | base64 -w 0
echo
```

---

## Important Notes

1. **Self-signed certificates** are used for development/testing. End users will see security warnings.
2. **Production releases** require certificates from trusted CAs:
   - **Windows**: DigiCert, Sectigo, or Microsoft Trusted Certificate
   - **macOS**: Apple Developer ID certificate + notarization
   - **Linux**: GPG signing (our self-signed GPG key is acceptable for community repos)
3. **Certificate files** in `signing/` directory should NEVER be committed to git. They are in `.gitignore`.
4. **Password**: All test certificates use password `omnibox`. Change this for production.
