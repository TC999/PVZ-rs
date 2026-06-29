import yaml

with open('diff.yaml') as f:
    r = yaml.safe_load(f)

print("=== MISMATCHES ===")
for m in r['mismatch']:
    print(f"  {m['name']:35s} cpp={m['cpp_params']} rust={m['rust_params']}")

print()
print("=== NON-TRIANGLE MISSING ===")
count = 0
for m in r['missing']:
    name = m['name']
    if not name.startswith(('TodDrawTriangle', 'DrawTriangle')):
        sig = m['cpp'].get('signature', '')[:60]
        print(f"  {name:35s} sig={sig}")
        count += 1
        if count >= 50:
            print("  ... (truncated)")
            break
print(f"\nNon-triangle missing count: >= {count}")

# Count how many are triangle vs non-triangle
tri_count = sum(1 for m in r['missing'] if m['name'].startswith(('TodDrawTriangle', 'DrawTriangle')))
non_tri = len(r['missing']) - tri_count
print(f"Triangle missing: {tri_count}")
print(f"Non-triangle missing: {non_tri}")
print(f"Total missing: {len(r['missing'])}")
print(f"Total mismatches: {len(r['mismatch'])}")
