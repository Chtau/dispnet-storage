use criterion::{criterion_group, criterion_main, Criterion};
use dispnet_shared::Package;
use dispnet_storage::{policy::{PolicyManager, PolicyRule, PolicyType, TriggerPolicy, PolicyTrigger, IncomingPolicy, LayerPolicy}, storage_manager::StorageManager, filestorage::FileStorageProvider, StorageProvider};

const FILE_STORAGE: &str = "test_fstore";
const DELETE_STORAGE: &str = "test_fdelete";
const FILE_KEY: &str = "1234";

fn clean_up(test_key: &str) {
    let f_path = format!("{}_{}", FILE_STORAGE, test_key);
    let d_path = format!("{}_{}", DELETE_STORAGE, test_key);
    let attr = std::fs::metadata(&f_path).unwrap();
    if attr.is_dir() {
        std::fs::remove_dir_all(&f_path).unwrap();
    }
    let attr = std::fs::metadata(&d_path).unwrap();
    if attr.is_dir() {
        std::fs::remove_dir_all(&d_path).unwrap();
    }
}

fn storage_provider_instance(test_key: &str) -> Box<dyn StorageProvider> {
    let f_path = format!("{}_{}", FILE_STORAGE, test_key);
    let d_path = format!("{}_{}", DELETE_STORAGE, test_key);

    Box::new(FileStorageProvider::new(f_path.to_owned(), d_path.to_owned()))
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

fn get_from_provider_manager() {
    let f_key = "get_provider";
    let mut manager = StorageManager::new();
    manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
    let _save_result = manager.save("layer1", FILE_KEY, "test".to_owned().into_bytes());
    let result = manager.get("layer1", FILE_KEY).unwrap();
    assert_eq!(result.size, 4);
    assert_eq!(result.key, FILE_KEY);
    clean_up(f_key);
}


fn find_in_provider_manager() {
    let f_key = "find_provider";
    let mut manager = StorageManager::new();
    manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
    let _save_result = manager.save("layer1", FILE_KEY, "test".to_owned().into_bytes());
    let result = manager.find(FILE_KEY).unwrap();
    assert_eq!(result.size, 4);
    assert_eq!(result.key, FILE_KEY);
    clean_up(f_key);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Policy validate trigger", |b| b.iter(|| policy_validate_trigger()));
    c.bench_function("Policy validate incoming", |b| b.iter(|| policy_validate_incoming()));
    c.bench_function("Policy resolve layer", |b| b.iter(|| policy_resolve_layer()));
    c.bench_function("Get result from storage provider manager", |b| b.iter(|| get_from_provider_manager()));
    c.bench_function("Find result in storage provider manager", |b| b.iter(|| find_in_provider_manager()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
