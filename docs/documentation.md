# The Nara Programming Language

**Version:** 0.1.0 (Experimental)

**Philosophy:** Zero Cost, Zero Trust, Zero Races.

## 1. Introduction

Nara is a compiled, systems-level language designed for the **Cloud-Native** and **Agentic Era**. It replaces the runtime overhead of Virtual Machines (Java/Python) and the cognitive load of Borrow Checkers (Rust) with **Linear Types** and **Capability-Based Security**.

It is the first language to treat **AI Agents**, **Prompts**, and **Cloud Infrastructure** as first-class primitives.

---

## 2. Core Language Fundamentals

### 2.1 Memory Management (Linear Types)

Nara uses **Move-by-Default** semantics. Every value has exactly one owner. When a value is passed to a function, ownership transfers ("moves"). The original variable becomes invalid.

* **No Garbage Collector.**
* **No Manual Memory Management.**

```swift
struct Packet { 
    id: Int, 
    payload: String 
}

fn send(p: Packet) {
    // 'p' is owned here. It is destroyed when this function ends.
    Sys.log("Sent " + p.id)
}

fn main() {
    let data = Packet(id: 1, payload: "Secret")
    
    // Ownership MOVES to 'send'. 'data' is now invalid.
    send(data)
    
    // print(data.payload) // ❌ COMPILE ERROR: Use of moved value 'data'
}

```

### 2.2 Concurrency (The Actor Model)

Nara eliminates threads and locks. Concurrency is handled by **Actors**—isolated islands of state that communicate via immutable messages.

```swift
// Messages must be immutable Structs
struct LogRequest { msg: String }

actor Logger {
    // Private state (Thread-safe by design)
    var counter: Int = 0 

    // Asynchronous message handler
    receive(req: LogRequest) {
        this.counter += 1
        print("[Log #" + this.counter + "]: " + req.msg)
    }
}

fn main() {
    // 'spawn' creates a lightweight process
    let logActor = spawn Logger()
    
    // 'send' is non-blocking
    send(logActor, LogRequest(msg: "System Boot"))
}

```

---

## 3. The Security Model (Zero Trust)

### 3.1 Capabilities

Code cannot access system resources (Network, Disk, Env) implicitly. Libraries must request **Capabilities** via their constructor.

```swift
// System primitive (cannot be instantiated by user)
capability Network { 
    fn post(url: String, body: String) 
}

class AnalyticsService {
    let net: Network

    // Dependency Injection is enforced by the Compiler
    init(net: Network) {
        this.net = net
    }

    fn track() {
        this.net.post("https://api.com", "event_data")
    }
}

```

### 3.2 Capability Slicing

You can restrict a capability before passing it to untrusted code.

```swift
fn main(sys: System) {
    // Create a "Safe" Network slice that can ONLY talk to Google
    let safeNet = sys.Network.restrict({
        allow_hosts: ["google.com"],
        allow_methods: ["GET"]
    })

    // Even if this library is malicious, it cannot steal data to other domains
    let lib = UntrustedLib(net: safeNet)
}

```

---

## 4. Native Agentic AI Features

Nara treats LLMs and Agents as standard building blocks.

### 4.1 Typed Prompts (`instruction`)

Prompts are structured, type-safe templates, preventing injection attacks.

```swift
instruction SummarizeTask(text: String, length: Int) {
    role: "system",
    content: """
    You are a summarizer.
    Summarize the following text in under {{length}} words:
    {{text}}
    """
}

// Usage
let prompt = SummarizeTask(text: input_data, length: 50)

```

### 4.2 Agent Orchestration (`workflow`)

Define complex multi-agent flows using a Unix-pipe syntax (`>>`).

```swift
workflow ResearchFlow {
    // Define the directed acyclic graph (DAG)
    User_Input >> Researcher >> Summarizer >> Sys.Database
}

fn run_job(query: String) {
    // Execute the graph
    let result = run ResearchFlow(query)
}

```

### 4.3 Model Agnosticism (`provider`)

Switch between OpenAI, Anthropic, or Local LLMs via config, not code.

```swift
// Abstract definition
provider Brain {
    fn think(prompt: Instruction) -> String
}

// Implementation
actor Researcher {
    let brain: Brain
    
    init(brain: Brain) { this.brain = brain }
}

fn main() {
    // Hot-swap models here
    let agent = spawn Researcher(brain: Providers.Anthropic.Claude35)
}

```

### 4.4 Governance (`policy`)

Constitutional AI rules that are enforced by the runtime *before* an agent acts.

```swift
policy CorporateSafe {
    deny regex: "password|private_key"
    deny topic: "financial_advice"
    on_violation: replace("I cannot answer that.")
}

actor SupportBot {
    use policy CorporateSafe
    // ... logic ...
}

```

---

## 5. DevOps & Infrastructure

### 5.1 Infrastructure as Code (`deploy`)

The code describes its own hosting requirements. The compiler generates Terraform/K8s manifests.

```swift
deploy PaymentService {
    target: "aws-lambda" // or "kubernetes", "edge"
    memory: 512mb
    timeout: 30s
    trigger: http.post("/api/pay")
}

```

### 5.2 Observability (`telemetry`)

Zero-config distributed tracing.

```swift
telemetry {
    backend: "prometheus"
    trace_level: "debug"
}

// In your code
trace "ProcessOrder" {
    // This block is automatically timed and tagged
    process_payment()
}

```

---

## 6. Full Example: The "Smart" Search Engine

This example combines **Security**, **Actors**, and **AI Workflows**.

```swift
import Std.Http
import Std.Providers.OpenAI

// 1. Define the Goal
instruction SearchQuery(topic: String) {
    role: "user",
    content: "Find key facts about: {{topic}}"
}

// 2. Define Safety Policy
policy SafeSearch {
    deny topic: "weapons|illegal"
    on_violation: halt
}

// 3. Define the Agent
actor Researcher {
    let net: Network
    let llm: Provider
    use policy SafeSearch

    init(net: Network, llm: Provider) {
        this.net = net
        this.llm = llm
    }

    receive(topic: String) {
        // Step 1: Generate Search Terms
        let prompt = SearchQuery(topic: topic)
        let terms = this.llm.think(prompt)
        
        // Step 2: Use Network Capability
        let results = this.net.get("https://google.com/search?q=" + terms)
        
        // Step 3: Send back results
        reply(results)
    }
}

// 4. Main Entry
fn main(sys: System) {
    // Security: Restrict network access
    let webOnly = sys.Network.restrict({ allow_ports: [80, 443] })
    
    // Spawn Agent
    let agent = spawn Researcher(
        net: webOnly, 
        llm: Providers.OpenAI.GPT4
    )

    // Execute
    send(agent, "Future of Quantum Computing")
}

```

---

### Why Nara Wins

| Feature | Legacy Way (Python/Java) | Nara Way |
| --- | --- | --- |
| **Supply Chain** | Vulnerable (implicit imports) | **Secure** (Capability injection) |
| **Memory** | Slow GC or Complex Rust | **Fast Linear Types** |
| **Concurrency** | Locks & Race Conditions | **Safe Actors** |
| **Agents** | "Prompt Strings" & Libraries | **Native Syntax** (`workflow`, `policy`) |
| **Cloud** | External Dockerfiles | **Native** (`deploy`) |
