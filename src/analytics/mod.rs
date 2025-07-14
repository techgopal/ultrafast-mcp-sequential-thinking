//! # Analytics Module
//!
//! Analytics and metrics functionality for the sequential thinking system.
//!
//! This module provides comprehensive analytics capabilities including
//! session analysis, performance metrics, and insights generation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

use crate::thinking::{ThoughtData, ThinkingStats, ThinkingProgress};

/// Analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Whether analytics is enabled
    pub enabled: bool,
    /// Analytics endpoint
    pub endpoint: String,
    /// Analytics API key
    pub api_key: Option<String>,
    /// Metrics collection interval in seconds
    pub collection_interval: u64,
    /// Whether to collect detailed metrics
    pub detailed_metrics: bool,
    /// Retention period for metrics in days
    pub retention_days: u32,
    /// Whether to anonymize data
    pub anonymize_data: bool,
    /// Export analytics data
    pub export_analytics: bool,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "http://localhost:9090".to_string(),
            api_key: None,
            collection_interval: 60,
            detailed_metrics: true,
            retention_days: 30,
            anonymize_data: false,
            export_analytics: false,
        }
    }
}

/// Session analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAnalytics {
    /// Session ID
    pub session_id: String,
    /// Session title
    pub session_title: String,
    /// Analysis timestamp
    pub analyzed_at: DateTime<Utc>,
    /// Basic metrics
    pub basic_metrics: BasicMetrics,
    /// Thinking patterns
    pub thinking_patterns: ThinkingPatterns,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Quality metrics
    pub quality_metrics: QualityMetrics,
    /// Insights
    pub insights: Vec<Insight>,
    /// Recommendations
    pub recommendations: Vec<Recommendation>,
}

/// Basic session metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicMetrics {
    /// Total thoughts
    pub total_thoughts: u32,
    /// Total revisions
    pub total_revisions: u32,
    /// Total branches
    pub total_branches: u32,
    /// Session duration in seconds
    pub session_duration: u64,
    /// Average thought length
    pub avg_thought_length: f64,
    /// Completion rate
    pub completion_rate: f64,
    /// Efficiency score
    pub efficiency_score: f64,
}

/// Thinking patterns analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingPatterns {
    /// Revision frequency
    pub revision_frequency: f64,
    /// Branching frequency
    pub branching_frequency: f64,
    /// Thought complexity trend
    pub complexity_trend: ComplexityTrend,
    /// Thinking style
    pub thinking_style: ThinkingStyle,
    /// Common patterns
    pub common_patterns: Vec<Pattern>,
}

/// Complexity trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityTrend {
    Increasing,
    Decreasing,
    Stable,
    Variable,
}

/// Thinking style classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThinkingStyle {
    Linear,
    Iterative,
    Exploratory,
    Analytical,
    Creative,
    Mixed,
}

/// Pattern identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Pattern type
    pub pattern_type: String,
    /// Pattern description
    pub description: String,
    /// Pattern frequency
    pub frequency: u32,
    /// Pattern confidence
    pub confidence: f64,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average processing time per thought
    pub avg_processing_time_ms: f64,
    /// Total processing time
    pub total_processing_time_ms: u64,
    /// Throughput (thoughts per minute)
    pub throughput: f64,
    /// Response time distribution
    pub response_time_distribution: HashMap<String, u32>,
    /// Performance bottlenecks
    pub bottlenecks: Vec<Bottleneck>,
}

/// Performance bottleneck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    /// Bottleneck type
    pub bottleneck_type: String,
    /// Bottleneck description
    pub description: String,
    /// Impact level
    pub impact_level: ImpactLevel,
    /// Suggested solution
    pub suggested_solution: String,
}

/// Impact level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Thought coherence score
    pub coherence_score: f64,
    /// Logical flow score
    pub logical_flow_score: f64,
    /// Completeness score
    pub completeness_score: f64,
    /// Clarity score
    pub clarity_score: f64,
    /// Overall quality score
    pub overall_quality_score: f64,
    /// Quality issues
    pub quality_issues: Vec<QualityIssue>,
}

/// Quality issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    /// Issue type
    pub issue_type: String,
    /// Issue description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Affected thoughts
    pub affected_thoughts: Vec<u32>,
}

/// Severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Minor,
    Moderate,
    Major,
    Critical,
}

/// Insight about the session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    /// Insight type
    pub insight_type: String,
    /// Insight description
    pub description: String,
    /// Confidence level
    pub confidence: f64,
    /// Supporting data
    pub supporting_data: HashMap<String, serde_json::Value>,
}

/// Recommendation for improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Recommendation type
    pub recommendation_type: String,
    /// Recommendation description
    pub description: String,
    /// Priority
    pub priority: Priority,
    /// Expected impact
    pub expected_impact: String,
    /// Implementation difficulty
    pub implementation_difficulty: Difficulty,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Implementation difficulty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    VeryHard,
}

/// Analytics engine for processing session data
pub struct AnalyticsEngine {
    /// Configuration
    config: AnalyticsConfig,
    /// Analytics data storage
    analytics_data: HashMap<String, SessionAnalytics>,
    /// Metrics aggregator
    metrics_aggregator: MetricsAggregator,
}

/// Metrics aggregator for collecting and processing metrics
#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsAggregator {
    /// Total sessions analyzed
    pub total_sessions: u64,
    /// Average session duration
    pub avg_session_duration: f64,
    /// Average thoughts per session
    pub avg_thoughts_per_session: f64,
    /// Average revisions per session
    pub avg_revisions_per_session: f64,
    /// Average branches per session
    pub avg_branches_per_session: f64,
    /// Performance trends
    pub performance_trends: HashMap<String, Vec<f64>>,
}

impl Default for MetricsAggregator {
    fn default() -> Self {
        Self {
            total_sessions: 0,
            avg_session_duration: 0.0,
            avg_thoughts_per_session: 0.0,
            avg_revisions_per_session: 0.0,
            avg_branches_per_session: 0.0,
            performance_trends: HashMap::new(),
        }
    }
}

impl AnalyticsEngine {
    /// Create a new analytics engine
    pub fn new() -> Self {
        Self {
            config: AnalyticsConfig::default(),
            analytics_data: HashMap::new(),
            metrics_aggregator: MetricsAggregator::default(),
        }
    }

    /// Create a new analytics engine with configuration
    pub fn with_config(config: AnalyticsConfig) -> Self {
        Self {
            config,
            analytics_data: HashMap::new(),
            metrics_aggregator: MetricsAggregator::default(),
        }
    }

    /// Analyze a thinking session
    pub fn analyze_session(
        &mut self,
        session_id: &str,
        session_title: &str,
        thoughts: &[ThoughtData],
        stats: &ThinkingStats,
        progress: &ThinkingProgress,
    ) -> SessionAnalytics {
        let analyzed_at = Utc::now();
        
        // Calculate basic metrics
        let basic_metrics = self.calculate_basic_metrics(thoughts, stats, progress);
        
        // Analyze thinking patterns
        let thinking_patterns = self.analyze_thinking_patterns(thoughts);
        
        // Calculate performance metrics
        let performance_metrics = self.calculate_performance_metrics(stats);
        
        // Calculate quality metrics
        let quality_metrics = self.calculate_quality_metrics(thoughts);
        
        // Generate insights
        let insights = self.generate_insights(thoughts, &basic_metrics, &thinking_patterns);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&basic_metrics, &quality_metrics);
        
        let analytics = SessionAnalytics {
            session_id: session_id.to_string(),
            session_title: session_title.to_string(),
            analyzed_at,
            basic_metrics,
            thinking_patterns,
            performance_metrics,
            quality_metrics,
            insights,
            recommendations,
        };
        
        // Store analytics data
        self.analytics_data.insert(session_id.to_string(), analytics.clone());
        
        // Update aggregator
        self.update_aggregator(&analytics);
        
