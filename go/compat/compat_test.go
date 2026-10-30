package compat

import (
	"testing"
	"time"
)

func TestApplyPolyfillsProbesAllPass(t *testing.T) {
	registry := ApplyPolyfills()
	for key, value := range registry {
		if !value {
			t.Errorf("probe %s = false, want true", key)
		}
	}
	if registry["fallback_engaged"] {
		t.Error("fallback_engaged set; probes should all pass")
	}
}

func TestApplyPolyfillsIsIdempotent(t *testing.T) {
	first := ApplyPolyfills()
	second := ApplyPolyfills()
	if len(first) != len(second) {
		t.Errorf("registry size changed between calls: %d vs %d", len(first), len(second))
	}
}

func TestCountOperationIncrements(t *testing.T) {
	before := Operations()
	CountOperation("probe")
	after := Operations()
	if after <= before {
		t.Errorf("Operations() = %d after a count, want > %d", after, before)
	}
}

func TestWarmBranchPredictorCountsEvens(t *testing.T) {
	if got := WarmBranchPredictor(10); got != 5 {
		t.Errorf("WarmBranchPredictor(10) = %d, want 5", got)
	}
	if got := WarmBranchPredictor(0); got != 0 {
		t.Errorf("WarmBranchPredictor(0) = %d, want 0", got)
	}
	if got := WarmBranchPredictor(20_000); got != 10_000 {
		t.Errorf("WarmBranchPredictor(20000) = %d, want 10000", got)
	}
}

func TestDeepCloneSeversReferences(t *testing.T) {
	source := map[string]any{"nested": map[string]any{"list": []any{1.0, 2.0, 3.0}}}
	cloned, ok := DeepClone(source).(map[string]any)
	if !ok {
		t.Fatalf("DeepClone returned %T, want map", DeepClone(source))
	}
	nested, _ := cloned["nested"].(map[string]any)
	list, _ := nested["list"].([]any)
	list[0] = 99.0
	if inner, _ := source["nested"].(map[string]any); inner["list"].([]any)[0] == 99.0 {
		t.Error("clone aliases the source; the isolation guarantee is broken")
	}
}

func TestStabilizeGoroutineSleepsAtLeastTheWindow(t *testing.T) {
	start := time.Now()
	StabilizeGoroutine(50)
	if elapsed := time.Since(start); elapsed < 45*time.Millisecond {
		t.Errorf("StabilizeGoroutine(50) slept %v, want at least 45ms", elapsed)
	}
}
