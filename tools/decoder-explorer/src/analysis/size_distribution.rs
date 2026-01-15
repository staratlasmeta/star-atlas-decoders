use super::AnalysisContext;
use anyhow::Result;
use std::collections::HashMap;

const SIZE_BUCKETS: &[usize] = &[
    100, 500, 1_000, 5_000, 10_000, 50_000, 100_000, 500_000, 1_000_000,
];

pub fn analyze(ctx: &AnalysisContext) -> Result<()> {
    tracing::info!("=== Account Size Distribution ===");

    let total_accounts = ctx.total_accounts();
    let total_size = ctx.total_data_size();

    tracing::info!("Total accounts: {}", total_accounts);
    tracing::info!(
        "Total data size: {} bytes ({:.2} MB)",
        total_size,
        total_size as f64 / 1_048_576.0
    );

    if total_accounts == 0 {
        tracing::warn!("No accounts found");
        return Ok(());
    }

    let avg_size = total_size / total_accounts;
    tracing::info!("Average account size: {} bytes", avg_size);

    let mut histogram: HashMap<String, usize> = HashMap::new();

    for (_, account) in ctx.accounts {
        let size = account.data.len();
        let bucket = get_size_bucket(size);
        *histogram.entry(bucket).or_insert(0) += 1;
    }

    tracing::info!("\nSize Distribution:");
    let mut buckets: Vec<_> = histogram.iter().collect();
    buckets.sort_by_key(|(bucket, _)| bucket_sort_key(bucket));

    for (bucket, count) in buckets {
        let percentage = (*count as f64 / total_accounts as f64) * 100.0;
        tracing::info!("  {:20} {:8} ({:5.2}%)", bucket, count, percentage);
    }

    let mut min_size = usize::MAX;
    let mut max_size = 0;

    for (_, account) in ctx.accounts {
        let size = account.data.len();
        min_size = min_size.min(size);
        max_size = max_size.max(size);
    }

    tracing::info!("\nSize Range:");
    tracing::info!("  Minimum: {} bytes", min_size);
    tracing::info!(
        "  Maximum: {} bytes ({:.2} KB)",
        max_size,
        max_size as f64 / 1024.0
    );

    Ok(())
}

fn get_size_bucket(size: usize) -> String {
    for &bucket in SIZE_BUCKETS {
        if size < bucket {
            return format!("< {} bytes", bucket);
        }
    }
    format!(">= {} bytes", SIZE_BUCKETS.last().unwrap())
}

fn bucket_sort_key(bucket: &str) -> usize {
    // Extract number from bucket string for sorting
    if let Some(num_str) = bucket.split_whitespace().nth(1) {
        num_str.replace(',', "").parse().unwrap_or(0)
    } else {
        usize::MAX
    }
}
