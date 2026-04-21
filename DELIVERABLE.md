#  DELIVERABLE — Flash Wallet SDK

> Plan B Network Bitcoin Developer Track 2026
> **Author:** Elisée Assinou — Cotonou, Bénin 

---

##  GitHub Repository

**URL:** https://github.com/elisee-assinou/flash-wallet-sdk

- License: MIT (Open Source)
- Language: Rust (backend), TypeScript (frontend + SDK)
- Status: Working MVP on Polar regtest

---

## 📹 Demo Video

> ⏳ Coming soon — will be added before final submission

---

## 🌐 Live Demo

> ⏳ Coming soon — ngrok or VPS deployment

---

##  Documentation

| Document | Description |
|----------|-------------|
| [README.md](./README.md) | Full project documentation |
| [Backend README](./packages/wallet-backend/README.md) | Backend API docs |
| [Swagger UI](http://localhost:8080/swagger-ui) | Interactive API explorer |

---

##  Assignment Coverage

### Axe 1 — Wallet
| Requirement | Status |
|-------------|--------|
| Lightning Address System |  LNURL-pay compatible Phoenix/Alby/Zeus |
| Auto-convert sats → XOF via Flash SELL_BITCOIN |  via SubscribeInvoices() gRPC |
| Configurable conversion ratio |  10% to 100% slider |
| Manual balance conversion |  POST /api/v1/wallet/balance/convert |
| Real-time balance (WebSocket) |  ws://host/ws/balance |
| Multi-user support |  isolated by lightning_address |
| Transaction history |  PENDING / COMPLETED / FAILED |

### Axe 2 — Business Integration
| Requirement | Status |
|-------------|--------|
| SDK / Package system |  @flash-wallet/merchant-sdk (npm + CDN) |
| Easy integration (non-technical users) |  3 lines of HTML/JS |
| QR code payment |  qrcode.js |
| Real-time webhook notifications |  payment.received / completed / failed |
| Multi-merchant support |  identified by lightning_address |

### Axe 3 — Onboarding Experience
| Requirement | Status                            |
|-------------|-----------------------------------|
| Login by Lightning Address | exact domain match                |
| Wallet configuration form | validation + error messages       |
| Mobile responsive | tested on Android                 |
| UX feedback (loading, errors, success) | clean                             |
| Swagger UI |  http://localhost:8080/swagger-ui |

---

## ️ Architecture Summary

3 Packages:
├── wallet-backend  → Rust/Axum + LND gRPC + Flash API
├── wallet-frontend → React + TypeScript + TailwindCSS
└── merchant-sdk    → TypeScript (CDN + npm)
4 Bounded Contexts (DDD):
├── conversion  → Flash SELL/BUY Bitcoin
├── wallet      → User config + balance
├── lightning   → LND + LNURL-pay + AutoConvert
└── merchant    → SDK + webhooks

---

##  Future Improvements

### Short Term (Post-Lugano)
| Feature | Description |
|---------|-------------|
| Mainnet LND deployment | Real Lightning channels with actual Bitcoin |
| JWT Authentication | Secure token-based auth per user |
| Multi-country MoMo | Togo (Togocel), Côte d'Ivoire (MTN/Moov) |
| Transaction COMPLETED detection | Poll Flash API for confirmed payouts |
| Rate limiting | Protect API endpoints from abuse |

### Medium Term
| Feature | Description |
|---------|-------------|
| React Native mobile app | iOS + Android native experience |
| LNURL-withdraw | Let users withdraw sats to external wallets |
| Taproot Assets | Issue stablecoins on Lightning |
| Fedimint integration | Federated custody for community wallets |
| Email/SMS notifications | Notify users on every payment |

### Long Term
| Feature | Description |
|---------|-------------|
| Non-custodial architecture | Remove custody risk with LDK |
| EMI License | Regulatory compliance for mainnet |
| Open-source contribution | Contribute back to LDK / Fedimint |
| Europe-Africa corridor | France-Bénin remittance via Lightning |
| SDK for WooCommerce/Shopify | E-commerce plugins for West Africa |

---

##  Lessons Learned

Flash is NOT a wallet — it is a Bitcoin/MoMo exchange platform. 

→ The SDK builds a custodial layer ON TOP of Flash
LND TLS in Rust requires tonic_lnd (not raw tonic)

→ Always use "localhost" DNS name, not "127.0.0.1"

3 separate LND clients are necessary to avoid mutex deadlock:

     invoice client, listener client, convert client
Flash API uses HTTP/1.1 — reqwest must send proper body

     Use serde_json body string, not .json() method
Balance source of truth = LND (not the database)

    Database can be corrupted; LND invoices cannot be faked

## Resources Used

- [Flash API Documentation](https://docs.bitcoinflash.xyz)
- [Lightning Address Protocol](https://lightningaddress.com/)
- [BOLT Specifications](https://github.com/lightning/bolts)
- [Plan B Network](https://planb.academy)
- [Polar](https://lightningpolar.com/)
- [Mastering Bitcoin — Andreas Antonopoulos](https://github.com/bitcoinbook/bitcoinbook)

---

##  Contact

**Elisée Assinou**
GitHub: [@elisee-assinou](https://github.com/elisee-assinou)
Mentor: [@maurientz](https://t.me/maurientz)