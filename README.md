# ternary-language-evolution: How communication protocols evolve over time

Models the evolution of communication protocols in balanced ternary {-1, 0, +1} systems. Agents start with a minimal proto-language, negotiate shared meanings, develop competing grammars, and merge dialects through creolization — all tracked across generations.

## Why This Exists

In multi-agent ternary systems, agents need to communicate. But they don't start with a shared language — they develop one. Signals drift in meaning over distance, grammars compete for fitness, and separate groups that develop independently need to merge their dialects. Without a formal model, you get chaos: agents that can't understand each other, protocols that diverge without detection, and no way to measure communication fitness. This crate provides the evolutionary framework.

## Core Concepts

- **Balanced ternary**: Three values: -1 (Neg), 0 (Zero), +1 (Pos). Signals carry ternary values.
- **SignalPool**: The vocabulary of available signals. Signals can be added, removed, and mutated (drift).
- **MeaningNegotiation**: Agents propose meanings for signals. Track who agrees with whom and measure consensus.
- **Grammar**: A set of rules for combining signals. Each rule maps a sequence of ternary inputs to an output. Grammars have a fitness score.
- **GrammarEvolution**: Grammars compete. Each generation, the fittest survive. Like natural selection for communication rules.
- **SignalDrift**: Tracks how a signal's value changes over distance (hops between agents). A signal that starts as Pos might become Neg after several hops.
- **Creolization**: When two dialects meet, they merge. Shared signals take the average value; unique signals are adopted as-is. The result is a creole — a new shared language.
- **ProtoLanguage**: The minimal communication system: one signal per ternary value (neg, zero, pos). All languages evolve from here.

## Quick Start

```toml
[dependencies]
ternary-language-evolution = "0.1"
```

```rust
use ternary_language_evolution::{ProtocolEvolution, ProtoLanguage, Ternary, Creolization};

// Start with the minimal proto-language
let proto = ProtoLanguage::minimal();
assert!(proto.is_valid());

// Two groups develop independently
let mut creole = Creolization::new();
creole.add_a("alert", Ternary::Pos);
creole.add_a("food", Ternary::Zero);
creole.add_b("alert", Ternary::Pos);
creole.add_b("danger", Ternary::Neg);
creole.merge(); // create shared language

// Track the full evolution
let mut evo = ProtocolEvolution::new();
evo.pool_mut().add("beep", Ternary::Pos);
evo.negotiation_mut().propose("alice", "beep", "danger");
evo.negotiation_mut().propose("bob", "beep", "danger");
evo.advance();
```

## API Overview

| Type | Description |
|------|-------------|
| `Ternary` | A ternary value: Neg (-1), Zero (0), Pos (+1) |
| `Signal` | A named signal carrying a ternary value |
| `SignalPool` | Vocabulary of available signals, with generation tracking |
| `MeaningNegotiation` | Tracks agent-proposed meanings and consensus |
| `Grammar` | A set of signal-combination rules with fitness |
| `GrammarRule` | Maps input ternary sequence to output ternary value |
| `GrammarEvolution` | Natural selection among competing grammars |
| `SignalDrift` | Tracks how signal values change over distance |
| `Creolization` | Merges two dialects into a shared creole |
| `ProtoLanguage` | Minimal communication: one signal per ternary value |
| `ProtocolEvolution` | Top-level orchestrator combining all components |

## How It Works

The `ProtocolEvolution` struct ties everything together: a `SignalPool` for vocabulary, a `MeaningNegotiation` for consensus tracking, a `GrammarEvolution` for rule selection, and a `SignalDrift` for measuring degradation. Each call to `advance()` increments the generation counter.

`GrammarEvolution` uses truncation selection: sort grammars by fitness, keep the top N. Simple and effective for small populations. More sophisticated selection (tournament, roulette) would require external dependencies.

`Creolization` averages the ternary values of shared signals. Since ternary has only three values, the average of Pos (+1) and Neg (-1) is Zero — a genuine compromise. Signals unique to one dialect are adopted unchanged into the creole.

`SignalDrift` records (distance, value) pairs. Total drift is the difference between the value at distance 0 and the value at the farthest recorded distance. This quantifies how much a signal degrades as it propagates.

## Known Limitations

- **No probabilistic drift**: Signal mutation is manual, not stochastic. Real language evolution involves random mutation rates.
- **Grammar rules are exact-match only**: No wildcards, no pattern variables. A rule for `[Pos, Neg]` won't match `[Neg, Pos]`.
- **No recursive grammars**: Rules are flat input→output mappings. No tree-structured or recursive compositions.
- **Creolization averaging is crude**: Real creole formation involves social power dynamics, frequency effects, and markedness. Here it's just arithmetic mean.
- **No agent-level modeling**: All agents are strings. No population structure, no spatial distribution, no network topology.
- **Single-threaded**: No concurrency support. Wrap in synchronization primitives for multi-threaded use.

## Use Cases

- **Multi-agent protocol development**: Agents in a ternary fleet develop communication protocols from scratch, negotiating meanings and evolving grammars.
- **Dialect merger**: Two groups that developed independently merge their signaling systems. Track what's shared, what conflicts, and what the creole looks like.
- **Signal degradation analysis**: Measure how much signals drift over distance. Detect when a protocol has degraded beyond usefulness.
- **Emergent language research**: Study how minimal ternary communication systems evolve complexity through selection and creolization.

## Ecosystem Context

Part of the SuperInstance ternary crate family. Relates to:
- `ternary-protocol` (the wire format these evolved languages transmit over)
- `ternary-grammar` (grammar structures this crate evolves)
- `ternary-language` (established languages this crate evolves toward)
- `ternary-agent` (agents that negotiate meanings)

## License

MIT
