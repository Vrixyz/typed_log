use std::any::Any;
use std::{any::TypeId, collections::HashMap};

/// Empty trait that you should implement for anything you intend to log.
///
/// This trait could be avoided, but it serves as a marker for users of your code who would like to provide custom logger:
/// They should implement a logging implementation for all types implementing [Loggable].
pub trait Loggable: Any {}

pub type LogFnType = dyn Fn(&dyn Loggable) + Sync + Send;

pub type LogFn = Box<LogFnType>;

use std::sync::LazyLock;

// Global hashmap more performant to find a unique implementation.
static LOG_MAP: LazyLock<std::sync::RwLock<HashMap<TypeId, Vec<LogFn>>>> =
    LazyLock::new(|| std::sync::RwLock::new(HashMap::new()));
// Global vec, less performant but more versatile.
static LOG_VEC: std::sync::RwLock<Vec<LogFn>> = std::sync::RwLock::new(Vec::new());

pub fn log_any<T: Loggable + 'static>(loggable: &T) {
    {
        let Ok(log_map) = LOG_MAP.read() else {
            return;
        };
        let Some(log_functions) = log_map.get(&TypeId::of::<T>()) else {
            return;
        };
        for log_function in log_functions {
            log_function(loggable);
        }
    }

    let Ok(log_vec) = LOG_VEC.read() else {
        return;
    };
    for log_function in log_vec.iter() {
        log_function(loggable);
    }
}

/// Sets how to log a specific type `T`.
///
/// This appends a log implementation for this specific type `T`: it doesn't override what's been set before through [push_log_impl].
///
/// See [push_log_impl] for a non specific version.
pub fn push_log_impl<T: Loggable + 'static>(log_fn: impl Fn(&T) + Sync + Send + 'static) {
    let mut log_map = match LOG_MAP.write() {
        Ok(lock) => lock,
        Err(_) => return,
    };

    // Wrap into a type-erased dyn Fn(&dyn Loggable)
    let wrapper = move |loggable: &dyn Loggable| {
        // Downcast the loggable to the expected concrete type
        if let Some(concrete) = (loggable as &dyn std::any::Any).downcast_ref::<T>() {
            log_fn(concrete);
        }
    };
    match log_map.entry(TypeId::of::<T>()) {
        std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
            occupied_entry.get_mut().push(Box::new(wrapper));
        }
        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(vec![Box::new(wrapper)]);
        }
    };
}

/// Sets how to log any type.
///
/// This appends a log implementation for any unknown type: it doesn't override what's been set before through [add_log_any].
///
/// Implementors of `log_fn` are responsible for leaving the function as early as possible if its parameter is not expected.
///
/// This is useful to share a logging implementation for multiple types.
pub fn push_log_any(log_fn: impl Fn(&dyn Loggable) + Sync + Send + 'static) {
    let mut log_vec = match LOG_VEC.write() {
        Ok(lock) => lock,
        Err(_) => return,
    };
    log_vec.push(Box::new(log_fn));
}
