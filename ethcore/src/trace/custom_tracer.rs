use trace::{
    trace::{RewardType, VMTrace},
    FlatTrace, Tracer, VMTracer,
};
use ethereum_types::{Address, U256};
use vm::{ActionParams, Error};
use bytes::ToPretty;

pub struct CustomTracer {
    pub calls: Vec<String>,
}

impl CustomTracer {
    pub fn new() -> Self {
        CustomTracer {
            calls: vec![],
        }
    }
}
impl Tracer for CustomTracer {
    type Output = String;

    fn prepare_trace_call(&mut self, params: &ActionParams, depth: usize, is_builtin: bool) {
        let to = params.address;
        let from = params.sender;
        let code = params.data.as_ref().map(|d|d.to_vec().to_hex()).unwrap_or(String::from(""));
        let value = params.value.value();
        self.calls.push(format!("{:x}::{:x}::{}::{}", to, from, code, value));
    }

    fn prepare_trace_create(&mut self, params: &ActionParams) {
    }

    fn done_trace_call(&mut self, gas_used: U256, output: &[u8]) {
    }

    fn done_trace_create(&mut self, gas_used: U256, code: &[u8], address: Address) {
    }

    fn done_trace_failed(&mut self, error: &Error) {
    }

    fn trace_suicide(&mut self, address: Address, balance: U256, refund_address: Address) {
    }

    fn trace_reward(&mut self, author: Address, value: U256, reward_type: RewardType) {
    }

    fn drain(self) -> Vec<Self::Output> {
        self.calls
    }
}

pub struct CustomVmTracer;

impl VMTracer for CustomVmTracer {
    type Output = ();

    fn trace_next_instruction(&mut self, _pc: usize, _instruction: u8, _current_gas: U256) -> bool {
        true
    }

    fn drain(self) -> Option<Self::Output> {
        Some(())
    }
}