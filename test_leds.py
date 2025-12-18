#!/usr/bin/env python3
"""Test script to verify all LEDs are working in animations."""
import sys
sys.path.insert(0, 'src')

from led_map import LED_REGIONS

print("\n=== LED MAPPING TEST ===\n")

# Verify all LEDs are mapped
all_mapped_leds = set()
for region_name, leds in LED_REGIONS.items():
    if region_name == 'all':
        continue
    if isinstance(leds, list):
        all_mapped_leds.update(leds)
        print(f"{region_name:30s}: LEDs {min(leds):2d}-{max(leds):2d} ({len(leds):2d} LEDs)")
    else:
        all_mapped_leds.add(leds)
        print(f"{region_name:30s}: LED  {leds:2d}       ( 1 LED)")

print(f"\n{'Total mapped':<30s}: {len(all_mapped_leds)} / 84 LEDs")

missing = set(range(84)) - all_mapped_leds
if missing:
    print(f"⚠️  Missing LEDs: {sorted(missing)}")
    sys.exit(1)
else:
    print("✅ All 84 LEDs are properly mapped!")

# Test that cpu and gpu regions don't overlap and cover everything
cpu_region = set(LED_REGIONS['cpu'])
gpu_region = set(LED_REGIONS['gpu'])

print(f"\nCPU region: {len(cpu_region)} LEDs (0-41)")
print(f"GPU region: {len(gpu_region)} LEDs (42-83)")

overlap = cpu_region & gpu_region
if overlap:
    print(f"⚠️  CPU and GPU overlap at LEDs: {sorted(overlap)}")
else:
    print("✅ No overlap between CPU and GPU regions")

if cpu_region | gpu_region == set(range(84)):
    print("✅ CPU + GPU regions cover all 84 LEDs")
else:
    print(f"⚠️  CPU + GPU don't cover all LEDs")

print("\n=== TEST COMPLETE ===\n")
