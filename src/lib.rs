#![forbid(unsafe_code)]

//! How communication protocols evolve over time in balanced ternary {-1, 0, +1} systems.
//!
//! Models signal pools, meaning negotiation, grammar competition, signal drift,
//! creolization (dialect merging), and minimal proto-languages — all in ternary space.

use std::collections::HashMap;

/// A ternary value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ternary {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Ternary {
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Neg),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Pos),
            _ => None,
        }
    }

    pub fn to_i8(self) -> i8 {
        self as i8
    }
}

/// A signal in the communication system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Signal {
    pub name: String,
    pub value: Ternary,
}

impl Signal {
    pub fn new(name: &str, value: Ternary) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

/// Pool of available signals that agents can use.
#[derive(Debug, Clone)]
pub struct SignalPool {
    signals: HashMap<String, Signal>,
    generation: u64,
}

impl SignalPool {
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
            generation: 0,
        }
    }

    /// Add a signal to the pool.
    pub fn add(&mut self, name: &str, value: Ternary) {
        self.signals.insert(name.to_string(), Signal::new(name, value));
    }

    /// Remove a signal.
    pub fn remove(&mut self, name: &str) -> bool {
        self.signals.remove(name).is_some()
    }

    /// Get a signal by name.
    pub fn get(&self, name: &str) -> Option<&Signal> {
        self.signals.get(name)
    }

    /// All signal names.
    pub fn names(&self) -> Vec<&str> {
        self.signals.keys().map(|s| s.as_str()).collect()
    }

    /// Number of signals.
    pub fn len(&self) -> usize {
        self.signals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }

    /// Advance to next generation.
    pub fn evolve(&mut self) {
        self.generation += 1;
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Mutate a signal's value (simulates random drift).
    pub fn mutate(&mut self, name: &str, new_value: Ternary) -> bool {
        if let Some(signal) = self.signals.get_mut(name) {
            signal.value = new_value;
            self.generation += 1;
            return true;
        }
        false
    }
}

impl Default for SignalPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks how agents negotiate shared meaning for signals.
#[derive(Debug, Clone)]
pub struct MeaningNegotiation {
    /// Agent name → signal name → assigned meaning
    meanings: HashMap<String, HashMap<String, String>>,
}

impl MeaningNegotiation {
    pub fn new() -> Self {
        Self {
            meanings: HashMap::new(),
        }
    }

    /// An agent proposes a meaning for a signal.
    pub fn propose(&mut self, agent: &str, signal: &str, meaning: &str) {
        self.meanings
            .entry(agent.to_string())
            .or_default()
            .insert(signal.to_string(), meaning.to_string());
    }

    /// Get the meaning an agent assigns to a signal.
    pub fn meaning(&self, agent: &str, signal: &str) -> Option<&str> {
        self.meanings.get(agent).and_then(|m| m.get(signal)).map(|s| s.as_str())
    }

    /// Check if two agents agree on a signal's meaning.
    pub fn agrees(&self, agent_a: &str, agent_b: &str, signal: &str) -> bool {
        match (self.meaning(agent_a, signal), self.meaning(agent_b, signal)) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// How many agents agree on a signal's meaning (consensus count).
    pub fn consensus_count(&self, signal: &str) -> usize {
        let mut meaning_counts: HashMap<&str, usize> = HashMap::new();
        for agent_map in self.meanings.values() {
            if let Some(m) = agent_map.get(signal) {
                *meaning_counts.entry(m.as_str()).or_insert(0) += 1;
            }
        }
        *meaning_counts.values().max().unwrap_or(&0)
    }

    /// Total agents participating.
    pub fn agent_count(&self) -> usize {
        self.meanings.len()
    }
}

impl Default for MeaningNegotiation {
    fn default() -> Self {
        Self::new()
    }
}

/// A grammar: a set of rules for combining signals.
#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    rules: Vec<GrammarRule>,
    fitness: f64,
}

/// A grammar rule: pattern → result.
#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub inputs: Vec<Ternary>,
    pub output: Ternary,
    pub description: String,
}