        analytics
    }

    /// Calculate basic metrics
    fn calculate_basic_metrics(
        &self,
        thoughts: &[ThoughtData],
        stats: &ThinkingStats,
        progress: &ThinkingProgress,
    ) -> BasicMetrics {
        let total_thoughts = thoughts.len() as u32;
        let total_revisions = thoughts.iter().filter(|t| t.is_revision()).count() as u32;
        let total_branches = thoughts.iter().filter(|t| t.is_branch()).count() as u32;
        
        let session_duration = if let (Some(first), Some(last)) = (thoughts.first(), thoughts.last()) {
            if let (Some(first_time), Some(last_time)) = (first.timestamp, last.timestamp) {
                (last_time - first_time).num_seconds() as u64
            } else {
                0
            }
        } else {
            0
        };
        
        let avg_thought_length = if total_thoughts > 0 {
            thoughts.iter().map(|t| t.thought.len()).sum::<usize>() as f64 / total_thoughts as f64
        } else {
            0.0
        };
        
        let completion_rate = if progress.total_thoughts > 0 {
            progress.completed_thoughts as f64 / progress.total_thoughts as f64
        } else {
            0.0
        };
        
        let efficiency_score = self.calculate_efficiency_score(thoughts, stats);
        
        BasicMetrics {
            total_thoughts,
            total_revisions,
            total_branches,
            session_duration,
            avg_thought_length,
            completion_rate,
            efficiency_score,
        }
    }

    /// Calculate efficiency score
    fn calculate_efficiency_score(&self, thoughts: &[ThoughtData], stats: &ThinkingStats) -> f64 {
        if thoughts.is_empty() {
            return 0.0;
        }
        
        let revision_ratio = if stats.total_thoughts > 0 {
            stats.total_revisions as f64 / stats.total_thoughts as f64
        } else {
            0.0
        };
        
        let branch_ratio = if stats.total_thoughts > 0 {
            stats.total_branches as f64 / stats.total_thoughts as f64
        } else {
            0.0
        };
        
        // Efficiency decreases with more revisions and branches
        let base_score = 1.0;
        let revision_penalty = revision_ratio * 0.3;
        let branch_penalty = branch_ratio * 0.2;
        
        (base_score - revision_penalty - branch_penalty).max(0.0)
    }

    /// Analyze thinking patterns
    fn analyze_thinking_patterns(&self, thoughts: &[ThoughtData]) -> ThinkingPatterns {
        let revision_frequency = if thoughts.len() > 1 {
            thoughts.iter().filter(|t| t.is_revision()).count() as f64 / (thoughts.len() - 1) as f64
        } else {
            0.0
        };
        
        let branching_frequency = if thoughts.len() > 1 {
            thoughts.iter().filter(|t| t.is_branch()).count() as f64 / (thoughts.len() - 1) as f64
        } else {
            0.0
        };
        
        let complexity_trend = self.analyze_complexity_trend(thoughts);
        let thinking_style = self.classify_thinking_style(thoughts);
        let common_patterns = self.identify_patterns(thoughts);
        
        ThinkingPatterns {
            revision_frequency,
            branching_frequency,
            complexity_trend,
            thinking_style,
            common_patterns,
        }
    }

    /// Analyze complexity trend
    fn analyze_complexity_trend(&self, thoughts: &[ThoughtData]) -> ComplexityTrend {
        if thoughts.len() < 3 {
            return ComplexityTrend::Stable;
        }
        
        let complexities: Vec<usize> = thoughts.iter().map(|t| t.thought.len()).collect();
        let first_third = complexities.len() / 3;
        let last_third = complexities.len() - first_third;
        
        let avg_first = complexities[..first_third].iter().sum::<usize>() as f64 / first_third as f64;
        let avg_last = complexities[last_third..].iter().sum::<usize>() as f64 / (complexities.len() - last_third) as f64;
        
        let change_ratio = (avg_last - avg_first) / avg_first.max(1.0);
        
        if change_ratio > 0.2 {
            ComplexityTrend::Increasing
        } else if change_ratio < -0.2 {
            ComplexityTrend::Decreasing
        } else if change_ratio.abs() < 0.1 {
            ComplexityTrend::Stable
        } else {
            ComplexityTrend::Variable
        }
    }

    /// Classify thinking style
    fn classify_thinking_style(&self, thoughts: &[ThoughtData]) -> ThinkingStyle {
        let revisions = thoughts.iter().filter(|t| t.is_revision()).count();
        let branches = thoughts.iter().filter(|t| t.is_branch()).count();
        let total = thoughts.len();
        
        if revisions > total / 3 {
            ThinkingStyle::Iterative
        } else if branches > total / 4 {
            ThinkingStyle::Exploratory
        } else if revisions == 0 && branches == 0 {
            ThinkingStyle::Linear
        } else if total > 10 {
            ThinkingStyle::Analytical
        } else {
            ThinkingStyle::Mixed
        }
    }

    /// Identify common patterns
    fn identify_patterns(&self, thoughts: &[ThoughtData]) -> Vec<Pattern> {
        let mut patterns = Vec::new();
        
        // Pattern: Frequent revisions
        let revision_count = thoughts.iter().filter(|t| t.is_revision()).count();
        if revision_count > thoughts.len() / 4 {
            patterns.push(Pattern {
                pattern_type: "frequent_revisions".to_string(),
                description: "High frequency of thought revisions".to_string(),
                frequency: revision_count as u32,
                confidence: 0.8,
            });
        }
        
        // Pattern: Branching exploration
        let branch_count = thoughts.iter().filter(|t| t.is_branch()).count();
        if branch_count > thoughts.len() / 5 {
            patterns.push(Pattern {
                pattern_type: "branching_exploration".to_string(),
                description: "Exploratory thinking with multiple branches".to_string(),
                frequency: branch_count as u32,
                confidence: 0.7,
            });
        }
        
        // Pattern: Linear progression
        if revision_count == 0 && branch_count == 0 && thoughts.len() > 3 {
            patterns.push(Pattern {
                pattern_type: "linear_progression".to_string(),
                description: "Straightforward linear thinking process".to_string(),
                frequency: thoughts.len() as u32,
                confidence: 0.9,
            });
        }
        
        patterns
    }

    /// Calculate performance metrics
    fn calculate_performance_metrics(&self, stats: &ThinkingStats) -> PerformanceMetrics {
        let throughput = if stats.total_processing_time_ms > 0 {
            (stats.total_thoughts as f64 * 60000.0) / stats.total_processing_time_ms as f64
        } else {
            0.0
        };
        
        let mut response_time_distribution = HashMap::new();
        response_time_distribution.insert("fast".to_string(), 0);
        response_time_distribution.insert("medium".to_string(), 0);
        response_time_distribution.insert("slow".to_string(), 0);
        
        let bottlenecks = Vec::new(); // Simplified for now
        
        PerformanceMetrics {
            avg_processing_time_ms: stats.avg_processing_time_ms,
            total_processing_time_ms: stats.total_processing_time_ms,
            throughput,
            response_time_distribution,
            bottlenecks,
        }
    }

    /// Calculate quality metrics
    fn calculate_quality_metrics(&self, thoughts: &[ThoughtData]) -> QualityMetrics {
        let coherence_score = self.calculate_coherence_score(thoughts);
        let logical_flow_score = self.calculate_logical_flow_score(thoughts);
        let completeness_score = self.calculate_completeness_score(thoughts);
        let clarity_score = self.calculate_clarity_score(thoughts);
        
        let overall_quality_score = (coherence_score + logical_flow_score + completeness_score + clarity_score) / 4.0;
        
        let quality_issues = self.identify_quality_issues(thoughts);
        
        QualityMetrics {
            coherence_score,
            logical_flow_score,
            completeness_score,
            clarity_score,
            overall_quality_score,
            quality_issues,
        }
    }

    /// Calculate coherence score
    fn calculate_coherence_score(&self, thoughts: &[ThoughtData]) -> f64 {
        if thoughts.len() < 2 {
            return 1.0;
        }
        
        let mut coherence_score: f32 = 1.0;
        
        for i in 1..thoughts.len() {
            let prev_thought = &thoughts[i - 1];
            let curr_thought = &thoughts[i];
            
            // Check for logical connections
            let has_connection = curr_thought.thought.to_lowercase().contains(&prev_thought.thought.to_lowercase()[..prev_thought.thought.len().min(10)]);
            
            if !has_connection && !curr_thought.is_revision() && !curr_thought.is_branch() {
                coherence_score -= 0.1;
            }
        }
        
        coherence_score.max(0.0) as f64
    }

    /// Calculate logical flow score
    fn calculate_logical_flow_score(&self, thoughts: &[ThoughtData]) -> f64 {
        if thoughts.is_empty() {
            return 0.0;
        }
        
        let mut flow_score: f32 = 1.0;
        let mut consecutive_revisions = 0;
        
        for thought in thoughts {
            if thought.is_revision() {
                consecutive_revisions += 1;
                if consecutive_revisions > 2 {
                    flow_score -= 0.1;
                }
            } else {
                consecutive_revisions = 0;
            }
        }
        
        flow_score.max(0.0) as f64
    }

    /// Calculate completeness score
    fn calculate_completeness_score(&self, thoughts: &[ThoughtData]) -> f64 {
        if thoughts.is_empty() {
            return 0.0;
        }
        
        let avg_length = thoughts.iter().map(|t| t.thought.len()).sum::<usize>() as f64 / thoughts.len() as f64;
        let min_acceptable_length = 20.0;
        
        if avg_length < min_acceptable_length {
            0.5
        } else if avg_length > 100.0 {
            1.0
        } else {
            0.5 + (avg_length - min_acceptable_length) / (100.0 - min_acceptable_length) * 0.5
        }
    }

    /// Calculate clarity score
    fn calculate_clarity_score(&self, thoughts: &[ThoughtData]) -> f64 {
        if thoughts.is_empty() {
            return 0.0;
        }
        
        let mut clarity_score: f32 = 1.0;
        
        for thought in thoughts {
            let words = thought.thought.split_whitespace().count();
            if words < 5 {
                clarity_score -= 0.1;
            }
            if thought.thought.len() > 500 {
                clarity_score -= 0.05;
            }
        }
        
        clarity_score.max(0.0) as f64
    }

    /// Identify quality issues
    fn identify_quality_issues(&self, thoughts: &[ThoughtData]) -> Vec<QualityIssue> {
        let mut issues = Vec::new();
        
        for (i, thought) in thoughts.iter().enumerate() {
            if thought.thought.len() < 10 {
                issues.push(QualityIssue {
                    issue_type: "short_thought".to_string(),
                    description: "Thought is too short".to_string(),
                    severity: Severity::Minor,
                    affected_thoughts: vec![i as u32 + 1],
                });
            }
            
            if thought.thought.len() > 1000 {
                issues.push(QualityIssue {
                    issue_type: "long_thought".to_string(),
                    description: "Thought is too long".to_string(),
                    severity: Severity::Moderate,
                    affected_thoughts: vec![i as u32 + 1],
                });
            }
        }
        
        issues
    }

    /// Generate insights
    fn generate_insights(
        &self,
        thoughts: &[ThoughtData],
        basic_metrics: &BasicMetrics,
        thinking_patterns: &ThinkingPatterns,
    ) -> Vec<Insight> {
        let mut insights = Vec::new();
        
        // Insight: High revision rate
        if thinking_patterns.revision_frequency > 0.3 {
            insights.push(Insight {
                insight_type: "high_revision_rate".to_string(),
                description: "High frequency of thought revisions suggests iterative thinking process".to_string(),
                confidence: 0.8,
                supporting_data: HashMap::new(),
            });
        }
        
        // Insight: Efficient thinking
        if basic_metrics.efficiency_score > 0.8 {
            insights.push(Insight {
                insight_type: "efficient_thinking".to_string(),
                description: "High efficiency score indicates effective problem-solving approach".to_string(),
                confidence: 0.9,
                supporting_data: HashMap::new(),
            });
        }
        
        // Insight: Exploratory thinking
        if thinking_patterns.branching_frequency > 0.2 {
            insights.push(Insight {
                insight_type: "exploratory_thinking".to_string(),
                description: "Multiple branches indicate exploratory thinking approach".to_string(),
                confidence: 0.7,
                supporting_data: HashMap::new(),
            });
        }
        
        insights
    }

    /// Generate recommendations
    fn generate_recommendations(
        &self,
        basic_metrics: &BasicMetrics,
        quality_metrics: &QualityMetrics,
    ) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        // Recommendation: Improve efficiency
        if basic_metrics.efficiency_score < 0.6 {
            recommendations.push(Recommendation {
                recommendation_type: "improve_efficiency".to_string(),
                description: "Consider reducing revisions and branches to improve efficiency".to_string(),
                priority: Priority::High,
                expected_impact: "20% improvement in efficiency".to_string(),
                implementation_difficulty: Difficulty::Medium,
            });
        }
        
        // Recommendation: Improve quality
        if quality_metrics.overall_quality_score < 0.7 {
            recommendations.push(Recommendation {
                recommendation_type: "improve_quality".to_string(),
                description: "Focus on thought clarity and logical flow".to_string(),
                priority: Priority::Medium,
                expected_impact: "15% improvement in quality".to_string(),
                implementation_difficulty: Difficulty::Easy,
            });
        }
        
        recommendations
    }

    /// Update metrics aggregator
    fn update_aggregator(&mut self, analytics: &SessionAnalytics) {
        self.metrics_aggregator.total_sessions += 1;
        
        let total_sessions = self.metrics_aggregator.total_sessions as f64;
        
        // Update averages
        self.metrics_aggregator.avg_session_duration = 
            (self.metrics_aggregator.avg_session_duration * (total_sessions - 1.0) + analytics.basic_metrics.session_duration as f64) / total_sessions;
        
        self.metrics_aggregator.avg_thoughts_per_session = 
            (self.metrics_aggregator.avg_thoughts_per_session * (total_sessions - 1.0) + analytics.basic_metrics.total_thoughts as f64) / total_sessions;
        
        self.metrics_aggregator.avg_revisions_per_session = 
            (self.metrics_aggregator.avg_revisions_per_session * (total_sessions - 1.0) + analytics.basic_metrics.total_revisions as f64) / total_sessions;
        
        self.metrics_aggregator.avg_branches_per_session = 
            (self.metrics_aggregator.avg_branches_per_session * (total_sessions - 1.0) + analytics.basic_metrics.total_branches as f64) / total_sessions;
    }

    /// Get analytics for a session
    pub fn get_session_analytics(&self, session_id: &str) -> Option<&SessionAnalytics> {
        self.analytics_data.get(session_id)
    }

    /// Get aggregated metrics
    pub fn get_aggregated_metrics(&self) -> &MetricsAggregator {
        &self.metrics_aggregator
    }

    /// Export analytics data
    pub fn export_analytics(&self) -> serde_json::Value {
        serde_json::json!({
            "analytics_data": self.analytics_data,
            "aggregated_metrics": self.metrics_aggregator,
            "exported_at": Utc::now()
        })
    }
}

