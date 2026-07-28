//! Data Health namespace wire types.

use serde::{Deserialize, Serialize};

pub type CheckRid = String;
pub type CheckReportRid = String;
pub type CheckGroupRid = String;
pub type CheckIntent = String;
pub type ColumnName = String;
pub type CheckReportLimit = u32;
pub type PercentageValue = f64;
pub type IgnoreEmptyTransactions = bool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeverityLevel {
    Moderate,
    Critical,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckResultStatus {
    Passed,
    Failed,
    Warning,
    Error,
    NotApplicable,
    NotComputable,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedianDeviationBoundsType {
    LowerBound,
    UpperBound,
    TwoTailed,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SchemaComparisonType {
    ExactMatchOrderedColumns,
    ExactMatchUnorderedColumns,
    ColumnAdditionsAllowed,
    ColumnAdditionsAllowedStrict,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrendType {
    NonIncreasing,
    NonDecreasing,
    StrictlyIncreasing,
    StrictlyDecreasing,
    Constant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetSubject {
    pub dataset_rid: String,
    pub branch_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericBounds {
    pub lower_bound: Option<f64>,
    pub upper_bound: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericBoundsConfig {
    pub numeric_bounds: NumericBounds,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateBounds {
    pub lower_bound: Option<String>,
    pub upper_bound: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateBoundsConfig {
    pub date_bounds: DateBounds,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBounds {
    pub lower_bound_in_seconds: Option<i64>,
    pub upper_bound_in_seconds: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBoundsConfig {
    pub time_bounds: TimeBounds,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedianDeviation {
    pub bounds_type: Option<MedianDeviationBoundsType>,
    pub data_points: i32,
    pub deviation_threshold: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedianDeviationConfig {
    pub median_deviation: MedianDeviation,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PercentageBounds {
    pub lower_bound_percentage: Option<PercentageValue>,
    pub upper_bound_percentage: Option<PercentageValue>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PercentageBoundsConfig {
    pub percentage_bounds: PercentageBounds,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PercentageCheckConfig {
    pub column_name: ColumnName,
    pub percentage_bounds: Option<PercentageBoundsConfig>,
    pub median_deviation: Option<MedianDeviationConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacePercentageCheckConfig {
    pub median_deviation: Option<MedianDeviationConfig>,
    pub percentage_bounds: Option<PercentageBoundsConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeCheckConfig {
    pub time_bounds: Option<TimeBoundsConfig>,
    pub median_deviation: Option<MedianDeviationConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTimeCheckConfig {
    pub time_bounds: Option<TimeBoundsConfig>,
    pub median_deviation: Option<MedianDeviationConfig>,
    pub ignore_empty_transactions: Option<IgnoreEmptyTransactions>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendConfig {
    pub trend_type: Option<TrendType>,
    pub difference_bounds: Option<NumericBounds>,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericColumnCheckConfig {
    pub column_name: ColumnName,
    pub numeric_bounds: Option<NumericBoundsConfig>,
    pub trend: Option<TrendConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceNumericColumnCheckConfig {
    pub numeric_bounds: Option<NumericBoundsConfig>,
    pub trend: Option<TrendConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCheckConfig {
    pub severity: SeverityLevel,
    pub escalation_config: Option<EscalationConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EscalationConfig {
    pub failures_to_critical: i32,
    pub time_interval_in_seconds: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnCountConfig {
    pub expected_value: i64,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnTypeConfig {
    pub column_name: ColumnName,
    pub expected_type: Option<String>,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceColumnTypeConfig {
    pub severity: SeverityLevel,
    pub expected_type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryKeyConfig {
    pub column_names: Vec<ColumnName>,
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacePrimaryKeyConfig {
    pub severity: SeverityLevel,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnInfo {
    pub name: ColumnName,
    pub column_type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfo {
    pub columns: Vec<ColumnInfo>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaComparisonConfig {
    pub expected_schema: SchemaInfo,
    pub schema_comparison_type: SchemaComparisonType,
    pub severity: SeverityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ColumnValue {
    Date { value: String },
    Boolean { value: bool },
    String { value: String },
    Numeric { value: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CheckConfig {
    NumericColumnRange {
        subject: DatasetSubject,
        column_name: ColumnName,
        numeric_bounds_config: NumericBoundsConfig,
    },
    JobStatus {
        subject: DatasetSubject,
        status_check_config: StatusCheckConfig,
    },
    NumericColumnMean {
        subject: DatasetSubject,
        numeric_column_check_config: NumericColumnCheckConfig,
    },
    DateColumnRange {
        subject: DatasetSubject,
        column_name: ColumnName,
        date_bounds_config: DateBoundsConfig,
    },
    JobDuration {
        subject: DatasetSubject,
        time_check_config: TimeCheckConfig,
    },
    ApproximateUniquePercentage {
        subject: DatasetSubject,
        percentage_check_config: PercentageCheckConfig,
    },
    BuildStatus {
        subject: DatasetSubject,
        status_check_config: StatusCheckConfig,
    },
    ColumnType {
        subject: DatasetSubject,
        column_type_config: ColumnTypeConfig,
    },
    AllowedColumnValues {
        subject: DatasetSubject,
        column_name: ColumnName,
        allowed_values: Vec<ColumnValue>,
        allow_null: Option<bool>,
        severity: SeverityLevel,
    },
    TimeSinceLastUpdated {
        subject: DatasetSubject,
        time_check_config: TransactionTimeCheckConfig,
    },
    NullPercentage {
        subject: DatasetSubject,
        percentage_check_config: PercentageCheckConfig,
    },
    TotalColumnCount {
        subject: DatasetSubject,
        column_count_config: ColumnCountConfig,
    },
    NumericColumnMedian {
        subject: DatasetSubject,
        numeric_column_check_config: NumericColumnCheckConfig,
    },
    BuildDuration {
        subject: DatasetSubject,
        time_check_config: TimeCheckConfig,
    },
    SchemaComparison {
        subject: DatasetSubject,
        schema_comparison_config: SchemaComparisonConfig,
    },
    PrimaryKey {
        subject: DatasetSubject,
        primary_key_config: PrimaryKeyConfig,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ReplaceCheckConfig {
    NumericColumnRange {
        numeric_bounds_config: NumericBoundsConfig,
    },
    JobStatus {
        status_check_config: StatusCheckConfig,
    },
    NumericColumnMean {
        numeric_column_check_config: ReplaceNumericColumnCheckConfig,
    },
    DateColumnRange {
        date_bounds_config: DateBoundsConfig,
    },
    JobDuration {
        time_check_config: TimeCheckConfig,
    },
    ApproximateUniquePercentage {
        percentage_check_config: ReplacePercentageCheckConfig,
    },
    BuildStatus {
        status_check_config: StatusCheckConfig,
    },
    ColumnType {
        column_type_config: ReplaceColumnTypeConfig,
    },
    AllowedColumnValues {
        allowed_values: Vec<ColumnValue>,
        severity: SeverityLevel,
        allow_null: Option<bool>,
    },
    TimeSinceLastUpdated {
        time_check_config: TransactionTimeCheckConfig,
    },
    NullPercentage {
        percentage_check_config: ReplacePercentageCheckConfig,
    },
    TotalColumnCount {
        column_count_config: ColumnCountConfig,
    },
    NumericColumnMedian {
        numeric_column_check_config: ReplaceNumericColumnCheckConfig,
    },
    BuildDuration {
        time_check_config: TimeCheckConfig,
    },
    SchemaComparison {
        schema_comparison_config: SchemaComparisonConfig,
    },
    PrimaryKey {
        primary_key_config: ReplacePrimaryKeyConfig,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckRequest {
    pub config: CheckConfig,
    pub intent: Option<CheckIntent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceCheckRequest {
    pub config: ReplaceCheckConfig,
    pub intent: Option<CheckIntent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Check {
    pub rid: CheckRid,
    pub groups: Vec<CheckGroupRid>,
    pub config: CheckConfig,
    pub intent: Option<CheckIntent>,
    pub created_by: Option<String>,
    pub updated_time: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub status: CheckResultStatus,
    pub message: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckReport {
    pub rid: CheckReportRid,
    pub check: Check,
    pub result: CheckResult,
    pub created_time: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLatestCheckReportsResponse {
    pub data: Vec<CheckReport>,
}
