use std::{mem::MaybeUninit, sync::OnceLock};

use librelic_sys::{
    core_clean, core_init, ep_param_set_any_pairf, wrapper_bn_t, wrapper_get_order, RLC_OK,
};

pub struct Instance {
    pub order: wrapper_bn_t,
}

impl Instance {
    pub fn new() -> &'static Self {
        static INSTANCE: OnceLock<Instance> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let ret = unsafe { core_init() };
            if ret != RLC_OK {
                panic!("Initialization of relic failed: {}", ret);
            }

            let mut order = MaybeUninit::uninit();
            let ret = unsafe {
                ep_param_set_any_pairf();
                wrapper_get_order(order.as_mut_ptr())
            };
            if ret != RLC_OK {
                panic!("Failed to obtain order: {}", ret);
            }
            let order = unsafe { order.assume_init() };

            Self { order }
        })
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            core_clean();
        }
    }
}
