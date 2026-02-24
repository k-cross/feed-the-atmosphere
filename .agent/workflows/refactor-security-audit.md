---
description: Workflow to validate architectural performance, security, and organizational health.
---

# Architectural Refactor & Security Audit (Rust)

This workflow performs a high-level scan of the codebase to identify architectural improvements, performance bottlenecks, and security risks. It is designed for a Rust CLI.

## Trigger
- User asks to "audit the project architecture".
- User asks to "check for high-level refactoring opportunities".
- User asks to "scan for security vulnerabilities" in the whole project.

## Steps

1. **Clarify Focus Area**
   - **Agent Action:** Ask the user where to direct the analysis.
   - *Prompt:* "Would you like me to focus on the **AT Protocol/Bsky Implementation**, the **AI Model**, or the **Integration** between them? Is there a specific component (e.g., authentication, state management) you are concerned about?"

2. **Map the Territory (Context Gathering)**
   - Based on the user's response, map out the relevant structure to understand the architecture.
   - List the project structure: `ls -R src/`
   - Read key configuration: `cat Cargo.toml`
   - Read entry points: `src/main.rs` or `src/lib.rs`.

3. **Run Ecosystem Tools**
   - Use the specific toolchain to gather objective data before manual analysis.
   - **Rust:**
     ```bash
     cargo clippy -- -D warnings  # Check for idiomatic Rust and common errors
     cargo audit                  # Check for security vulnerabilities in dependencies
     ```

4. **Architectural Analysis (Mental Sandbox)**
   Analyze the gathered context and tool output against these pillars:

   -   **Concurrency:** Is `tokio` or `async-std` used efficiently? Are there blocking operations in async contexts?
   -   **Modularity:** Are domain concerns leaking into the API layer? Should logic be extracted into Traits or separate crates?
   -   **Safety:** Are `unsafe` blocks used? If so, are they necessary and documented?

5. **Report & Recommendations**
   - Summarize the **Architectural Health** of the scanned area.
   - List **Top 3 Recommendations** (e.g., "Refactor the Auth middleware to use a custom Extractor").
   - Group findings by **Security**, **Performance**, and **Maintainability**.
