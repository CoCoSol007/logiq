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

### Basics
```lgk
# Create a new type
User: {alice}.

# Create predicates
pre isAdmin: User.

isAdmin(alice).
```

then we request the DB

```lgk
isAdmin(alice)? # True
```

### Define a predicate

```lgk
# Create a new type
Interupter: {interupter1}.

# Create predicates
pre isOn: Interupter.
pre isOff: Interupter :-
	!isOn(x1).

isOn(interupter1).
```

then we request the DB

```lgk
isOff(interupter1)? # False
```
### Multi-predicates

```lgk
# Create a new types
User: {alice, bob, moi}.
Role: {Admin, client}.

# Create predicates
pre isAdmin: User.
pre delegated: User, User.
pre isAuthorized: User :- 
	  isAdmin(x1)
	| (delegated(X, x1) & isAdmin(X)).

# Initial facts
isAdmin(alice).
delegated(alice, moi).
```
And then we request
```
isAdmin(moi)? # False
isAdmin(alice)? # true
isAuthorized(moi)? # True
```

### Multi-predicates with recursion

```lgk
# Create a new types
User: {alice, bob, moi}.
Role: {Admin, client}.

# Create predicates
pre isAdmin: User.
pre delegated: User, User.
pre isAuthorized: User :- 
	  isAdmin(x1)
	| (delegated(X, x1) & isAuthorized(X)).

# Initial facts
isAdmin(alice).
delegated(alice, moi).
delegated(moi, bob).
```
And then we request
```
isAdmin(moi)? # False
isAdmin(bob)? # False
isAdmin(alice)? # true
isAuthorized(moi)? # True
isAuthorized(bob)? # True
```

### Generics in request
```lgk
# Create a new types
User: {alice, bob, charlie, zoe}.

# Create predicates
pre link: User, User.
pre isFriend: User, User :-
	  link(x1, x2)
	| link(x2, x1).

link(alice, bob).
link(charlie, bob).
link(zoe, charlie).
```

request

```
isFriend(X, bob)?
		# alice & charlie
```