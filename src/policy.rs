use dispnet_shared::Package;

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
    layer_key: String,
    get_validation_conditions: Vec<PolicyTrigger>,
}

pub struct LayerPolicy {
    success_layer_key: String,
}

pub struct IncomingPolicy {

}

pub enum PolicyType {
    Trigger(TriggerPolicy),
    Layer(LayerPolicy),
    Incoming(IncomingPolicy),
}

pub trait Policy {
    fn get_type() -> PolicyType;
    fn validate(package: Package, client: String) -> bool;
}