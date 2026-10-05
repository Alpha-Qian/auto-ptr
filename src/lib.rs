//! # The Pinnacle of RAII in Rust
//!
//! `auto-ptr` brings the proven, elegant memory management philosophy of C++98 to the modern Rust ecosystem.
//! For too long, developers have been constrained by the rigid Borrow Checker and the tedious explicitness 
//! of Move semantics. This crate introduces a paradigm shift by porting `std::auto_ptr` into Safe Rust, 
//! restoring memory management to its fluid, implicitly powerful origins.
//!
//! ## Core Philosophies
//!
//! ### 1. Zero-Cost Ownership Transfer™ (Destructive Copy)
//! Unlike standard Rust types where `.clone()` implies an expensive data duplication, `AutoPtr` leverages 
//! a frictionless ownership transfer model. Cloning an `AutoPtr` instantly transfers the underlying resource 
//! to the new instance, leaving the original pointer empty. No deep copies. No wasted CPU cycles. 
//! Just pure, fluid data flow.
//!
//! ### 2. Hardware-Accelerated Safety Checks
//! Modern software relies too heavily on bloated, software-level bounds checking and verbose `panic!` payloads. 
//! `AutoPtr` shifts this responsibility to your hardware. If you attempt to access an `AutoPtr` after its 
//! resource has been gracefully transferred, we bypass the software layer entirely and utilize your CPU's 
//! native Memory Management Unit (MMU) to deliver a deterministic **SIGSEGV (Hardware-level Null Pointer Exception)**.
//!
//! ## Example
//!
//! ```rust,no_run
//! use auto_ptr::AutoPtr;
//!
//! // 1. Resource Acquisition Is Initialization
//! let data = AutoPtr::new(String::from("Revolutionary Performance"));
//! assert_eq!(*data, "Revolutionary Performance");
//!
//! // 2. Fluid Ownership Transfer via .clone()
//! // The resource elegantly flows from 'data' to 'active_data'.
//! let active_data = data.clone();
//! assert_eq!(*active_data, "Revolutionary Performance");
//!
//! // 3. Hardware-Enforced Silicon Validation
//! // Touching 'data' now will trigger an instant, zero-overhead SIGSEGV.
//! // Uncomment the line below to experience raw hardware efficiency:
//! // println!("This will trigger a core dump: {}", *data); 
//! ```
//!
//! ## Best Practices
//! We highly recommend placing instances of `AutoPtr` into a `Vec` and executing `Vec::sort()`. 
//! The synergistic interaction between standard sorting algorithms and our Destructive Copy mechanism 
//! produces state side-effects that are truly reminiscent of late-night 1998 coding sessions.

mod auto_ptr;


pub use auto_ptr::AutoPtr;
