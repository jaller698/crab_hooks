// Documentation: https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks
use strum_macros::{Display, EnumString};

#[derive(Display, Clone, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum HookTypes {
    PreCommit,
    PrepareCommitMsg,
    CommitMsg,
    PostCommit,
    ApplyPatchMsg,  // Email based
    PreApplyPatch,  // Email based
    PostApplyPatch, // Email based
    PreRebase,
    PostRewrite,
    PostCheckout,
    PostMerge,
    PrePush,
    PreAutoGC,   // Pre Garbage collection
    PreReceive,  // Server side
    Update,      // Server side
    PostReceive, // Server side
}
