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

    pub fn incoming_policies_count(self: &Self) -> usize {
        self.incoming_policies.len()
    }

    pub fn layer_policies_count(self: &Self) -> usize {
        self.layer_policies.len()
    }

    pub fn trigger_policies_count(self: &Self) -> usize {
        self.trigger_policies.len()
    }

    pub fn policies_count(self: &Self) -> usize {
        self.incoming_policies_count() + self.layer_policies_count() + self.trigger_policies_count()
    }

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

    pub fn remove(self: &mut Self, name: String) -> bool {
        if self.incoming_policies.iter().any(|f|f.name == name) {
            let index = self.incoming_policies.iter().position(|x|x.name == name).unwrap();
            self.incoming_policies.remove(index);
            return true;
        }
        if self.layer_policies.iter().any(|f|f.name == name) {
            let index = self.layer_policies.iter().position(|x|x.name == name).unwrap();
            self.layer_policies.remove(index);
            return true;
        }
        if self.trigger_policies.iter().any(|f|f.name == name) {
            let index = self.trigger_policies.iter().position(|x|x.name == name).unwrap();
            self.trigger_policies.remove(index);
            return true;
        }
        false
    }

    pub fn clear(self: &mut Self) -> bool {

        false
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
        source_layer: &str,
        trigger: &PolicyTrigger,
        package: &Package,
        client: &str,
    ) -> bool {
        for policy in self.trigger_policies.iter().filter(|x| {
            if let PolicyType::Trigger(trigger_policy) = &x.policy_type {
                if trigger_policy.get_validation_conditions.contains(trigger) && trigger_policy.layer == source_layer {
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
    use super::{PolicyManager, IncomingPolicy, PolicyType, PolicyRule, LayerPolicy, TriggerPolicy, PolicyTrigger};


    #[test]
    fn add_policies() {
        let mut manager = PolicyManager::new();
        manager.add({
            PolicyRule { 
                name: "1".to_owned(),
                policy_type: PolicyType::Incoming(IncomingPolicy {}), 
                validation_callback: |_x, _y|true
            }
        });
        manager.add({
            PolicyRule { 
                name: "2".to_owned(),
                policy_type: PolicyType::Layer(LayerPolicy { success_layer_key: "".to_owned() }), 
                validation_callback: |_x, _y|true
            }
        });
        manager.add({
            PolicyRule {
                name: "3".to_owned(), 
                policy_type: PolicyType::Trigger(TriggerPolicy { layer: "l1".to_owned(), get_validation_conditions: vec![PolicyTrigger::BeforeSave] }),
                validation_callback: |_x, _y|true
            }
        });
        assert_eq!(manager.policies_count(), 3);
        assert_eq!(manager.incoming_policies_count(), 1);
        assert_eq!(manager.layer_policies_count(), 1);
        assert_eq!(manager.trigger_policies_count(), 1);
    }

}
