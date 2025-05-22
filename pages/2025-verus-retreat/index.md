# Verus Retreat @ Secure Foundations Lab, CMU -- May 27-30 2025

We are organizing a retreat for the [Verus](https://github.com/verus-lang/verus) project involving researchers and practitioners involved in the development of Verus, Verus-related research, and Verus-based projects.

Bryan Parno (@parno) and the Secure Foundations Lab will be hosting the retreat at Carnegie Mellon University during the week of May 26th 2025.

We are planning 3 days of "official" semi-structured time (Tuesday the 27th through Thursday the 29th), and there will be informal meet-ups and discussion on the Friday after (30th).

Sessions will begin at 9:30 AM.  We will be meeting in the [Collaborative Innovation Center (CIC)](https://www.cylab.cmu.edu/about/visiting.html), room 2101.


---

# Program

This is a tentative program.

## Tuesday, May 27th

### Overview of Recently Added Verus Features  
**Expected:** 9:30 AM - 10:30 AM ET

Over 125 issues closed and 230 PRs merged since the last Retreat!

Presentations:
- **`cargo verus`**
- **Proof closures**
- **Exec termination checking**
- **Safe API checker**
- **No cheating mode**
- **Better foundational definitions of Seq/Set/Map**
- **`returns` clause**
- **Overflowable integer library**
- **Attribute-based annotations**
- **Logical atomicity + fractional resource libraries**
- **Verus analyzer improvements**

---

### Experience Reports: Building Systems in Verus - Part 1  
**Expected:** 10:30 AM ET - Lunchtime

Each report should include:
- What was built/verified  
- What Verus did well  
- What problems were encountered / features that would have been useful

Systems:
- Verified OS and hardware model
- Migrating a Dafny project to Verus
- Automatically generating Verus from AADL and SysMLv2 models
- Verifying the Rust standard library

---

### Lunch

---

### Experience Reports: Building Systems in Verus - Part 2  
**Expected:** 12:45 - 2 PM

Each report should include:
- What was built/verified  
- What Verus did well  
- What problems were encountered / features that would have been useful

Systems:
- Trait-based verification of high-performance parsers and serializers with Vest (Yi Cai)  
- Verus as a backend for the Noir language (Aristotelis Papanis)  
- Automatically stabilizing proofs (Amar Shah)  
- Verus + AI (Shan Lu)

---

### Feature Request Triage  
**Expected:** 2 PM - 4 PM ET  

See the [projects list](https://github.com/orgs/verus-lang/projects/1/views/1)

Examples:
- `requires`/`ensures` for spec functions  
- Type invariants  
- Overloaded `Deref`  
- Iterators

---

### Retreat Dinner  
**6:30 PM** at **Butterjoint**

---

## Wednesday, May 28th

### Documentation Write-a-thon  
**Expected:** 9:30 AM (2–3 hours in the morning)  
See guidelines and sign up here: **Verus Retreat 2025 -- Documentation Write-a-Thon**

---

### Group Photo  
**Expected:** 1:00 PM

---

### Feature Design Discussion  
**Expected:** 1:15 PM - 5:30 PM

Topics:
- Lean integration
- `cargo-verus`
  - Should it build Verus’s dependencies (e.g., `vstd`) or use pre-compiles?  
- Unified Deep + Shallow View  
- Finite/Infinite Map/Set distinctions
- Trait extensions and specs for std traits
  - Support for `Cmp`/`Eq`/`PartialEq`?
- `async` support  
- "Next-generation of Tracked/Ghost code", design and implementation  
- Verification by inlining (a la Kani?)
- "Best-effort" bug finding mode (extended static checking)
- `vstd` organization & documentation
- Broader support for `&mut`
- Unwind specs
- Module composition using sharded state machines
- `VerusSync` syntax

---

### Evening Activity  
**Board Game Night**

---

## Thursday, May 29th

### Feature Design Discussion Continued

- **Information flow**
- **11:30 AM:** Attribute-based syntax
- Supporting Rust proposal for contract syntax
- Brainstorming next big steps for Verus

Ongoing Research Work:
- Ownership logic automation
- Tunable automation

---

### Hike in Frick Park  
**~2 hours in the late afternoon**

---

## Friday, May 30th

### Informal meetings and discussions

### Internal Issue Triage

Topics:
- Replacing lifetime generation  
- Using our own type-checker for ghost code
- Verus + Leaf (foundations of Verus’s concurrency mechanisms)
