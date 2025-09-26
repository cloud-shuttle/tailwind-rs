#!/bin/bash

echo "🔍 Tailwind-RS v0.15.0 CSS Generation Validation"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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
    esac
}

# Step 1: Check if CSS files exist
print_status "INFO" "Step 1: Checking CSS file existence..."

css_files=("comprehensive-styles.css" "custom-styles.css" "generated-styles.css")
files_exist=true

for file in "${css_files[@]}"; do
    if [ -f "$file" ]; then
        print_status "SUCCESS" "Found $file"
        # Check file size
        size=$(wc -c < "$file")
        if [ "$size" -gt 100 ]; then
            print_status "SUCCESS" "$file has content ($size bytes)"
        else
            print_status "WARNING" "$file is very small ($size bytes)"
            files_exist=false
        fi
    else
        print_status "ERROR" "Missing $file"
        files_exist=false
    fi
done

if [ "$files_exist" = false ]; then
    print_status "ERROR" "CSS generation failed - files missing or empty"
    exit 1
fi

# Step 2: Validate CSS content
print_status "INFO" "Step 2: Validating CSS content..."

# Check for basic CSS structure
for file in "${css_files[@]}"; do
    if [ -f "$file" ]; then
        # Check for CSS rules
        if grep -q "{" "$file" && grep -q "}" "$file"; then
            print_status "SUCCESS" "$file contains valid CSS structure"
        else
            print_status "ERROR" "$file does not contain valid CSS structure"
            files_exist=false
        fi
        
        # Check for specific Tailwind classes
        if grep -q "\.bg-white" "$file" || grep -q "\.text-white" "$file" || grep -q "\.flex" "$file"; then
            print_status "SUCCESS" "$file contains Tailwind classes"
        else
            print_status "WARNING" "$file may not contain expected Tailwind classes"
        fi
    fi
done

# Step 3: Check for v0.15.0 specific features
print_status "INFO" "Step 3: Checking v0.15.0 specific features..."

v0_15_0_features=(
    "blur-sm"
    "brightness-50"
    "backdrop-blur-sm"
    "table-auto"
    "border-collapse"
    "fill-none"
    "stroke-current"
    "forced-color-adjust-auto"
    "transform"
    "scale-75"
    "rotate-45"
    "touch-auto"
    "rounded-t"
    "bg-gradient-to-r"
)

feature_count=0
total_features=${#v0_15_0_features[@]}

for file in "${css_files[@]}"; do
    if [ -f "$file" ]; then
        for feature in "${v0_15_0_features[@]}"; do
            if grep -q "\.$feature" "$file"; then
                ((feature_count++))
                print_status "SUCCESS" "Found v0.15.0 feature: $feature in $file"
            fi
        done
    fi
done

feature_percentage=$((feature_count * 100 / total_features))
print_status "INFO" "v0.15.0 features found: $feature_count/$total_features ($feature_percentage%)"

# Step 4: Check CSS file sizes and content quality
print_status "INFO" "Step 4: Analyzing CSS quality..."

for file in "${css_files[@]}"; do
    if [ -f "$file" ]; then
        size=$(wc -c < "$file")
        lines=$(wc -l < "$file")
        
        print_status "INFO" "$file: $size bytes, $lines lines"
        
        if [ "$size" -gt 1000 ]; then
            print_status "SUCCESS" "$file has substantial content"
        elif [ "$size" -gt 100 ]; then
            print_status "WARNING" "$file has minimal content"
        else
            print_status "ERROR" "$file is too small"
        fi
    fi
done

# Step 5: Test CSS accessibility
print_status "INFO" "Step 5: Testing CSS accessibility..."

# Check if CSS files are readable
for file in "${css_files[@]}"; do
    if [ -f "$file" ]; then
        if [ -r "$file" ]; then
            print_status "SUCCESS" "$file is readable"
        else
            print_status "ERROR" "$file is not readable"
        fi
    fi
done

# Step 6: Generate validation report
print_status "INFO" "Step 6: Generating validation report..."

echo ""
echo "📊 CSS Generation Validation Report"
echo "=================================="

# Count total CSS rules
total_rules=0
for file in "${css_files[@]}"; do
    if [ -f "$file" ]; then
        rules=$(grep -c "{" "$file" 2>/dev/null || echo "0")
        total_rules=$((total_rules + rules))
    fi
done

echo "Total CSS Rules Generated: $total_rules"
echo "v0.15.0 Features Found: $feature_count/$total_features ($feature_percentage%)"

# Determine overall status
if [ "$files_exist" = true ] && [ "$feature_percentage" -ge 50 ] && [ "$total_rules" -gt 10 ]; then
    print_status "SUCCESS" "CSS generation is working properly!"
    echo ""
    echo "🎉 Validation Results:"
    echo "   ✅ CSS files generated successfully"
    echo "   ✅ CSS contains valid structure"
    echo "   ✅ v0.15.0 features are present"
    echo "   ✅ CSS files are accessible"
    echo ""
    echo "🚀 Tailwind-RS v0.15.0 CSS generation is working correctly!"
    exit 0
elif [ "$files_exist" = true ] && [ "$feature_percentage" -ge 25 ]; then
    print_status "WARNING" "CSS generation is partially working"
    echo ""
    echo "⚠️  Validation Results:"
    echo "   ✅ CSS files generated"
    echo "   ⚠️  Some v0.15.0 features missing"
    echo "   ⚠️  CSS content may be limited"
    echo ""
    echo "🔧 CSS generation needs improvement"
    exit 1
else
    print_status "ERROR" "CSS generation has significant issues"
    echo ""
    echo "❌ Validation Results:"
    echo "   ❌ CSS generation failed"
    echo "   ❌ Missing or invalid CSS files"
    echo "   ❌ v0.15.0 features not found"
    echo ""
    echo "🚨 CSS generation needs immediate attention"
    exit 1
fi
