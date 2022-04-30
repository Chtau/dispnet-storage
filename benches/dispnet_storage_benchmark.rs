use criterion::{criterion_group, criterion_main, Criterion};
use dispnet_shared::Package;
use dispnet_storage::policy::{PolicyManager, PolicyRule, PolicyType, TriggerPolicy, PolicyTrigger, IncomingPolicy, LayerPolicy};

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

fn policy_validate_trigger() {
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
    assert!(!manager.validate_trigger("l1", &PolicyTrigger::BeforeSave, &get_package(), "client"));
}

fn policy_validate_incoming() {
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

fn policy_resolve_layer() {
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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Policy validate trigger", |b| b.iter(|| policy_validate_trigger()));
    c.bench_function("Policy validate incoming", |b| b.iter(|| policy_validate_incoming()));
    c.bench_function("Policy resolve layer", |b| b.iter(|| policy_resolve_layer()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
