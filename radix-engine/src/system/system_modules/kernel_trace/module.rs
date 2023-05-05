use crate::kernel::actor::Actor;
use crate::kernel::call_frame::CallFrameUpdate;
use crate::kernel::kernel_api::KernelInvocation;
use crate::system::module::SystemModule;
use crate::system::system_callback::{SystemConfig, SystemInvocation};
use crate::system::system_callback_api::SystemCallbackObject;
use crate::types::*;
use crate::{errors::RuntimeError, kernel::kernel_api::KernelApi};
use colored::Colorize;
use radix_engine_interface::api::field_lock_api::LockFlags;
use radix_engine_interface::types::{EntityType, LockHandle, NodeId, SubstateKey};
use sbor::rust::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct KernelTraceModule {}

#[macro_export]
macro_rules! log {
    ( $api: expr, $msg: expr $( , $arg:expr )* ) => {
        #[cfg(not(feature = "alloc"))]
        println!("{}[{}] {}", "    ".repeat($api.kernel_get_current_depth()), $api.kernel_get_current_depth(), sbor::rust::format!($msg, $( $arg ),*));
    };
}

#[allow(unused_variables)] // for no_std
impl<V: SystemCallbackObject> SystemModule<SystemConfig<V>> for KernelTraceModule {
    fn before_invoke<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        invocation: &KernelInvocation<SystemInvocation>,
        input_size: usize,
    ) -> Result<(), RuntimeError> {
        let message = format!(
            "Invoking: fn = {:?}, input size = {}",
            invocation.resolved_actor, input_size
        )
        .green();

        log!(api, "{}", message);
        Ok(())
    }

    fn before_push_frame<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        callee: &Actor,
        nodes_and_refs: &mut CallFrameUpdate,
        _args: &IndexedScryptoValue,
    ) -> Result<(), RuntimeError> {
        log!(api, "Sending nodes: {:?}", nodes_and_refs.nodes_to_move);
        log!(api, "Sending refs: {:?}", nodes_and_refs.node_refs_to_copy);
        Ok(())
    }

    fn on_execution_finish<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        nodes_and_refs: &CallFrameUpdate,
    ) -> Result<(), RuntimeError> {
        log!(api, "Returning nodes: {:?}", nodes_and_refs.nodes_to_move);
        log!(
            api,
            "Returning refs: {:?}",
            nodes_and_refs.node_refs_to_copy
        );
        Ok(())
    }

    fn after_invoke<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        output_size: usize,
    ) -> Result<(), RuntimeError> {
        log!(api, "Exiting: output size = {}", output_size);
        Ok(())
    }

    fn on_allocate_node_id<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        node_type: Option<EntityType>,
        virtual_node: bool,
    ) -> Result<(), RuntimeError> {
        log!(
            api,
            "Allocating node id: type = {:?}  virtual = {}",
            node_type,
            virtual_node
        );
        Ok(())
    }

    fn before_create_node<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        node_id: &NodeId,
        node_module_init: &BTreeMap<ModuleNumber, BTreeMap<SubstateKey, IndexedScryptoValue>>,
    ) -> Result<(), RuntimeError> {
        let mut module_substate_keys = BTreeMap::<&ModuleNumber, Vec<&SubstateKey>>::new();
        for (module_id, m) in node_module_init {
            for (substate_key, _) in m {
                module_substate_keys
                    .entry(module_id)
                    .or_default()
                    .push(substate_key);
            }
        }
        let message = format!(
            "Creating node: id = {:?}, type = {:?}, substates = {:?}",
            node_id,
            node_id.entity_type(),
            module_substate_keys
        )
        .red();
        log!(api, "{}", message);
        Ok(())
    }

    fn before_drop_node<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        node_id: &NodeId,
    ) -> Result<(), RuntimeError> {
        log!(api, "Dropping node: id = {:?}", node_id);
        Ok(())
    }

    fn before_lock_substate<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        node_id: &NodeId,
        module_id: &ModuleNumber,
        offset: &SubstateKey,
        flags: &LockFlags,
    ) -> Result<(), RuntimeError> {
        log!(
            api,
            "Locking substate: node id = {:?}, module_id = {:?}, substate_key = {:?}, flags = {:?}",
            node_id,
            module_id,
            offset,
            flags
        );
        Ok(())
    }

    fn after_lock_substate<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        handle: LockHandle,
        first_lock_from_db: bool,
        size: usize,
    ) -> Result<(), RuntimeError> {
        log!(
            api,
            "Substate locked: handle = {:?}, first_lock_from_db = {:?}",
            handle,
            first_lock_from_db
        );
        Ok(())
    }

    fn on_read_substate<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        lock_handle: LockHandle,
        size: usize,
    ) -> Result<(), RuntimeError> {
        log!(
            api,
            "Reading substate: handle = {}, size = {:?}",
            lock_handle,
            size
        );
        Ok(())
    }

    fn on_write_substate<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        lock_handle: LockHandle,
        size: usize,
    ) -> Result<(), RuntimeError> {
        log!(
            api,
            "Writing substate: handle = {}, size = {:?}",
            lock_handle,
            size
        );
        Ok(())
    }

    fn on_drop_lock<Y: KernelApi<SystemConfig<V>>>(
        api: &mut Y,
        lock_handle: LockHandle,
    ) -> Result<(), RuntimeError> {
        log!(api, "Dropping lock: handle = {} ", lock_handle);
        Ok(())
    }
}