// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// [`Relay`] options
#[derive(Debug, Clone)]
pub struct RelayOptions {
    /// Allow/disallow read actions
    read: Arc<AtomicBool>,
    /// Allow/disallow write actions
    write: Arc<AtomicBool>,
}

impl Default for RelayOptions {
    fn default() -> Self {
        Self::new(true, true)
    }
}

impl RelayOptions {
    /// New [`RelayOptions`]
    pub fn new(read: bool, write: bool) -> Self {
        Self {
            read: Arc::new(AtomicBool::new(read)),
            write: Arc::new(AtomicBool::new(write)),
        }
    }

    /// Get read option
    pub fn read(&self) -> bool {
        self.read.load(Ordering::SeqCst)
    }

    /// Set read option
    pub fn set_read(&self, read: bool) {
        let _ = self
            .read
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| Some(read));
    }

    /// Get write option
    pub fn write(&self) -> bool {
        self.write.load(Ordering::SeqCst)
    }

    /// Set write option
    pub fn set_write(&self, write: bool) {
        let _ = self
            .write
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| Some(write));
    }
}

/// Filter options
#[derive(Debug, Clone, Copy, Default)]
pub enum FilterOptions {
    /// Exit on EOSE
    #[default]
    ExitOnEOSE,
    /// After EOSE is received, keep listening for N more events that match the filter, then return
    WaitForEventsAfterEOSE(u16),
    /// After EOSE is received, keep listening for matching events for [`Duration`] more time, then return
    WaitDurationAfterEOSE(Duration),
}

/// Relay Pool Options
#[derive(Debug, Clone, Copy)]
pub struct RelayPoolOptions {
    /// Notification channel size (default: 1024)
    pub notification_channel_size: usize,
    /// Task channel size (default: 1024)
    pub task_channel_size: usize,
    /// Max seen events by Task thread (default: 100000)
    ///
    /// A lower number can cause receiving in notification channel
    /// the same event multiple times
    pub task_max_seen_events: usize,
    /// Shutdown on drop
    pub shutdown_on_drop: bool,
}

impl Default for RelayPoolOptions {
    fn default() -> Self {
        Self {
            notification_channel_size: 1024,
            task_channel_size: 1024,
            task_max_seen_events: 100_000,
            shutdown_on_drop: false,
        }
    }
}
