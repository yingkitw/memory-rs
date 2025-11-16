//! Advanced filtering and query DSL

use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Filter operator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterOperator {
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Greater than
    Gt,
    /// Greater than or equal
    Gte,
    /// Less than
    Lt,
    /// Less than or equal
    Lte,
    /// Contains substring
    Contains,
    /// In list
    In,
    /// Not in list
    NotIn,
    /// Exists
    Exists,
    /// Between range
    Between,
}

impl FilterOperator {
    /// Get operator symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Eq => "==",
            Self::Ne => "!=",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::Lt => "<",
            Self::Lte => "<=",
            Self::Contains => "contains",
            Self::In => "in",
            Self::NotIn => "not_in",
            Self::Exists => "exists",
            Self::Between => "between",
        }
    }
}

/// Filter value
#[derive(Debug, Clone)]
pub enum FilterValue {
    /// String value
    String(String),
    /// Number value
    Number(f64),
    /// Boolean value
    Bool(bool),
    /// List of values
    List(Vec<FilterValue>),
    /// Date value
    Date(DateTime<Utc>),
}

/// Single filter condition
#[derive(Debug, Clone)]
pub struct FilterCondition {
    /// Field name
    pub field: String,
    /// Operator
    pub operator: FilterOperator,
    /// Value
    pub value: FilterValue,
}

impl FilterCondition {
    /// Create an equality filter
    pub fn eq(field: String, value: FilterValue) -> Self {
        Self {
            field,
            operator: FilterOperator::Eq,
            value,
        }
    }

    /// Create a contains filter
    pub fn contains(field: String, value: String) -> Self {
        Self {
            field,
            operator: FilterOperator::Contains,
            value: FilterValue::String(value),
        }
    }

    /// Create a range filter
    pub fn between(field: String, min: f64, max: f64) -> Self {
        Self {
            field,
            operator: FilterOperator::Between,
            value: FilterValue::List(vec![
                FilterValue::Number(min),
                FilterValue::Number(max),
            ]),
        }
    }

    /// Create an exists filter
    pub fn exists(field: String) -> Self {
        Self {
            field,
            operator: FilterOperator::Exists,
            value: FilterValue::Bool(true),
        }
    }
}

/// Logical operator for combining filters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    /// AND - all conditions must be true
    And,
    /// OR - at least one condition must be true
    Or,
    /// NOT - condition must be false
    Not,
}

/// Complex filter query
#[derive(Debug, Clone)]
pub struct FilterQuery {
    /// Conditions
    pub conditions: Vec<FilterCondition>,
    /// Logical operator
    pub logical_op: LogicalOperator,
    /// Nested queries
    pub nested: Vec<FilterQuery>,
}

impl FilterQuery {
    /// Create a new filter query
    pub fn new(logical_op: LogicalOperator) -> Self {
        Self {
            conditions: Vec::new(),
            logical_op,
            nested: Vec::new(),
        }
    }

    /// Add a condition
    pub fn add_condition(mut self, condition: FilterCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Add a nested query
    pub fn add_nested(mut self, query: FilterQuery) -> Self {
        self.nested.push(query);
        self
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();

        for condition in &self.conditions {
            parts.push(format!(
                "{} {} {}",
                condition.field,
                condition.operator.symbol(),
                self.format_value(&condition.value)
            ));
        }

        for nested in &self.nested {
            parts.push(format!("({})", nested.to_string()));
        }

        let op_str = match self.logical_op {
            LogicalOperator::And => " AND ",
            LogicalOperator::Or => " OR ",
            LogicalOperator::Not => " NOT ",
        };

        parts.join(op_str)
    }

    /// Format filter value
    fn format_value(&self, value: &FilterValue) -> String {
        match value {
            FilterValue::String(s) => format!("\"{}\"", s),
            FilterValue::Number(n) => n.to_string(),
            FilterValue::Bool(b) => b.to_string(),
            FilterValue::List(items) => {
                let formatted: Vec<String> = items.iter().map(|v| self.format_value(v)).collect();
                format!("[{}]", formatted.join(", "))
            }
            FilterValue::Date(d) => d.to_rfc3339(),
        }
    }
}

/// Aggregation function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationFunction {
    /// Count items
    Count,
    /// Sum values
    Sum,
    /// Average values
    Avg,
    /// Minimum value
    Min,
    /// Maximum value
    Max,
    /// Distinct count
    Distinct,
}

