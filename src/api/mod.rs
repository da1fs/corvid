//! Shell innards: the loopback UI server and the slim page link.
//!
//! v2.0.0 removed the local automation API (REST + SSE + webhook + page
//! automation ops) as a project red line: no unofficial automation
//! surface toward the embedded chat services. What remained is exactly
//! what the shell needs to feel native: notifications to the pipeline,
//! unread badges, page state for the dots, the call-pill recovery. See
//! docs/RED-LINES.md; a compliant re-introduction path is documented
//! there too (only through something the services authorize in writing).

pub mod link;
pub mod server;
pub use link::reconcile_after_popup_close;
