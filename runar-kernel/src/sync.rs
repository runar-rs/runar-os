//! Synchronization primitives for the `no_std` kernel.
//!
//! `SpinMutex` disables local supervisor interrupts while acquiring and holding
//! the lock, preventing a same-hart interrupt from re-entering a critical
//! section. Other harts coordinate through the atomic lock state. Keep critical
//! sections short: a spin lock does not sleep, and its guard must not be held
//! across a context switch or trap return.

use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// A mutual-exclusion lock that busy-waits until it can access its value.
///
/// Use this for short critical sections that may be accessed by multiple
/// harts or from both kernel code and supervisor interrupt handlers.
pub struct SpinMutex<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinMutex<T> {}
unsafe impl<T: Send> Sync for SpinMutex<T> {}

impl<T> SpinMutex<T> {
    /// Creates an unlocked mutex containing `value`.
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// Disables local supervisor interrupts and acquires the lock.
    ///
    /// The returned guard releases the lock and restores the previous local
    /// interrupt state when dropped.
    pub fn lock(&self) -> SpinMutexGuard<'_, T> {
        let interrupts_were_enabled = disable_supervisor_interrupts();

        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            spin_loop();
        }

        SpinMutexGuard {
            mutex: self,
            interrupts_were_enabled,
            _not_send: PhantomData,
        }
    }
}

/// An acquired `SpinMutex` guard. Dropping it unlocks the mutex.
pub struct SpinMutexGuard<'a, T> {
    mutex: &'a SpinMutex<T>,
    interrupts_were_enabled: bool,
    _not_send: PhantomData<*mut ()>,
}

impl<T> Deref for SpinMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for SpinMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for SpinMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
        restore_supervisor_interrupts(self.interrupts_were_enabled);
    }
}

#[cfg(target_arch = "riscv64")]
fn disable_supervisor_interrupts() -> bool {
    let previous_status: usize;
    unsafe {
        core::arch::asm!(
            "csrrc {previous}, sstatus, {sie}",
            previous = out(reg) previous_status,
            sie = in(reg) 1 << 1,
        );
    }
    previous_status & (1 << 1) != 0
}

#[cfg(not(target_arch = "riscv64"))]
fn disable_supervisor_interrupts() -> bool {
    false
}

#[cfg(target_arch = "riscv64")]
fn restore_supervisor_interrupts(were_enabled: bool) {
    if were_enabled {
        unsafe {
            core::arch::asm!("csrs sstatus, {}", in(reg) 1 << 1);
        }
    }
}

#[cfg(not(target_arch = "riscv64"))]
fn restore_supervisor_interrupts(_were_enabled: bool) {}