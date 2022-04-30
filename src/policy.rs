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

pub struct PolicyManager {
    incoming_policies: Vec<PolicyRule>,
    layer_policies: Vec<PolicyRule>,
    trigger_policies: Vec<PolicyRule>,
}

impl PolicyManager {
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

    pub fn validate_incoming(self: &Self, package: &Package, client: &str) -> bool {
        for policy in self.incoming_policies.iter() {
            if !policy.validate(package, client) {
                return false;
            }
        }
        true
    }

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

    pub fn validate_trigger(
        self: &Self,
        trigger: &PolicyTrigger,
        package: &Package,
        client: &str,
    ) -> bool {
        for policy in self.trigger_policies.iter().filter(|x| {
            if let PolicyType::Trigger(trigger_policy) = &x.policy_type {
                if trigger_policy.get_validation_conditions.contains(trigger) {
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
