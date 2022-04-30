use dispnet_shared::Package;

#[derive(Debug, PartialEq)]
pub enum PolicyTrigger {
    BeforeGet = 0,
    AfterGet = 1,
    BeforeSave = 2,
    AfterSave = 3,
    BeforeDelete = 4,
    AfterDelete = 5,
    BeforeFree = 6,
}

pub struct TriggerPolicy {
    layer: String,
    get_validation_conditions: Vec<PolicyTrigger>,
}

pub struct LayerPolicy {
    success_layer_key: String,
}

pub struct IncomingPolicy {}

pub enum PolicyType {
    Trigger(TriggerPolicy),
    Layer(LayerPolicy),
    Incoming(IncomingPolicy),
}

pub trait Policy {
    fn get_type(self: &Self) -> &PolicyType;
    fn validate(self: &Self, package: &Package, client: &str) -> bool;
}

pub struct PolicyRule {
    name: String,
    policy_type: PolicyType,
    validation_callback: fn(package: &Package, client: &str) -> bool,
}

impl Policy for PolicyRule {
    fn get_type(self: &Self) -> &PolicyType {
        &self.policy_type
    }

    fn validate(self: &Self, package: &Package, client: &str) -> bool {
        (self.validation_callback)(package, client)
    }
}

/// Policy manager for package validation and layer selection.
pub struct PolicyManager {
    incoming_policies: Vec<PolicyRule>,
    layer_policies: Vec<PolicyRule>,
    trigger_policies: Vec<PolicyRule>,
}

impl PolicyManager {
    pub fn new() -> Self {
        Self {
            incoming_policies: vec![],
            layer_policies: vec![],
            trigger_policies: vec![],
        }
    }

    /// Active policies count for incoming package validation.
    pub fn incoming_policies_count(self: &Self) -> usize {
        self.incoming_policies.len()
    }

    /// Active policies count for layer resolving.
    pub fn layer_policies_count(self: &Self) -> usize {
        self.layer_policies.len()
    }

    /// Active policies count for package triggers.
    pub fn trigger_policies_count(self: &Self) -> usize {
        self.trigger_policies.len()
    }

    /// Total active policies count.
    pub fn policies_count(self: &Self) -> usize {
        self.incoming_policies_count() + self.layer_policies_count() + self.trigger_policies_count()
    }

    /// Register a policy, to update a policy call the `remove` function first.
    pub fn add(self: &mut Self, policy: PolicyRule) {
        match &policy.policy_type {
            PolicyType::Incoming(_incoming) => {
                self.incoming_policies.push(policy);
            }
            PolicyType::Layer(_layer) => {
                self.layer_policies.push(policy);
            }
            PolicyType::Trigger(_trigger) => {
                self.trigger_policies.push(policy);
            }
        }
    }

    /// Remove a registered policy.
    pub fn remove(self: &mut Self, name: &str) -> bool {
        if self.incoming_policies.iter().any(|f| f.name == name) {
            let index = self
                .incoming_policies
                .iter()
                .position(|x| x.name == name)
                .unwrap();
            self.incoming_policies.remove(index);
            return true;
        }
        if self.layer_policies.iter().any(|f| f.name == name) {
            let index = self
                .layer_policies
                .iter()
                .position(|x| x.name == name)
                .unwrap();
            self.layer_policies.remove(index);
            return true;
        }
        if self.trigger_policies.iter().any(|f| f.name == name) {
            let index = self
                .trigger_policies
                .iter()
                .position(|x| x.name == name)
                .unwrap();
            self.trigger_policies.remove(index);
            return true;
        }
        false
    }

    /// Clear all registered policies.
    pub fn clear(self: &mut Self) {
        self.incoming_policies.clear();
        self.layer_policies.clear();
        self.trigger_policies.clear();
    }

    /// Validate incoming packages. Only returns `false` if any policy has failed.
    pub fn validate_incoming(self: &Self, package: &Package, client: &str) -> bool {
        for policy in self.incoming_policies.iter() {
            if !policy.validate(package, client) {
                return false;
            }
        }
        true
    }

    /// Resolve the layer name which should be used for the package. Returns `Err` if no policy matches the conditions for the package.
    pub fn resolve_layer(self: &Self, package: &Package, client: &str) -> Result<String, ()> {
        for policy in self.layer_policies.iter() {
            // on a successful validation we switch to the provided layer
            if policy.validate(package, client) {
                if let PolicyType::Layer(layer) = &policy.policy_type {
                    return Ok(layer.success_layer_key.to_owned());
                }
            }
        }
        Err(())
    }

