---
description: This describes the process for designing a new feature.
---

Phase 1: Analysis & Preparation
  1. Ingest Specifications: Read and analyze the provided feature specifications.
  2. Validation Check: Determine if the specifications are complete and unambiguous.
    - If Yes: Proceed to Phase 2.
    - If No: Generates a list of clarifying questions and pause for user input.
Phase 2: Implementation Loop Goal: Achieve a working implementation that passes existing standards.
  1. Code Modification: Write or modify code to implement the feature logic.
  2. Linting & Static Analysis: Run linting tools. Fix any syntax or style violations immediately.
  3. Regression Testing: Run the existing test suite.
  4. Error Correction:
    - If errors found: Analyze the error, apply a fix, and return to step 2 (Linting).
    - If no errors: Proceed to Phase 3.
Phase 3: Verification & Coverage
  1. New Test Generation: Create new unit/integration tests specifically targeting the logic added in Phase 2.
  2. Final Verification: Run all tests (new + existing).
  3. Completion: If all tests pass, output the final diff/codebase.
