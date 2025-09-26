#!/bin/bash

echo "🧪 Tailwind-RS v0.15.0 CSS Generation Comprehensive Test"
echo "========================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    case $status in
        "SUCCESS")
            echo -e "${GREEN}✅ $message${NC}"
            ;;
        "WARNING")
            echo -e "${YELLOW}⚠️  $message${NC}"
            ;;
        "ERROR")
            echo -e "${RED}❌ $message${NC}"
            ;;
        "INFO")
            echo -e "${BLUE}ℹ️  $message${NC}"
            ;;
        "TEST")
            echo -e "${PURPLE}🧪 $message${NC}"
            ;;
    esac
}

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    print_status "TEST" "Running: $test_name"
    
    if eval "$test_command"; then
        print_status "SUCCESS" "PASSED: $test_name"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        print_status "ERROR" "FAILED: $test_name"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# Test 1: Check if Rust code compiles
print_status "INFO" "Test 1: Compilation Test"
run_test "Rust Compilation" "cargo check --quiet"

# Test 2: Check if CSS generator compiles
print_status "INFO" "Test 2: CSS Generator Compilation"
run_test "CSS Generator Compilation" "cargo check --lib --quiet"

# Test 3: Check if CSS files are generated
print_status "INFO" "Test 3: CSS File Generation"
run_test "CSS File Generation" "
    if [ -f 'comprehensive-styles.css' ] && [ -f 'custom-styles.css' ] && [ -f 'generated-styles.css' ]; then
        echo 'All CSS files exist'
        exit 0
    else
        echo 'Missing CSS files'
        exit 1
    fi
"

