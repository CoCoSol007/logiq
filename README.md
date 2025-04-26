<a id="readme-top"></a>
<div align="center">

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <img src="logo.png" width="400">
  <p align="center">
    <br />
    Logik is a strongly-typed logic interpreter written in Kotlin.
  </p>
</div>

---

**Logik** is a strongly-typed logic interpreter written in **Kotlin**, designed to model and evaluate systems based on propositional and first-order logic. It supports the declaration of types, constants, functions, predicates, axioms, and initial facts, and enables the inference of logical truths through deduction.

## Features

- Declarative syntax inspired by formal logic and rule-based systems
- Custom domain types (e.g., `User`, `Role`)
- Typed function and predicate declarations
- First-order quantification (`∀` and `∃`) and standard logical connectives (`->`, `&`, `~`, etc...)
- Axiomatic reasoning
- Logical query evaluation (true/false)

## Example

```logik
Types:
    User
    Role

Constants:
    Alice : User
    Bob : User
    Charlie : User
    Admin : Role

Functions:
    roleOf(User) : Role
    managerOf(User) : User

Predicates:
    Authorized(User)
    Access(User)
    Delegated(User, User)

Axioms:
    ∀u:User, roleOf(u) = Admin -> Authorized(u)
    ∀u,v:User, Authorized(u) & Delegated(u,v) -> Authorized(v)
    ∀u:User, Authorized(managerOf(u)) -> Authorized(u)
    ∀u:User, Authorized(u) -> Access(u)

Init:
    roleOf(Alice) = Admin
    managerOf(Charlie) = Bob
    Delegated(Alice, Bob)

Eval:
    Access(Alice)       // True
    Access(Bob)         // True
    Authorized(Charlie) // True
```