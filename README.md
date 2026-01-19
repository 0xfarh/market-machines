# Market Machines

*Prediction markets, AMMs, and consensus engines*

---

## Why This Repo Exists

This repository is a **7‑project learning arc** that treats *market design* the same way engineers learn a new programming language:

> primitives → pricing → automation → liquidity → consensus → production

The end goal is to fully understand — and prototype — the architecture behind **Fact Machine–style markets**.

---

## The Learning Arc (Repos)

### 1. `binary-primitives`

**The smallest possible prediction market**

* Binary YES/NO contracts
* Fixed pricing
* Simple resolution

> Teaches: probabilities, payoffs, resolution

---

### 2. `price-by-flow`

**Markets without automation**

* Manual price updates
* Order imbalance
* Supply/demand intuition

> Teaches: price discovery, flow

---

### 3. `rubber-band-amm`

**An intuitive LMSR implementation**

* Automated pricing
* Liquidity parameter (b)
* Bounded loss

> Teaches: LMSR, AMMs, platform risk control

---

### 4. `unified-binary-book`

**Single orderbook for YES/NO markets**

* Kalshi‑style architecture
* Unified pricing
* Market making friendly

> Teaches: orderbooks, spreads, professional liquidity

---

### 5. `spread-harvester`

**Market making on binary events**

* Always‑on quotes
* Inventory management
* Spread capture

> Teaches: Wintermute‑style market making

---

### 6. `consensus-engine`

**Markets that resolve by belief**

* 24‑hour markets
* Social voting
* Reflexivity

> Teaches: consensus markets, manipulation resistance

---

### 7. `fact-machine-v0`

**A 24‑hour LMSR prediction market with majority resolution**

* LMSR liquidity
* Vote‑based resolution
* Full system prototype

> Teaches: end‑to‑end market design

---

## Stack (Consistent Across All Repos)

* **Core logic:** Rust
* **API:** Axum
* **Simulation:** Python (NumPy, Pandas)
* **DB:** Postgres + Redis
* **Frontend:** Next.js + TypeScript
* **Infra:** Docker

Each repo follows the same structure to reduce cognitive load.

---

## Standard Repo Structure

```text
.
├── core/            # Rust market engine
├── api/             # Axum REST API
├── sim/             # Python simulations
├── frontend/        # Minimal UI / dashboards
├── docker-compose.yml
├── README.md
```

---

## How to Use This Repo

1. Start at **binary-primitives**
2. Progress linearly
3. Do not skip projects
4. Each repo has:

   * a clear goal
   * a short README
   * one core idea

This is not about speed.
It is about **intuition accumulation**.


---

## Status

🚧 In Progress

