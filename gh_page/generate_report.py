import os
import json
import sys
from datetime import datetime

def format_time(ns):
    if ns is None:
        return "N/A"
    if ns < 1000:
        return f"{ns:.2f} ns"
    elif ns < 1000000:
        return f"{ns / 1000:.2f} µs"
    elif ns < 1000000000:
        return f"{ns / 1000000:.2f} ms"
    else:
        return f"{ns / 1000000000:.2f} s"

def main():
    # Paths are relative to the project root
    workspace_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    criterion_dir = os.path.join(workspace_dir, "target", "criterion")
    output_dir = os.path.join(workspace_dir, "gh_page")
    output_path = os.path.join(output_dir, "benchmark_data.json")

    if not os.path.exists(criterion_dir):
        print(f"Error: Criterion directory '{criterion_dir}' not found.")
        sys.exit(1)

    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    print("Scanning Criterion benchmark directory...")
    raw_data = {}

    for root, dirs, files in os.walk(criterion_dir):
        if "new" in dirs:
            new_dir = os.path.join(root, "new")
            bench_path = os.path.join(new_dir, "benchmark.json")
            est_path = os.path.join(new_dir, "estimates.json")
            
            if os.path.exists(bench_path) and os.path.exists(est_path):
                try:
                    with open(bench_path, "r") as f:
                        bench_data = json.load(f)
                    with open(est_path, "r") as f:
                        est_data = json.load(f)
                    
                    group_id = bench_data.get("group_id")
                    function_id = bench_data.get("function_id") or "default"
                    value_str = bench_data.get("value_str")
                    
                    try:
                        val = float(value_str) if '.' in value_str else int(value_str)
                    except (ValueError, TypeError):
                        val = value_str
                    
                    mean = est_data.get("mean", {})
                    median = est_data.get("median", {})
                    std_dev = est_data.get("std_dev", {})
                    
                    point_data = {
                        "value_str": value_str,
                        "value_num": val,
                        "mean_ns": mean.get("point_estimate"),
                        "mean_lower_ns": mean.get("confidence_interval", {}).get("lower_bound"),
                        "mean_upper_ns": mean.get("confidence_interval", {}).get("upper_bound"),
                        "median_ns": median.get("point_estimate"),
                        "std_dev_ns": std_dev.get("point_estimate"),
                        "std_dev_pct": (std_dev.get("point_estimate") / mean.get("point_estimate") * 100) if mean.get("point_estimate") else 0
                    }
                    
                    if group_id not in raw_data:
                        raw_data[group_id] = {}
                    if function_id not in raw_data[group_id]:
                        raw_data[group_id][function_id] = []
                    
                    raw_data[group_id][function_id].append(point_data)
                except Exception as e:
                    print(f"Warning: Failed to parse benchmark at {root}: {e}")

    # Sort the points for each function by their numeric value
    for group_id in raw_data:
        for function_id in raw_data[group_id]:
            raw_data[group_id][function_id].sort(
                key=lambda x: x["value_num"] if isinstance(x["value_num"], (int, float)) else 0
            )

    # Calculate some general stats
    total_benchmarks = 0
    fastest_time = float('inf')
    fastest_name = ""
    slowest_time = 0
    slowest_name = ""

    for g_id, fns in raw_data.items():
        for f_id, pts in fns.items():
            total_benchmarks += len(pts)
            for pt in pts:
                m_ns = pt["mean_ns"]
                if m_ns is not None:
                    if m_ns < fastest_time:
                        fastest_time = m_ns
                        fastest_name = f"{g_id}/{f_id} ({pt['value_str']})"
                    if m_ns > slowest_time:
                        slowest_time = m_ns
                        slowest_name = f"{g_id}/{f_id} ({pt['value_str']})"

    summary_stats = {
        "generated_at": datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
        "total_benchmarks": total_benchmarks,
        "fastest_time": fastest_time if fastest_time != float('inf') else 0,
        "fastest_time_str": format_time(fastest_time) if fastest_time != float('inf') else "N/A",
        "fastest_name": fastest_name,
        "slowest_time": slowest_time,
        "slowest_time_str": format_time(slowest_time) if slowest_time > 0 else "N/A",
        "slowest_name": slowest_name,
        "group_count": len(raw_data)
    }

    # Output structured JSON
    payload = {
        "benchmarks": raw_data,
        "stats": summary_stats
    }

    with open(output_path, "w") as f:
        json.dump(payload, f, indent=2)

    print(f"\nSuccess! Benchmark JSON data written to {output_path}")
    print(f"Summary stats:")
    print(f"  - Total benchmark measurements processed: {summary_stats['total_benchmarks']}")
    print(f"  - Total unique groups: {summary_stats['group_count']}")

if __name__ == "__main__":
    main()
