# Advanced Filtering Guide - mem0-rs

## Overview

mem0-rs provides a comprehensive filtering and query DSL for advanced memory queries, aggregations, and time-based filtering.

## Filter Operators

### Comparison Operators

```rust
use mem0_rs::filtering::{FilterCondition, FilterValue, FilterOperator};

// Equal
FilterCondition::eq("status".to_string(), FilterValue::String("active".to_string()))

// Not equal
FilterCondition {
    field: "status".to_string(),
    operator: FilterOperator::Ne,
    value: FilterValue::String("inactive".to_string()),
}

// Greater than
FilterCondition {
    field: "score".to_string(),
    operator: FilterOperator::Gt,
    value: FilterValue::Number(80.0),
}

// Less than or equal
FilterCondition {
    field: "age".to_string(),
    operator: FilterOperator::Lte,
    value: FilterValue::Number(30.0),
}
```

### String Operators

```rust
// Contains substring
FilterCondition::contains("content".to_string(), "coffee".to_string())

// In list
FilterCondition {
    field: "category".to_string(),
    operator: FilterOperator::In,
    value: FilterValue::List(vec![
        FilterValue::String("preference".to_string()),
        FilterValue::String("fact".to_string()),
    ]),
}

// Not in list
FilterCondition {
    field: "status".to_string(),
    operator: FilterOperator::NotIn,
    value: FilterValue::List(vec![
        FilterValue::String("deleted".to_string()),
        FilterValue::String("archived".to_string()),
    ]),
}
```

### Range Operators

```rust
// Between range
FilterCondition::between("score".to_string(), 70.0, 100.0)

// Exists
FilterCondition::exists("metadata".to_string())
```

## Complex Queries

### AND Queries

```rust
use mem0_rs::filtering::{FilterQuery, LogicalOperator};

let query = FilterQuery::new(LogicalOperator::And)
    .add_condition(FilterCondition::eq(
        "status".to_string(),
        FilterValue::String("active".to_string()),
    ))
    .add_condition(FilterCondition::contains(
        "content".to_string(),
        "coffee".to_string(),
    ));

// Matches: status == "active" AND content contains "coffee"
```

### OR Queries

```rust
let query = FilterQuery::new(LogicalOperator::Or)
    .add_condition(FilterCondition::eq(
        "category".to_string(),
        FilterValue::String("preference".to_string()),
    ))
    .add_condition(FilterCondition::eq(
        "category".to_string(),
        FilterValue::String("fact".to_string()),
    ));

// Matches: category == "preference" OR category == "fact"
```

### Nested Queries

```rust
let inner_query = FilterQuery::new(LogicalOperator::Or)
    .add_condition(FilterCondition::eq(
        "type".to_string(),
        FilterValue::String("preference".to_string()),
    ));

let outer_query = FilterQuery::new(LogicalOperator::And)
    .add_condition(FilterCondition::eq(
        "status".to_string(),
        FilterValue::String("active".to_string()),
    ))
    .add_nested(inner_query);

// Matches: status == "active" AND (type == "preference")
```

## Aggregations

### Count

```rust
use mem0_rs::filtering::{AggregationQuery, AggregationFunction};

let agg = AggregationQuery::new(
    AggregationFunction::Count,
    "id".to_string(),
);

// Count total memories
```

### Sum

```rust
let agg = AggregationQuery::new(
    AggregationFunction::Sum,
    "score".to_string(),
);

// Sum all scores
```

### Average

```rust
let agg = AggregationQuery::new(
    AggregationFunction::Avg,
    "score".to_string(),
);

// Average score
```

### Min/Max

```rust
let min_agg = AggregationQuery::new(
    AggregationFunction::Min,
    "score".to_string(),
);

let max_agg = AggregationQuery::new(
    AggregationFunction::Max,
    "score".to_string(),
);
```

### Distinct

```rust
let agg = AggregationQuery::new(
    AggregationFunction::Distinct,
    "category".to_string(),
);

// Count distinct categories
```

### Group By

```rust
let agg = AggregationQuery::new(
    AggregationFunction::Count,
    "id".to_string(),
).group_by("category".to_string());

// Count memories by category
```

## Time-Based Filtering

### Specific Time Range

```rust
use mem0_rs::filtering::TimeFilter;
use chrono::Utc;

let start = Utc::now() - chrono::Duration::days(7);
let end = Utc::now();

let filter = TimeFilter::new(
    "created_at".to_string(),
    start,
    end,
);

// Memories created in last 7 days
```

### Today

```rust
let filter = TimeFilter::today("created_at".to_string());

// Memories created today
```

### Last N Days

```rust
let filter = TimeFilter::last_n_days("created_at".to_string(), 30);

// Memories created in last 30 days
```

## Query Builder

### Basic Query

```rust
use mem0_rs::filtering::QueryBuilder;

let query = QueryBuilder::new()
    .limit(10)
    .offset(0)
    .build();

// Get first 10 results
```

