//! Health monitoring for RPC endpoints

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use url::Url;
use metrics::{counter, histogram};

#[derive(Debug, Clone)]
pub struct EndpointHealth {
    pub url: Url,
    pub response_time: Duration,
    pub error_count: u64,
    pub success_count: u64,
    pub last_error: Option<String>,
    pub last_success: Option<Instant>,
    pub is_healthy: bool,
}

impl EndpointHealth {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            response_time: Duration::from_millis(0),
            error_count: 0,
            success_count: 0,
            last_error: None,
            last_success: None,
            is_healthy: true,
        }
    }

    pub fn record_success(&mut self, response_time: Duration) {
        self.success_count += 1;
        self.response_time = response_time;
        self.last_success = Some(Instant::now());
        self.is_healthy = true;

        // Record metrics
        counter!("rpc_success_total", 1, "endpoint" => self.url.to_string());
        histogram!("rpc_response_time_seconds", response_time.as_secs_f64(), "endpoint" => self.url.to_string());
    }

    pub fn record_error(&mut self, error: String) {
        self.error_count += 1;
        self.last_error = Some(error);
        self.is_healthy = false;

        // Record metrics
        counter!("rpc_error_total", 1, "endpoint" => self.url.to_string());
    }

    pub fn health_score(&self) -> f64 {
        let total = self.success_count + self.error_count;
        if total == 0 {
            return 1.0;
        }

        let success_rate = self.success_count as f64 / total as f64;
        let avg_response_time = self.response_time.as_millis() as f64;
        
        // Add time-based degradation
        let time_factor = if let Some(last_success) = self.last_success {
            let time_since_last_success = last_success.elapsed();
            if time_since_last_success > Duration::from_secs(60) {
                0.5 // Degrade score if no success in last minute
            } else {
                1.0
            }
        } else {
            0.5
        };
        
        // Score based on success rate, response time, and time factor
        success_rate * (1000.0 / (avg_response_time + 1.0)) * time_factor
    }

    pub fn should_retry(&self) -> bool {
        // Allow retry if healthy or if error count is below threshold
        self.is_healthy || self.error_count < 3
    }
}

#[derive(Debug, Clone)]
pub struct HealthMonitor {
    endpoints: Arc<RwLock<Vec<EndpointHealth>>>,
}

impl HealthMonitor {
    pub fn new(endpoints: Vec<Url>) -> Self {
        let health_data = endpoints
            .into_iter()
            .map(EndpointHealth::new)
            .collect();

        Self {
            endpoints: Arc::new(RwLock::new(health_data)),
        }
    }

    pub async fn record_success(&self, url: &Url, response_time: Duration) {
        let mut endpoints = self.endpoints.write().await;
        if let Some(endpoint) = endpoints.iter_mut().find(|e| e.url == *url) {
            endpoint.record_success(response_time);
        }
    }

    pub async fn record_error(&self, url: &Url, error: String) {
        let mut endpoints = self.endpoints.write().await;
        if let Some(endpoint) = endpoints.iter_mut().find(|e| e.url == *url) {
            endpoint.record_error(error);
        }
    }

    pub async fn get_healthiest_endpoint(&self) -> Option<Url> {
        let endpoints = self.endpoints.read().await;
        endpoints
            .iter()
            .filter(|e| e.should_retry())
            .max_by(|a, b| a.health_score().partial_cmp(&b.health_score()).unwrap())
            .map(|e| e.url.clone())
    }

    pub async fn get_health_stats(&self) -> Vec<EndpointHealth> {
        self.endpoints.read().await.clone()
    }
} 