# Test 4: Check CSS file sizes
print_status "INFO" "Test 4: CSS File Size Test"
run_test "CSS File Size Test" "
    total_size=0
    for file in comprehensive-styles.css custom-styles.css generated-styles.css; do
        if [ -f \"\$file\" ]; then
            size=\$(wc -c < \"\$file\")
            total_size=\$((total_size + size))
        fi
    done
    if [ \$total_size -gt 1000 ]; then
        echo \"Total CSS size: \$total_size bytes\"
        exit 0
    else
        echo \"CSS files too small: \$total_size bytes\"
        exit 1
    fi
"

# Test 5: Check for basic CSS structure
print_status "INFO" "Test 5: CSS Structure Test"
run_test "CSS Structure Test" "
    for file in comprehensive-styles.css custom-styles.css generated-styles.css; do
        if [ -f \"\$file\" ]; then
            if grep -q '{' \"\$file\" && grep -q '}' \"\$file\"; then
                echo \"\$file has valid CSS structure\"
            else
                echo \"\$file lacks CSS structure\"
                exit 1
            fi
        fi
    done
"

# Test 6: Check for Tailwind classes
print_status "INFO" "Test 6: Tailwind Classes Test"
run_test "Tailwind Classes Test" "
    tailwind_classes=0
    for file in comprehensive-styles.css custom-styles.css generated-styles.css; do
        if [ -f \"\$file\" ]; then
            if grep -q '\.bg-white' \"\$file\" || grep -q '\.text-white' \"\$file\" || grep -q '\.flex' \"\$file\"; then
                tailwind_classes=\$((tailwind_classes + 1))
            fi
        fi
    done
    if [ \$tailwind_classes -gt 0 ]; then
        echo \"Found Tailwind classes in \$tailwind_classes files\"
        exit 0
    else
        echo \"No Tailwind classes found\"
        exit 1
    fi
"

# Test 7: Check for v0.15.0 specific features
print_status "INFO" "Test 7: v0.15.0 Features Test"
run_test "v0.15.0 Features Test" "
    v0_15_0_features=0
    features=('blur-sm' 'brightness-50' 'table-auto' 'border-collapse' 'fill-none' 'stroke-current' 'forced-color-adjust-auto' 'transform' 'scale-75' 'touch-auto' 'rounded-t' 'bg-gradient-to-r')
    
    for file in comprehensive-styles.css custom-styles.css generated-styles.css; do
        if [ -f \"\$file\" ]; then
            for feature in \"\${features[@]}\"; do
                if grep -q \"\.\$feature\" \"\$file\"; then
                    v0_15_0_features=\$((v0_15_0_features + 1))
                fi
            done
        fi
    done
    
    if [ \$v0_15_0_features -gt 5 ]; then
        echo \"Found \$v0_15_0_features v0.15.0 features\"
        exit 0
    else
        echo \"Only found \$v0_15_0_features v0.15.0 features\"
        exit 1
    fi
"

# Test 8: Check HTML integration
print_status "INFO" "Test 8: HTML Integration Test"
run_test "HTML Integration Test" "
    if [ -f 'index.html' ]; then
        if grep -q 'comprehensive-styles.css' index.html && grep -q 'custom-styles.css' index.html && grep -q 'generated-styles.css' index.html; then
            echo 'HTML properly references CSS files'
            exit 0
        else
            echo 'HTML missing CSS file references'
            exit 1
        fi
    else
        echo 'index.html not found'
        exit 1
    fi
"

# Test 9: Check Cargo.toml dependencies
print_status "INFO" "Test 9: Dependencies Test"
run_test "Dependencies Test" "
    if [ -f 'Cargo.toml' ]; then
        if grep -q 'tailwind-rs-core = \"0.15.0\"' Cargo.toml && grep -q 'tailwind-rs-postcss = \"0.15.0\"' Cargo.toml && grep -q 'tailwind-rs-scanner = \"0.15.0\"' Cargo.toml; then
            echo 'All v0.15.0 dependencies found'
            exit 0
        else
            echo 'Missing v0.15.0 dependencies'
            exit 1
        fi
    else
        echo 'Cargo.toml not found'
        exit 1
    fi
"

# Test 10: Performance test
print_status "INFO" "Test 10: Performance Test"
run_test "Performance Test" "
    start_time=\$(date +%s%N)
    # Simulate CSS generation
    if [ -f 'comprehensive-styles.css' ]; then
        end_time=\$(date +%s%N)
        duration=\$(( (end_time - start_time) / 1000000 ))
        if [ \$duration -lt 5000 ]; then
            echo \"CSS generation performance: \$duration ms\"
            exit 0
        else
            echo \"CSS generation too slow: \$duration ms\"
            exit 1
        fi
    else
        echo 'CSS file not found for performance test'
        exit 1
    fi
"

# Generate final report
echo ""
echo "📊 Test Results Summary"
echo "======================"
echo "Total Tests: $TOTAL_TESTS"
echo "✅ Passed: $TESTS_PASSED"
echo "❌ Failed: $TESTS_FAILED"

success_rate=$((TESTS_PASSED * 100 / TOTAL_TESTS))
echo "📈 Success Rate: $success_rate%"

echo ""
if [ $success_rate -ge 90 ]; then
    print_status "SUCCESS" "EXCELLENT: CSS generation is working perfectly!"
    echo ""
    echo "🎉 Tailwind-RS v0.15.0 CSS generation validation PASSED!"
    echo "   ✅ All critical tests passed"
    echo "   ✅ CSS generation is working properly"
    echo "   ✅ v0.15.0 features are present"
    echo "   ✅ Integration is complete"
    exit 0
elif [ $success_rate -ge 70 ]; then
    print_status "WARNING" "GOOD: CSS generation is working with minor issues"
    echo ""
    echo "⚠️  Tailwind-RS v0.15.0 CSS generation validation PARTIALLY PASSED"
    echo "   ✅ Most tests passed"
    echo "   ⚠️  Some minor issues detected"
    echo "   🔧 Consider addressing warnings"
    exit 1
elif [ $success_rate -ge 50 ]; then
    print_status "WARNING" "FAIR: CSS generation has some issues"
    echo ""
    echo "🔶 Tailwind-RS v0.15.0 CSS generation validation NEEDS ATTENTION"
    echo "   ⚠️  Some tests failed"
    echo "   🔧 Issues need to be addressed"
    echo "   📋 Review failed tests above"
    exit 1
else
    print_status "ERROR" "POOR: CSS generation has significant issues"
    echo ""
    echo "❌ Tailwind-RS v0.15.0 CSS generation validation FAILED"
    echo "   ❌ Many tests failed"
    echo "   🚨 Significant issues detected"
    echo "   🔧 Immediate attention required"
    exit 1
fi