### Complex Query

```rust
let query = QueryBuilder::new()
    .filter(FilterQuery::new(LogicalOperator::And)
        .add_condition(FilterCondition::eq(
            "status".to_string(),
            FilterValue::String("active".to_string()),
        )))
    .aggregate(AggregationQuery::new(
        AggregationFunction::Count,
        "id".to_string(),
    ))
    .time_filter(TimeFilter::last_n_days("created_at".to_string(), 7))
    .limit(100)
    .offset(0)
    .build();
```

### With Aggregation

```rust
let query = QueryBuilder::new()
    .aggregate(AggregationQuery::new(
        AggregationFunction::Avg,
        "score".to_string(),
    ).group_by("category".to_string()))
    .build();

// Average score by category
```

## Filter Values

### String Values

```rust
FilterValue::String("active".to_string())
```

### Numeric Values

```rust
FilterValue::Number(42.0)
FilterValue::Number(3.14)
```

### Boolean Values

```rust
FilterValue::Bool(true)
FilterValue::Bool(false)
```

### Date Values

```rust
use chrono::Utc;

FilterValue::Date(Utc::now())
```

### List Values

```rust
FilterValue::List(vec![
    FilterValue::String("a".to_string()),
    FilterValue::String("b".to_string()),
    FilterValue::String("c".to_string()),
])
```

## Query Examples

### Example 1: Find Active Preferences

```rust
let query = FilterQuery::new(LogicalOperator::And)
    .add_condition(FilterCondition::eq(
        "status".to_string(),
        FilterValue::String("active".to_string()),
    ))
    .add_condition(FilterCondition::eq(
        "memory_type".to_string(),
        FilterValue::String("preference".to_string()),
    ));

// Find all active preferences
```

### Example 2: Search with Time Filter

```rust
let query = QueryBuilder::new()
    .filter(FilterQuery::new(LogicalOperator::And)
        .add_condition(FilterCondition::contains(
            "content".to_string(),
            "coffee".to_string(),
        )))
    .time_filter(TimeFilter::last_n_days("created_at".to_string(), 7))
    .limit(10)
    .build();

// Find memories about coffee from last 7 days
```

### Example 3: Aggregation with Filter

```rust
let query = QueryBuilder::new()
    .filter(FilterQuery::new(LogicalOperator::And)
        .add_condition(FilterCondition::eq(
            "status".to_string(),
            FilterValue::String("active".to_string()),
        )))
    .aggregate(AggregationQuery::new(
        AggregationFunction::Count,
        "id".to_string(),
    ).group_by("memory_type".to_string()))
    .build();

// Count active memories by type
```

### Example 4: Complex Nested Query

```rust
let type_filter = FilterQuery::new(LogicalOperator::Or)
    .add_condition(FilterCondition::eq(
        "memory_type".to_string(),
        FilterValue::String("preference".to_string()),
    ))
    .add_condition(FilterCondition::eq(
        "memory_type".to_string(),
        FilterValue::String("fact".to_string()),
    ));

let main_query = FilterQuery::new(LogicalOperator::And)
    .add_condition(FilterCondition::eq(
        "status".to_string(),
        FilterValue::String("active".to_string()),
    ))
    .add_nested(type_filter);

// Find active preferences or facts
```

## Performance Tips

### 1. Use Specific Filters
```rust
// Good: Specific filter
FilterCondition::eq("status".to_string(), FilterValue::String("active".to_string()))

// Avoid: Too broad
FilterCondition::exists("id".to_string())
```

### 2. Limit Results
```rust
QueryBuilder::new()
    .limit(100)
    .offset(0)
    .build()
```

### 3. Use Time Filters
```rust
// Reduces search space
TimeFilter::last_n_days("created_at".to_string(), 30)
```

### 4. Aggregate Efficiently
```rust
// Group before counting
AggregationQuery::new(AggregationFunction::Count, "id".to_string())
    .group_by("category".to_string())
```

## Best Practices

1. **Use Type-Safe Filters**
   - Use FilterCondition helpers
   - Avoid string concatenation
   - Validate filter values

2. **Optimize Queries**
   - Use specific filters
   - Add time constraints
   - Limit result sets
   - Use aggregations

3. **Handle Errors**
   - Check filter validity
   - Handle empty results
   - Validate aggregations

4. **Test Queries**
   - Test with sample data
   - Verify filter logic
   - Check aggregation results

## Troubleshooting

### Empty Results
- Check filter conditions
- Verify field names
- Check data types
- Review time range

### Performance Issues
- Add time filters
- Limit result set
- Use aggregations
- Index frequently filtered fields

### Incorrect Aggregations
- Verify group by field
- Check aggregation function
- Review filter conditions
- Validate data types

## See Also

- [README.md](README.md) - Main documentation
- [ADVANCED_FEATURES.md](ADVANCED_FEATURES.md) - Advanced features
- [STORAGE_BACKENDS.md](STORAGE_BACKENDS.md) - Storage backends
