#![cfg_attr(target_os = "none", no_std)]

pub(crate) const NUM_REGS: usize = 32;
pub(crate) const BITWIDTH: usize = 256;
pub(crate) const NUM_WINDOWS: usize = 16;
pub const RF_SIZE_IN_U32: usize = NUM_REGS*(BITWIDTH/32); // 32 registers, 256 bits/register/32 bits per u32
pub const TOTAL_RF_SIZE_IN_U32: usize = NUM_REGS*(BITWIDTH/32)*NUM_WINDOWS; // 32 registers, 256 bits/register/32 bits per u32, times 16 windows

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy)]
pub struct Job {
    /// If present the SID of the server to which we should return results asynchronously.
    /// If None, then the job will run synchronously.
    pub id: Option<[u32; 4]>,
    /// start location for microcode load
    pub uc_start: u32,
    /// length of the microcode to run
    pub uc_len: u32,
    /// microcode program
    pub ucode: [u32; 1024],
    /// initial register file contents (also contains any arguments to the program)
    pub rf: [u32; RF_SIZE_IN_U32],
    /// which register window, if any, to use for the job
    pub window: Option<u8>,
}

pub enum EngineError {
    InternalError,
    UnsupportedFeature,
    ServerNotFound,
    EngineBusy,
    IllegalOpcode,
    UnknownError,
}

pub trait XousEngine25519 {
    fn new() -> Self;
    fn run_job(&mut self, job: Job) -> Result<[u32; RF_SIZE_IN_U32], EngineError>;
}