    /// Validation based on trigger events. Only returns `false` if any policy has failed.
    pub fn validate_trigger(
        self: &Self,
        source_layer: &str,
        trigger: &PolicyTrigger,
        package: &Package,
        client: &str,
    ) -> bool {
        for policy in self.trigger_policies.iter().filter(|x| {
            if let PolicyType::Trigger(trigger_policy) = &x.policy_type {
                if trigger_policy.get_validation_conditions.contains(trigger)
                    && trigger_policy.layer == source_layer
                {
                    return true;
                }
            }
            false
        }) {
            // every trigger policy filtered by `PolicyTrigger`
            if !policy.validate(package, client) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{
        IncomingPolicy, LayerPolicy, PolicyManager, PolicyRule, PolicyTrigger, PolicyType,
        TriggerPolicy,
    };
    use dispnet_shared::Package;

    #[test]
    fn add_policies() {
        let mut manager = PolicyManager::new();
        manager.add({
            PolicyRule {
                name: "1".to_owned(),
                policy_type: PolicyType::Incoming(IncomingPolicy {}),
                validation_callback: |_x, _y| true,
            }
        });
        manager.add({
            PolicyRule {
                name: "2".to_owned(),
                policy_type: PolicyType::Layer(LayerPolicy {
                    success_layer_key: "".to_owned(),
                }),
                validation_callback: |_x, _y| true,
            }
        });
        manager.add({
            PolicyRule {
                name: "3".to_owned(),
                policy_type: PolicyType::Trigger(TriggerPolicy {
                    layer: "l1".to_owned(),
                    get_validation_conditions: vec![PolicyTrigger::BeforeSave],
                }),
                validation_callback: |_x, _y| true,
            }
        });
        assert_eq!(manager.policies_count(), 3);
        assert_eq!(manager.incoming_policies_count(), 1);
        assert_eq!(manager.layer_policies_count(), 1);
        assert_eq!(manager.trigger_policies_count(), 1);
    }

    #[test]
    fn remove_policy() {
        let mut manager = PolicyManager::new();
        manager.add({
            PolicyRule {
                name: "1".to_owned(),
                policy_type: PolicyType::Incoming(IncomingPolicy {}),
                validation_callback: |_x, _y| true,
            }
        });
        manager.add({
            PolicyRule {
                name: "2".to_owned(),
                policy_type: PolicyType::Layer(LayerPolicy {
                    success_layer_key: "".to_owned(),
                }),
                validation_callback: |_x, _y| true,
            }
        });
        manager.add({
            PolicyRule {
                name: "3".to_owned(),
                policy_type: PolicyType::Trigger(TriggerPolicy {
                    layer: "l1".to_owned(),
                    get_validation_conditions: vec![PolicyTrigger::BeforeSave],
                }),
                validation_callback: |_x, _y| true,
            }
        });
        assert_eq!(manager.policies_count(), 3);
        manager.remove("1");
        assert_eq!(manager.policies_count(), 2);
        assert_eq!(manager.incoming_policies_count(), 0);

        manager.remove("2");
        assert_eq!(manager.policies_count(), 1);
        assert_eq!(manager.layer_policies_count(), 0);

        manager.remove("3");
        assert_eq!(manager.policies_count(), 0);
        assert_eq!(manager.trigger_policies_count(), 0);
    }

    #[test]
    fn clear_policies() {
        let mut manager = PolicyManager::new();
        manager.add({
            PolicyRule {
                name: "1".to_owned(),
                policy_type: PolicyType::Incoming(IncomingPolicy {}),
                validation_callback: |_x, _y| true,
            }
        });
        manager.add({
            PolicyRule {
                name: "2".to_owned(),
                policy_type: PolicyType::Layer(LayerPolicy {
                    success_layer_key: "".to_owned(),
                }),
                validation_callback: |_x, _y| true,
            }
        });
        assert_eq!(manager.policies_count(), 2);
        manager.clear();
        assert_eq!(manager.policies_count(), 0);
    }

    fn get_package() -> Package {
        Package {
            index: 0,
            checksum: "".to_owned(),
            compression_algorithm: "".to_owned(),
            normalized_size: 0,
            package_id: "1".to_owned(),
            size: 0,
        }
    }

    #[test]
    fn validate_incoming() {
        let mut manager = PolicyManager::new();
        manager.add({
            PolicyRule {
                name: "1".to_owned(),
                policy_type: PolicyType::Incoming(IncomingPolicy {}),
                validation_callback: |_x, _y| true,
            }
        });
        let valid = manager.validate_incoming(
            &get_package(),
            "client",
        );
        assert!(valid);
    }

    #[test]
    fn resolve_layer() {
        let mut manager = PolicyManager::new();
        manager.add({
            PolicyRule {
                name: "2".to_owned(),
                policy_type: PolicyType::Layer(LayerPolicy {
                    success_layer_key: "test1".to_owned(),
                }),
                validation_callback: |_x, _y| true,
            }
        });
        let layer = manager.resolve_layer(
            &get_package(),
            "client",
        ).unwrap();
        assert_eq!(layer, "test1");
    }

    #[test]
    fn validate_trigger() {
        let mut manager = PolicyManager::new();
        manager.add({
            PolicyRule {
                name: "3".to_owned(),
                policy_type: PolicyType::Trigger(TriggerPolicy {
                    layer: "l1".to_owned(),
                    get_validation_conditions: vec![PolicyTrigger::BeforeSave],
                }),
                validation_callback: |_x, _y| false,
            }
        });
        let valid = manager.validate_trigger(
            "l1",
            &PolicyTrigger::BeforeSave,
            &get_package(),
            "client",
        );
        assert!(!valid);

        let not_valid_layer = manager.validate_trigger(
            "l2",
            &PolicyTrigger::BeforeSave,
            &get_package(),
            "client",
        );
        assert!(not_valid_layer);

        let not_valid_event = manager.validate_trigger(
            "l1",
            &PolicyTrigger::AfterSave,
            &get_package(),
            "client",
        );
        assert!(not_valid_event);
    }
}
