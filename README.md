<a id="readme-top"></a>
<div align="center">

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <img src="logo.png" width="400">
  <p align="center">
    <br />
    Logik is a strongly-typed logic interpreter written in Rust.
  </p>
</div>

---

**Logik** is a strongly-typed logic interpreter written in **Rust**, designed to model and evaluate systems based on propositional and first-order logic. It supports the declaration of types, constants, functions, predicates, axioms, and initial facts, and enables the inference of logical truths through deduction.

## Features

- Declarative syntax inspired by formal logic and rule-based systems
- Custom domain types (e.g., `User`, `Role`)
- Typed function and predicate declarations
- First-order quantification (`∀` and `∃`) and standard logical connectives (`->`, `&`, `~`, etc...)
- Axiomatic reasoning
- Logical query evaluation (true/false)

## Example

```Logik
TYPES {
  User: {alice, bob, charlie}
  Role: {admin, client}
}
  

FUNCTIONS {
  roleOf: User -> Role
  managerOf: User -> User
  delegated: User.User -> boolean
  authorized: User -> boolean
  access: User -> boolean
}

AXIOMS {
  ∀u ∈ User, roleOf(u) = admin => authorized(u)
  ∀u,v ∈ User.User, authorized(u) ∧ delegated(u,v) => authorized(v)
  ∀u ∈ User, authorized(managerOf(u)) => authorized(u)
  ∀u ∈ User, authorized(u) => access(u)
}

INIT {
  roleOf(alice) := admin
  managerOf(charlie) := bob
  delegated(alice, bob)
  authorized(alice)
  authorized(bob)
}

QUERY {
  access(alice)
  access(bob)
  authorized(charlie)
}
```
