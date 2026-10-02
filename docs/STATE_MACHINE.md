# State machine

The lifecycle is Draft -> VerificationPending -> Verified -> SeedLive -> SeedClosed -> BondingLive -> RaiseClosed -> Graduating -> Graduated. Draft and pending verification may be cancelled. SeedLive, SeedClosed, BondingLive, and RaiseClosed may reach Failed or Refundable where allowed; Failed may become Refundable. Terminal statuses have no outgoing transitions. The backend domain and Anchor `ProjectStatus::can_transition_to` encode this graph.

`submit_for_verification` is founder-only and moves Draft to VerificationPending. `approve_project` is admin-only and accepts only VerificationPending, then records Verified. `launch_seed` requires founder authority and Verified status. Buying requires BondingLive; refunds require Failed or Refundable; milestone claims require RaiseClosed; graduation is permissionless and requires RaiseClosed. Those financial instructions check preconditions and return `NOT_IMPLEMENTED`; no money or lifecycle transition is performed by them.

All state-changing instructions require the protocol not to be paused, except `set_paused` can unpause. Initialization is restricted to the upgrade authority and starts paused. Admin verification and pause events are emitted.
