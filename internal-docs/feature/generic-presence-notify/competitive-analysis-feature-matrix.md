# Competitive Analysis: Open-Source Presence Detection Tools

*Comparison of open-core features only. Commercial/paid features are excluded from this matrix.*

---

## Comparison Matrix

| Feature | [proximityd](https://github.com/levonk/btnotify) | [ESPresense](https://github.com/ESPresense/ESPresense) | [find3](https://github.com/schollz/find3) | [monitor](https://github.com/andrewjfreyer/monitor) | [OpenMQTTGateway](https://github.com/1technophile/OpenMQTTGateway) | [RuView](https://github.com/ruvnet/RuView) | [Happy Bubbles](https://github.com/happy-bubbles/presence) |
|---------|:------------------------------------------------:|:-------------------------------------------------------:|:-----------------------------------------:|:----------------------------------------------------:|:----------------------------------------------------------------:|:-------------------------------------------:|:----------------------------------------------------------:|
| **License** | ⭐ AGPL-3.0 | ⭐ AGPL-3.0 | ⭐ MIT | ⭐ MIT | ⚠️ GPL-3.0 | ⭐ MIT | ⭐ Apache-2.0 |
| **BLE Scanning** | ⭐ Full (BlueZ, ESP32 nodes) | ⭐ Full (ESP32 native) | ⭐ Full | ⭐ Full (passive + active) | ⭐ Full (ESP32/ESP8266) | ❌ None | ⭐ Full (ESP32) |
| **WiFi Scanning** | ⭐ ARP, ping sweep | ❌ None | ⭐ WiFi + magnetic | ❌ None | ⚠️ Limited (beacons only) | ⭐⭐ WiFi CSI (primary) | ❌ None |
| **Network Discovery** | ⭐ mDNS, ping sweep | ❌ None | ⚠️ Limited | ❌ None | ⚠️ Limited | ❌ None | ❌ None |
| **Party / Person Abstraction** | ⭐⭐ Native `party → device → identifier` | ❌ MAC-only | ⚠️ Grouping | ❌ MAC-only | ❌ MAC-only | ❌ MAC/IP only | ❌ MAC-only |
| **Device Grouping** | ⭐ Multiple identifiers per device | ❌ One MAC = one device | ⚠️ Groups by location | ❌ One MAC = one device | ❌ One MAC = one device | ❌ One MAC/IP = one device | ❌ One MAC = one device |
| **Signal Audit Log** | ⭐ SQLite with queries | ❌ None | ⚠️ Internal (no export) | ❌ None | ❌ None | ❌ None | ❌ None |
| **Correlation Engine** | ⭐ Jaccard + suggestions | ❌ Manual | ⭐⭐ ML (Random Forest) | ❌ Manual | ❌ None | ⚠️ Statistical | ❌ None |
| **Location Model** | ⭐ Building/floor/room/zone | ⚠️ Room-level (ESP32 node) | ⭐⭐ Room-level (ML) | ❌ None | ❌ None | ⭐ Room-level (WiFi CSI) | ⚠️ Room-level (node-based) |
| **Standalone Notifications** | ⭐ Discord, Slack, webhook, MQTT | ❌ Requires Home Assistant | ❌ None | ❌ Requires Home Assistant | ❌ Requires Home Assistant | ❌ None | ❌ Requires Home Assistant |
| **MQTT Publishing** | ⭐ Native | ⭐ Native | ❌ None | ⭐ Native | ⭐⭐ Native (primary) | ❌ None | ⭐ Native |
| **CLI-First** | ⭐⭐ Yes (`proximityd` CLI) | ❌ No | ⚠️ CLI exists | ⚠️ Shell script | ❌ No | ⚠️ CLI exists | ❌ No |
| **Cross-Platform Daemon** | ⭐ Linux (full), macOS/Windows (CLI) | ❌ ESP32 only | ⭐ Linux, macOS, Windows | ⭐ Linux | ❌ ESP32 only | ⭐ Linux | ⭐ Linux |
| **Web UI** | ⭐ Embedded dashboard | ❌ None | ⭐ Yes | ❌ None | ❌ None | ⚠️ Basic | ⭐ Yes |
| **ESP32 Support** | ⭐ Yes (`proximityd-node`) | ⭐⭐ Native | ❌ No | ❌ No | ⭐⭐ Native | ❌ No | ⭐ Native |
| **Multi-Building Federation** | ⭐ MQTT federation | ❌ None | ❌ None | ❌ None | ⚠️ Limited | ❌ None | ❌ None |
| **Active Community** | ☑️ New | ⭐ 1.4k stars, active | ☑️ 4.8k stars, unmaintained | ☑️ 2.1k stars, quiet | ⭐ 4k stars, active | ⭐ 70k stars, viral | ⚠️ 110 stars, abandoned |
| **Setup Difficulty** | ☑️ Medium (Rust build) | ☑️ Medium (flash ESP32) | ⚠️ Hard (ML pipeline) | ☑️ Medium (bash + MQTT) | ☑️ Medium (flash ESP32) | ⚠️ Hard (WiFi CSI) | ☑️ Easy (Docker) |
| **Year Introduced** | 2026 | 2021 | 2018 | 2018 | 2017 | 2025 | 2016 |

---

## Scoring Legend

| Icon | Meaning |
|:----:|---------|
| ⭐⭐ | Best-in-class / unique differentiator |
| ⭐ | Strong support |
| ☑️ | Good / adequate |
| ⚠️ | Partial / limited |
| ❌ | Not available |

---

## Key Findings

### Market Gap: Standalone Identity Broker

No existing open-source tool combines **multi-signal scanning** + **person-level abstraction** + **standalone notifications** without requiring Home Assistant. Every competitor either:

- Requires HA (ESPresense, monitor, OpenMQTTGateway, Happy Bubbles)
- Has no identity layer (all of them except find3, which is unmaintained)
- Has no audit log (all of them)
- Has no CLI-first workflow (all of them)

### Where proximityd Wins (Open-Core)

1. **Identity-first design.** The `party → device → identifier` hierarchy is unique. Other tools track MACs; `proximityd` tracks *people*.
2. **Signal audit log.** SQLite append-only log enables debugging, forensics, and offline analysis that no competitor offers.
3. **Standalone operation.** Notifications work out of the box without Home Assistant, MQTT broker, or cloud service.
4. **CLI + Web UI.** Both interfaces are first-class. Competitors are either web-only or CLI-only.
5. **Cross-platform CLI.** `discover`, `status`, and `export` commands run on macOS and Windows even though the daemon requires Linux.

### Where Competitors Win (Open-Core)

| Competitor | Their Strength | Our Response |
|------------|---------------|--------------|
| **ESPresense** | Deep Apple IRK enrollment for private BLE MACs | Document IRK enrollment as a future enhancement; use hostname/mDNS as fallback |
| **find3** | ML-based room-level positioning | Our correlation engine is simpler but actionable; find3 is unmaintained |
| **RuView** | WiFi CSI for high-precision spatial tracking | We cover the 90% use case (presence vs. absence) with simpler signals |
| **OpenMQTTGateway** | Broad protocol support (IR, RF, BLE, LoRa) | We are focused, not broad; users can run both side by side |

---

## Open-Source Sustainability

| Project | License | Commercial Option | Funding Model |
|---------|:-------:|:-------------------:|---------------|
| proximityd | AGPL-3.0 | ✅ Dual license | Community + commercial licenses |
| ESPresense | AGPL-3.0 | ⚠️ Hardware only | Theengs hardware sales |
| find3 | MIT | ❌ None | None (unmaintained) |
| monitor | MIT | ❌ None | None |
| OpenMQTTGateway | GPL-3.0 | ⚠️ Hardware only | Theengs hardware sales |
| RuView | MIT | ❌ None | None (viral, no revenue) |
| Happy Bubbles | Apache-2.0 | ❌ None | None (abandoned) |

**Observation:** Every project in this space is either abandoned, hardware-dependent, or has no sustainable funding model. `proximityd`'s AGPL + dual-license approach is designed to solve this by creating a direct commercial incentive to maintain the open-source core.

---

*Document version: 1.0*
*Last updated: 2026-06-02*