impl AggregationFunction {
    /// Get function name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::Sum => "sum",
            Self::Avg => "avg",
            Self::Min => "min",
            Self::Max => "max",
            Self::Distinct => "distinct",
        }
    }
}

/// Aggregation query
#[derive(Debug, Clone)]
pub struct AggregationQuery {
    /// Function to apply
    pub function: AggregationFunction,
    /// Field to aggregate
    pub field: String,
    /// Group by field
    pub group_by: Option<String>,
    /// Filter to apply before aggregation
    pub filter: Option<FilterQuery>,
}

impl AggregationQuery {
    /// Create a new aggregation query
    pub fn new(function: AggregationFunction, field: String) -> Self {
        Self {
            function,
            field,
            group_by: None,
            filter: None,
        }
    }

    /// Add group by
    pub fn group_by(mut self, field: String) -> Self {
        self.group_by = Some(field);
        self
    }

    /// Add filter
    pub fn with_filter(mut self, filter: FilterQuery) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Time-based filter
#[derive(Debug, Clone)]
pub struct TimeFilter {
    /// Start time
    pub start: DateTime<Utc>,
    /// End time
    pub end: DateTime<Utc>,
    /// Field to filter on
    pub field: String,
}

impl TimeFilter {
    /// Create a new time filter
    pub fn new(field: String, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end, field }
    }

    /// Create a filter for today
    pub fn today(field: String) -> Self {
        let now = Utc::now();
        let start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end = now.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();

        Self { start, end, field }
    }

    /// Create a filter for last N days
    pub fn last_n_days(field: String, days: i64) -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::days(days);

        Self { start, end, field }
    }
}

/// Query builder for complex queries
pub struct QueryBuilder {
    filters: Vec<FilterQuery>,
    aggregations: Vec<AggregationQuery>,
    time_filters: Vec<TimeFilter>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            aggregations: Vec::new(),
            time_filters: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// Add a filter
    pub fn filter(mut self, filter: FilterQuery) -> Self {
        self.filters.push(filter);
        self
    }

    /// Add an aggregation
    pub fn aggregate(mut self, agg: AggregationQuery) -> Self {
        self.aggregations.push(agg);
        self
    }

    /// Add a time filter
    pub fn time_filter(mut self, filter: TimeFilter) -> Self {
        self.time_filters.push(filter);
        self
    }

    /// Set limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Build the query
    pub fn build(self) -> Query {
        Query {
            filters: self.filters,
            aggregations: self.aggregations,
            time_filters: self.time_filters,
            limit: self.limit,
            offset: self.offset,
        }
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete query
#[derive(Debug, Clone)]
pub struct Query {
    /// Filters to apply
    pub filters: Vec<FilterQuery>,
    /// Aggregations to apply
    pub aggregations: Vec<AggregationQuery>,
    /// Time filters to apply
    pub time_filters: Vec<TimeFilter>,
    /// Result limit
    pub limit: Option<usize>,
    /// Result offset
    pub offset: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_condition_eq() {
        let condition = FilterCondition::eq(
            "status".to_string(),
            FilterValue::String("active".to_string()),
        );

        assert_eq!(condition.field, "status");
        assert_eq!(condition.operator, FilterOperator::Eq);
    }

    #[test]
    fn test_filter_condition_contains() {
        let condition = FilterCondition::contains(
            "content".to_string(),
            "coffee".to_string(),
        );

        assert_eq!(condition.operator, FilterOperator::Contains);
    }

    #[test]
    fn test_filter_query() {
        let query = FilterQuery::new(LogicalOperator::And)
            .add_condition(FilterCondition::eq(
                "status".to_string(),
                FilterValue::String("active".to_string()),
            ));

        assert_eq!(query.conditions.len(), 1);
    }

    #[test]
    fn test_aggregation_query() {
        let agg = AggregationQuery::new(
            AggregationFunction::Count,
            "id".to_string(),
        ).group_by("category".to_string());

        assert_eq!(agg.function, AggregationFunction::Count);
        assert_eq!(agg.group_by, Some("category".to_string()));
    }

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new()
            .limit(10)
            .offset(5)
            .build();

        assert_eq!(query.limit, Some(10));
        assert_eq!(query.offset, Some(5));
    }

    #[test]
    fn test_time_filter() {
        let now = Utc::now();
        let filter = TimeFilter::new("created_at".to_string(), now, now);

        assert_eq!(filter.field, "created_at");
    }
}
