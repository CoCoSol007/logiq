<a id="readme-top"></a>
<div align="center">

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <img src="https://raw.githubusercontent.com/CoCoSol007/logiq/main/logo.png" width="400">
  <p align="center">
    <br />
    <strong>A Domain-Specific Language for SAT Solving</strong>
  </p>
</div>

## About The Project

logiq is a Domain-Specific Language (DSL) designed for expressing and solving Boolean satisfiability (SAT) problems. The project provides an intuitive syntax for writing logical propositions and automatically determines their satisfiability using advanced SAT solving algorithms.

## Installation
If you have Rust and Cargo installed, you can install logiq using Cargo:
```bash
cargo install logiq
```

Alternatively, you can download precompiled binaries from the [releases page](https://github.com/CoCoSol007/logiq/releases) based on your operating system.

## Language Syntax

### Basic Elements

#### Boolean Values
```
true    // Boolean true
false   // Boolean false
```

#### Variables
Variables are identifiers that can hold boolean values:
```
x       // Variable named 'x'
flag    // Variable named 'flag'
P1      // Variable named 'P1'
```

### Logical Operators

#### Basic Operators
```
not     // Negation (¬)
and     // Conjunction (∧)
or      // Disjunction (∨)
```

#### Advanced Operators
```
->      // Implication (→)
<->     // Biconditional/Equivalence (↔)
```

### Operator Precedence
From highest to lowest precedence:
1. `not` (Negation)
2. `and` (Conjunction)
3. `or` (Disjunction)
4. `->` (Implication)
5. `<->` (Equivalence)

### Grouping
Use parentheses to override default precedence:
```
(A or B) and C
not (P -> Q)
```

## Example Expressions

### Simple Propositions
```
// Basic boolean operations
A and B
x or y
not P

// Using parentheses for clarity
(A or B) and not C
```

### Implications
```
// If P then Q
P -> Q

// Equivalent to: not P or Q
not rainy -> sunny
```

### Biconditionals
```
// P if and only if Q
P <-> Q

// Equivalent to: (P -> Q) and (Q -> P)
raining <-> wet_ground
```

### Complex Expressions
```
// De Morgan's laws
not (A and B) <-> (not A or not B)

// Logical puzzle
(A -> B) and (B -> C) and A and not C
```

## Usage


**Important**: Each line in the file represents a separate rule or clause that must be satisfied. When solving, the SAT solver will find an assignment that makes ALL rules true simultaneously.

For example, if your file contains:
```
A or B
not A or C
B -> not C
```

The solver will find values for A, B, and C such that:
- `A or B` is true AND
- `not A or C` is true AND  
- `B -> not C` is true

This is equivalent to solving the conjunction: `(A or B) and (not A or C) and (B -> not C)`.


### Command Line Interface

Run a logical expression directly:
```bash
logiq run "A and B or not C"
```

Parse and evaluate from a file:
```bash
logiq run-file examples/puzzle.logic
```

### File Format
Create files with your logical expressions:

```
// examples/simple
// This is a comment
P and Q
not (R -> S)

// Multiple expressions on separate lines
A or B
C <-> D
```

### Results interpretation
The output will indicate whether the expression is satisfiable and provide all possible assignments of variables if it is.

for example, for ``logiq run "A and Not A"``:
```
The proposition is unsatisfiable.
```

But for ``logiq run "A <-> B"``:
```
The proposition is satisfiable.
Possible assignments:

-- Possibility #1 --
  A = true
  B = true

-- Possibility #2 --
  A = false
  B = false
```