impl Grammar {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rules: Vec::new(),
            fitness: 0.5,
        }
    }

    pub fn add_rule(&mut self, inputs: Vec<Ternary>, output: Ternary, description: &str) {
        self.rules.push(GrammarRule {
            inputs,
            output,
            description: description.to_string(),
        });
    }

    /// Apply the grammar to a sequence of ternary values.
    /// Returns the output of the first matching rule, or None.
    pub fn apply(&self, inputs: &[Ternary]) -> Option<Ternary> {
        for rule in &self.rules {
            if rule.inputs.len() == inputs.len() && rule.inputs.iter().zip(inputs.iter()).all(|(a, b)| a == b) {
                return Some(rule.output);
            }
        }
        None
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn set_fitness(&mut self, f: f64) {
        self.fitness = f;
    }
}

/// Grammars compete; the fittest survive.
#[derive(Debug, Clone)]
pub struct GrammarEvolution {
    grammars: Vec<Grammar>,
    generation: u64,
}

impl GrammarEvolution {
    pub fn new() -> Self {
        Self {
            grammars: Vec::new(),
            generation: 0,
        }
    }

    pub fn add(&mut self, grammar: Grammar) {
        self.grammars.push(grammar);
    }

    /// Run one generation: keep the top N grammars by fitness.
    pub fn select(&mut self, top_n: usize) {
        self.grammars.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap_or(std::cmp::Ordering::Equal));
        self.grammars.truncate(top_n);
        self.generation += 1;
    }

    pub fn grammars(&self) -> &[Grammar] {
        &self.grammars
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn count(&self) -> usize {
        self.grammars.len()
    }

    /// The fittest grammar.
    pub fn fittest(&self) -> Option<&Grammar> {
        self.grammars.iter().max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl Default for GrammarEvolution {
    fn default() -> Self {
        Self::new()
    }
}

/// How signals change meaning over distance (generation hops).
#[derive(Debug, Clone)]
pub struct SignalDrift {
    /// signal_name → list of (distance, value) representing value at each hop
    drift_map: HashMap<String, Vec<(u32, Ternary)>>,
}

impl SignalDrift {
    pub fn new() -> Self {
        Self {
            drift_map: HashMap::new(),
        }
    }

    /// Record the value of a signal at a given distance.
    pub fn record(&mut self, signal: &str, distance: u32, value: Ternary) {
        self.drift_map
            .entry(signal.to_string())
            .or_default()
            .push((distance, value));
    }

    /// Get the value of a signal at a given distance (exact match).
    pub fn value_at(&self, signal: &str, distance: u32) -> Option<Ternary> {
        self.drift_map.get(signal).and_then(|entries| {
            entries.iter().find(|(d, _)| *d == distance).map(|(_, v)| *v)
        })
    }

    /// How much a signal has drifted from distance 0 to max distance.
    pub fn total_drift(&self, signal: &str) -> i32 {
        self.drift_map.get(signal).map_or(0, |entries| {
            if entries.is_empty() {
                return 0;
            }
            let base = entries.iter().filter(|(d, _)| *d == 0).map(|(_, v)| v.to_i8()).next().unwrap_or(0);
            let farthest = entries.iter().map(|(d, v)| (*d, v.to_i8())).max_by_key(|(d, _)| *d).map(|(_, v)| v).unwrap_or(base);
            (farthest - base) as i32
        })
    }

    /// Number of signals being tracked.
    pub fn signal_count(&self) -> usize {
        self.drift_map.len()
    }
}

impl Default for SignalDrift {
    fn default() -> Self {
        Self::new()
    }
}

/// Merge two protocol dialects into a creole (shared language).
#[derive(Debug, Clone)]
pub struct Creolization {
    dialect_a: HashMap<String, Ternary>,
    dialect_b: HashMap<String, Ternary>,
    creole: HashMap<String, Ternary>,
}

impl Creolization {
    pub fn new() -> Self {
        Self {
            dialect_a: HashMap::new(),
            dialect_b: HashMap::new(),
            creole: HashMap::new(),
        }
    }

    /// Add a signal to dialect A.
    pub fn add_a(&mut self, name: &str, value: Ternary) {
        self.dialect_a.insert(name.to_string(), value);
    }

    /// Add a signal to dialect B.
    pub fn add_b(&mut self, name: &str, value: Ternary) {
        self.dialect_b.insert(name.to_string(), value);
    }

    /// Merge: shared signals take the average (ternary-rounded), unique signals are adopted as-is.
    pub fn merge(&mut self) {
        self.creole.clear();
        // Shared signals: average values
        for (name, val_a) in &self.dialect_a {
            if let Some(val_b) = self.dialect_b.get(name) {
                let avg = (val_a.to_i8() as i32 + val_b.to_i8() as i32) / 2;
                let rounded = Ternary::from_i8(avg as i8).unwrap_or(Ternary::Zero);
                self.creole.insert(name.clone(), rounded);
            } else {
                // Only in A
                self.creole.insert(name.clone(), *val_a);
            }
        }
        // Signals only in B
        for (name, val_b) in &self.dialect_b {
            if !self.dialect_a.contains_key(name) {
                self.creole.insert(name.clone(), *val_b);
            }
        }
    }

    /// Get a signal from the merged creole.
    pub fn get(&self, name: &str) -> Option<Ternary> {
        self.creole.get(name).copied()
    }

    /// Number of signals in the creole.
    pub fn creole_size(&self) -> usize {
        self.creole.len()
    }

    /// Signals shared between dialects.
    pub fn shared_signals(&self) -> Vec<&str> {
        self.dialect_a.keys()
            .filter(|k| self.dialect_b.contains_key(*k))
            .map(|s| s.as_str())
            .collect()
    }

    /// How many signals disagree between dialects.
    pub fn disagreement_count(&self) -> usize {
        self.dialect_a.iter()
            .filter(|(k, v)| self.dialect_b.get(*k).map_or(false, |vb| vb != *v))
            .count()
    }
}

impl Default for Creolization {
    fn default() -> Self {
        Self::new()
    }
}

/// A minimal communication system with the fewest possible signals.
#[derive(Debug, Clone)]
pub struct ProtoLanguage {
    signals: HashMap<String, Ternary>,
}

impl ProtoLanguage {
    /// Create the minimal proto-language: three signals for the three ternary values.
    pub fn minimal() -> Self {
        let mut signals = HashMap::new();
        signals.insert("neg".to_string(), Ternary::Neg);
        signals.insert("zero".to_string(), Ternary::Zero);
        signals.insert("pos".to_string(), Ternary::Pos);
        Self { signals }
    }

    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, value: Ternary) {
        self.signals.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<Ternary> {
        self.signals.get(name).copied()
    }

    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Is this a valid proto-language? (at least one signal per ternary value)
    pub fn is_valid(&self) -> bool {
        let has_neg = self.signals.values().any(|v| *v == Ternary::Neg);
        let has_zero = self.signals.values().any(|v| *v == Ternary::Zero);
        let has_pos = self.signals.values().any(|v| *v == Ternary::Pos);
        has_neg && has_zero && has_pos
    }

    /// Compose two signals: combine their values.
    pub fn compose(&self, a: &str, b: &str) -> Option<Ternary> {
        let va = self.signals.get(a)?;
        let vb = self.signals.get(b)?;
        let sum = va.to_i8() + vb.to_i8();
        Ternary::from_i8(sum.clamp(-1, 1))
    }
}

impl Default for ProtoLanguage {
    fn default() -> Self {
        Self::new()
    }
}

/// Top-level: protocol evolution tracking.
#[derive(Debug, Clone)]
pub struct ProtocolEvolution {
    pool: SignalPool,
    negotiation: MeaningNegotiation,
    grammars: GrammarEvolution,
    drift: SignalDrift,
    generation: u64,
}

impl ProtocolEvolution {
    pub fn new() -> Self {
        Self {
            pool: SignalPool::new(),
            negotiation: MeaningNegotiation::new(),
            grammars: GrammarEvolution::new(),
            drift: SignalDrift::new(),
            generation: 0,
        }
    }

    pub fn pool(&self) -> &SignalPool {
        &self.pool
    }

    pub fn pool_mut(&mut self) -> &mut SignalPool {
        &mut self.pool
    }

    pub fn negotiation(&self) -> &MeaningNegotiation {
        &self.negotiation
    }

    pub fn negotiation_mut(&mut self) -> &mut MeaningNegotiation {
        &mut self.negotiation
    }

    pub fn grammars(&self) -> &GrammarEvolution {
        &self.grammars
    }

    pub fn grammars_mut(&mut self) -> &mut GrammarEvolution {
        &mut self.grammars
    }

    pub fn drift(&self) -> &SignalDrift {
        &self.drift
    }

    pub fn drift_mut(&mut self) -> &mut SignalDrift {
        &mut self.drift
    }

    /// Advance one generation: evolve the pool and select grammars.
    pub fn advance(&mut self) {
        self.pool.evolve();
        self.generation += 1;
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

impl Default for ProtocolEvolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_from_i8() {
        assert_eq!(Ternary::from_i8(-1), Some(Ternary::Neg));
        assert_eq!(Ternary::from_i8(0), Some(Ternary::Zero));
        assert_eq!(Ternary::from_i8(1), Some(Ternary::Pos));
        assert_eq!(Ternary::from_i8(2), None);
    }

    #[test]
    fn test_signal_pool_add_get() {
        let mut pool = SignalPool::new();
        pool.add("danger", Ternary::Neg);
        pool.add("safe", Ternary::Pos);
        assert_eq!(pool.get("danger").unwrap().value, Ternary::Neg);
        assert_eq!(pool.get("safe").unwrap().value, Ternary::Pos);
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn test_signal_pool_remove() {
        let mut pool = SignalPool::new();
        pool.add("x", Ternary::Zero);
        assert!(pool.remove("x"));
        assert!(!pool.remove("x"));
        assert!(pool.is_empty());
    }

    #[test]
    fn test_signal_pool_mutate() {
        let mut pool = SignalPool::new();
        pool.add("sig", Ternary::Pos);
        assert!(pool.mutate("sig", Ternary::Neg));
        assert_eq!(pool.get("sig").unwrap().value, Ternary::Neg);
        assert_eq!(pool.generation(), 1); // mutate increments
    }

    #[test]
    fn test_signal_pool_evolve() {
        let mut pool = SignalPool::new();
        assert_eq!(pool.generation(), 0);
        pool.evolve();
        assert_eq!(pool.generation(), 1);
    }

    #[test]
    fn test_meaning_negotiation_propose() {
        let mut mn = MeaningNegotiation::new();
        mn.propose("alice", "beep", "danger");
        mn.propose("bob", "beep", "danger");
        assert_eq!(mn.meaning("alice", "beep"), Some("danger"));
        assert!(mn.agrees("alice", "bob", "beep"));
    }

    #[test]
    fn test_meaning_negotiation_disagree() {
        let mut mn = MeaningNegotiation::new();
        mn.propose("alice", "beep", "danger");
        mn.propose("bob", "beep", "food");
        assert!(!mn.agrees("alice", "bob", "beep"));
    }

    #[test]
    fn test_meaning_consensus_count() {
        let mut mn = MeaningNegotiation::new();
        mn.propose("alice", "beep", "danger");
        mn.propose("bob", "beep", "danger");
        mn.propose("carol", "beep", "food");
        assert_eq!(mn.consensus_count("beep"), 2);
    }

    #[test]
    fn test_grammar_apply_match() {
        let mut g = Grammar::new("basic");
        g.add_rule(vec![Ternary::Pos, Ternary::Neg], Ternary::Zero, "cancel");
        assert_eq!(g.apply(&[Ternary::Pos, Ternary::Neg]), Some(Ternary::Zero));
    }

    #[test]
    fn test_grammar_apply_no_match() {
        let g = Grammar::new("empty");
        assert_eq!(g.apply(&[Ternary::Pos]), None);
    }

    #[test]
    fn test_grammar_fitness() {
        let mut g = Grammar::new("f");
        assert_eq!(g.fitness(), 0.5);
        g.set_fitness(0.9);
        assert_eq!(g.fitness(), 0.9);
    }

    #[test]
    fn test_grammar_evolution_select() {
        let mut ge = GrammarEvolution::new();
        let mut g1 = Grammar::new("weak");
        g1.set_fitness(0.2);
        let mut g2 = Grammar::new("strong");
        g2.set_fitness(0.9);
        let mut g3 = Grammar::new("mid");
        g3.set_fitness(0.5);
        ge.add(g1);
        ge.add(g2);
        ge.add(g3);
        ge.select(2);
        assert_eq!(ge.count(), 2);
        assert_eq!(ge.fittest().unwrap().name, "strong");
    }

    #[test]
    fn test_signal_drift_record_and_value() {
        let mut sd = SignalDrift::new();
        sd.record("beep", 0, Ternary::Pos);
        sd.record("beep", 3, Ternary::Neg);
        assert_eq!(sd.value_at("beep", 0), Some(Ternary::Pos));
        assert_eq!(sd.value_at("beep", 3), Some(Ternary::Neg));
        assert_eq!(sd.total_drift("beep"), -2);
    }

    #[test]
    fn test_signal_drift_no_drift() {
        let sd = SignalDrift::new();
        assert_eq!(sd.total_drift("nonexistent"), 0);
    }

    #[test]
    fn test_creolization_merge_agreed() {
        let mut c = Creolization::new();
        c.add_a("beep", Ternary::Pos);
        c.add_b("beep", Ternary::Pos);
        c.merge();
        assert_eq!(c.get("beep"), Some(Ternary::Pos)); // avg of 1+1/2 = 1
    }

    #[test]
    fn test_creolization_merge_disagreed() {
        let mut c = Creolization::new();
        c.add_a("beep", Ternary::Pos);
        c.add_b("beep", Ternary::Neg);
        c.merge();
        assert_eq!(c.get("beep"), Some(Ternary::Zero)); // avg of 1+(-1)/2 = 0
    }

    #[test]
    fn test_creolization_unique_signals() {
        let mut c = Creolization::new();
        c.add_a("a_only", Ternary::Pos);
        c.add_b("b_only", Ternary::Neg);
        c.add_a("shared", Ternary::Zero);
        c.add_b("shared", Ternary::Zero);
        c.merge();
        assert_eq!(c.creole_size(), 3);
        assert_eq!(c.get("a_only"), Some(Ternary::Pos));
        assert_eq!(c.get("b_only"), Some(Ternary::Neg));
    }

    #[test]
    fn test_creolization_disagreement() {
        let mut c = Creolization::new();
        c.add_a("x", Ternary::Pos);
        c.add_b("x", Ternary::Neg);
        c.add_a("y", Ternary::Zero);
        c.add_b("y", Ternary::Zero);
        assert_eq!(c.disagreement_count(), 1);
        assert_eq!(c.shared_signals().len(), 2);
    }

    #[test]
    fn test_proto_language_minimal() {
        let pl = ProtoLanguage::minimal();
        assert!(pl.is_valid());
        assert_eq!(pl.signal_count(), 3);
        assert_eq!(pl.get("neg"), Some(Ternary::Neg));
    }

    #[test]
    fn test_proto_language_compose() {
        let pl = ProtoLanguage::minimal();
        assert_eq!(pl.compose("pos", "neg"), Some(Ternary::Zero));
        assert_eq!(pl.compose("pos", "pos"), Some(Ternary::Pos));
        assert!(pl.compose("neg", "neg").is_some()); // -2 clamped to -1
    }

    #[test]
    fn test_proto_language_invalid() {
        let mut pl = ProtoLanguage::new();
        pl.add("only_pos", Ternary::Pos);
        assert!(!pl.is_valid());
    }

    #[test]
    fn test_protocol_evolution_advance() {
        let mut pe = ProtocolEvolution::new();
        pe.pool_mut().add("sig", Ternary::Pos);
        pe.advance();
        assert_eq!(pe.generation(), 1);
        assert_eq!(pe.pool().generation(), 1);
    }
}