impl Default for AnalyticsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thinking::ThoughtData;

    #[test]
    fn test_analytics_engine_creation() {
        let engine = AnalyticsEngine::new();
        assert!(!engine.config.enabled);
        assert_eq!(engine.config.endpoint, "http://localhost:9090");
    }

    #[test]
    fn test_basic_metrics_calculation() {
        let engine = AnalyticsEngine::new();
        let thoughts = vec![
            ThoughtData::new("First thought".to_string(), 1, 3),
            ThoughtData::new("Second thought".to_string(), 2, 3),
            ThoughtData::new("Third thought".to_string(), 3, 3),
        ];
        
        let stats = ThinkingStats::default();
        let progress = ThinkingProgress::new(3, 3);
        
        let metrics = engine.calculate_basic_metrics(&thoughts, &stats, &progress);
        
        assert_eq!(metrics.total_thoughts, 3);
        assert_eq!(metrics.total_revisions, 0);
        assert_eq!(metrics.total_branches, 0);
        assert!(metrics.avg_thought_length > 0.0);
    }

    #[test]
    fn test_thinking_patterns_analysis() {
        let engine = AnalyticsEngine::new();
        let thoughts = vec![
            ThoughtData::new("First thought".to_string(), 1, 3),
            ThoughtData::revision("Revised thought".to_string(), 2, 1),
            ThoughtData::new("Third thought".to_string(), 3, 3),
        ];
        
        let patterns = engine.analyze_thinking_patterns(&thoughts);
        
        assert!(patterns.revision_frequency > 0.0);
        assert_eq!(patterns.revision_frequency, 0.5);
    }

    #[test]
    fn test_quality_metrics_calculation() {
        let engine = AnalyticsEngine::new();
        let thoughts = vec![
            ThoughtData::new("This is a well-formed thought with sufficient detail".to_string(), 1, 3),
            ThoughtData::new("Another comprehensive thought".to_string(), 2, 3),
        ];
        
        let metrics = engine.calculate_quality_metrics(&thoughts);
        
        assert!(metrics.coherence_score > 0.0);
        assert!(metrics.logical_flow_score > 0.0);
        assert!(metrics.completeness_score > 0.0);
        assert!(metrics.clarity_score > 0.0);
        assert!(metrics.overall_quality_score > 0.0);
    }
} 