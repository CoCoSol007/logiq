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

## Example

```Logik
# Types
ty User: {alice, bob, charlie}
ty Role: {admin, client}

# Predicates
pre isRoleOf: User.Role 
pre isManagerOf: User.User 
pre delegated: User.User
pre authorized: User
pre access: User

# Initial facts
fact isRoleOf(alice, admin)
fact isManagerOf(bob, alice)
fact delegated(charlie, bob)

# Rules
rule authorized_if_admin:
  ∀u ∈ User, roleOf(u, admin) => authorized(u)

rule authorization_delegation:
  ∀a ∈ User, ∀b ∈ User,
    delegated(a, b) ∧ authorized(a) => authorized(b)

rule access_requires_auth:
  ∀u ∈ User, authorized(u) => access(u)
```

And then we can query the system:

```Logik
? access(alice)
? access(bob)
? authorized(charlie)
```