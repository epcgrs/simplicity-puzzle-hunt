#!/bin/bash

# Script: list-puzzles.sh
# Purpose: List, verify and manage puzzle status
# Usage:
#   ./list-puzzles.sh           # Interactive mode (asks to archive)
#   ./list-puzzles.sh --auto    # Auto-archive solved puzzles
#   ./list-puzzles.sh --help    # Show help

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Parse command line arguments
AUTO_MODE=false
SHOW_HELP=false

for arg in "$@"; do
    case $arg in
        --auto|-a)
            AUTO_MODE=true
            shift
            ;;
        --help|-h)
            SHOW_HELP=true
            shift
            ;;
        *)
            ;;
    esac
done

# Show help if requested
if [ "$SHOW_HELP" = true ]; then
    echo ""
    echo "🎯 Simplicity Puzzle Hunt - List & Manage Puzzles"
    echo ""
    echo "Usage:"
    echo "  ./list-puzzles.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --auto, -a     Auto-archive solved puzzles without prompting"
    echo "  --help, -h     Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./list-puzzles.sh          # Interactive mode"
    echo "  ./list-puzzles.sh --auto   # Auto-archive mode (for cron)"
    echo ""
    echo "Archive location: ./archived_puzzles/"
    echo ""
    exit 0
fi

# Display header based on mode
if [ "$AUTO_MODE" = true ]; then
    echo ""
    echo -e "${CYAN}🤖 AUTO-ARCHIVE MODE${NC}"
    echo "========================"
    echo "$(date '+%Y-%m-%d %H:%M:%S')"
    echo ""
else
    echo ""
    echo "╔══════════════════════════════════════╗"
    echo "║       🎯 PUZZLE HUNT STATUS 🎯       ║"
    echo "╚══════════════════════════════════════╝"
    echo ""
fi

# Create archived folder if it doesn't exist
ARCHIVE_DIR="archived_puzzles"
if [ ! -d "$ARCHIVE_DIR" ]; then
    mkdir -p "$ARCHIVE_DIR"
    if [ "$AUTO_MODE" = false ]; then
        echo "📁 Created archive directory: $ARCHIVE_DIR"
        echo ""
    fi
fi

# Show mode info for interactive mode
if [ "$AUTO_MODE" = false ]; then
    echo "💡 Tip: Use './list-puzzles.sh --auto' for automatic archiving"
    echo ""
fi

# Elements CLI path
ELEMENTS_CLI="$HOME/Desktop/hub/blockchain/elements/src/elements-cli"

# Counters
count_total=0
count_active=0
count_solved=0
count_error=0
total_prize_active=0

# Arrays to store puzzles
declare -a active_puzzles
declare -a solved_puzzles

echo "🔍 Scanning puzzle files..."
echo ""

# Function to check UTXO status
check_utxo_status() {
    local address=$1
    local txid=$2
    local vout=$3

    # Try to get UTXO info
    if [ -f "$ELEMENTS_CLI" ]; then
        # Check specific UTXO
        utxo_result=$($ELEMENTS_CLI -chain=liquidtestnet gettxout "$txid" "$vout" 2>/dev/null)

        if [ ! -z "$utxo_result" ] && [ "$utxo_result" != "null" ]; then
            # UTXO exists
            return 0
        else
            # UTXO spent or doesn't exist
            return 1
        fi
    else
        # Can't check, assume unknown
        return 2
    fi
}

# Process all puzzle files
for puzzle_file in puzzle_*.json; do
    # Skip if not a regular file or if it's a SECRET file
    if [[ ! -f "$puzzle_file" ]] || [[ "$puzzle_file" == *"_SECRET.json" ]]; then
        continue
    fi

    count_total=$((count_total + 1))

    # Extract information from JSON
    address=$(jq -r '.address' "$puzzle_file" 2>/dev/null)
    amount=$(jq -r '.amount' "$puzzle_file" 2>/dev/null)
    target_hash=$(jq -r '.target_hash' "$puzzle_file" 2>/dev/null)
    hint=$(jq -r '.hint // "No hint provided"' "$puzzle_file" 2>/dev/null)
    txid=$(jq -r '.txid' "$puzzle_file" 2>/dev/null)
    vout=$(jq -r '.vout // 0' "$puzzle_file" 2>/dev/null)
    created_at=$(jq -r '.created_at // "Unknown"' "$puzzle_file" 2>/dev/null)

    # Check if we have valid data
    if [ "$address" == "null" ] || [ -z "$address" ]; then
        echo -e "${RED}⚠️  Invalid puzzle file: $puzzle_file${NC}"
        count_error=$((count_error + 1))
        continue
    fi

    # Check UTXO status
    check_utxo_status "$address" "$txid" "$vout"
    status=$?

    if [ $status -eq 0 ]; then
        # UTXO is active
        count_active=$((count_active + 1))
        active_puzzles+=("$puzzle_file")

        # Add to total active prize
        if [[ "$amount" =~ ^[0-9]+\.?[0-9]*$ ]]; then
            total_prize_active=$(echo "$total_prize_active + $amount" | bc)
        fi

        echo -e "${GREEN}✅ ACTIVE PUZZLE${NC}"
        echo "   📄 File: $puzzle_file"
        echo "   📍 Address: ${address:0:20}..."
        echo "   💰 Prize: $amount L-BTC"
        echo "   💡 Hint: \"$hint\""
        echo "   🔐 Hash: ${target_hash:0:20}..."
        echo "   📅 Created: $created_at"
        echo ""

    elif [ $status -eq 1 ]; then
        # UTXO is spent (puzzle solved)
        count_solved=$((count_solved + 1))
        solved_puzzles+=("$puzzle_file")

        echo -e "${YELLOW}🏆 SOLVED PUZZLE${NC}"
        echo "   📄 File: $puzzle_file"
        echo "   📍 Address: ${address:0:20}..."
        echo "   💰 Prize was: $amount L-BTC"
        echo "   💡 Hint was: \"$hint\""

        # Handle archiving based on mode
        if [ "$AUTO_MODE" = true ]; then
            # Auto-archive mode: archive without asking
            timestamp=$(date +%Y%m%d_%H%M%S)
            archived_name="${ARCHIVE_DIR}/${timestamp}_${puzzle_file}"

            mv "$puzzle_file" "$archived_name"
            echo -e "   ${GREEN}✓ Auto-archived to: ${archived_name}${NC}"

            # Also move the SECRET file if it exists
            secret_file="${puzzle_file%.json}_SECRET.json"
            if [ -f "$secret_file" ]; then
                archived_secret="${ARCHIVE_DIR}/${timestamp}_${secret_file}"
                mv "$secret_file" "$archived_secret"
                echo -e "   ${GREEN}✓ Secret archived to: ${archived_secret}${NC}"
            fi
        else
            # Interactive mode: ask before archiving
            echo ""
            echo -n "   Archive this solved puzzle? (y/n) "
            read -r archive_choice
            if [[ "$archive_choice" == "y" || "$archive_choice" == "Y" ]]; then
                # Move to archive
                mv "$puzzle_file" "$ARCHIVE_DIR/"

                # Also move the SECRET file if it exists
                secret_file="${puzzle_file%.json}_SECRET.json"
                if [ -f "$secret_file" ]; then
                    mv "$secret_file" "$ARCHIVE_DIR/"
                    echo -e "   ${GREEN}✓ Archived both public and secret files${NC}"
                else
                    echo -e "   ${GREEN}✓ Archived puzzle file${NC}"
                fi
            fi
        fi
        echo ""

    else
        # Cannot determine status
        echo -e "${BLUE}❓ UNKNOWN STATUS${NC}"
        echo "   📄 File: $puzzle_file"
        echo "   📍 Address: ${address:0:20}..."
        echo "   💰 Prize: $amount L-BTC"
        echo "   💡 Hint: \"$hint\""
        echo "   ⚠️  Cannot verify UTXO status (elementsd not available)"
        echo ""
    fi
done

# Show summary
echo ""
echo "╔══════════════════════════════════════╗"
echo "║           📊 SUMMARY                 ║"
echo "╚══════════════════════════════════════╝"
echo ""
echo "📈 Statistics:"
echo "   Total puzzles found: $count_total"
echo -e "   ${GREEN}Active puzzles: $count_active${NC}"
echo -e "   ${YELLOW}Solved puzzles: $count_solved${NC}"
if [ $count_error -gt 0 ]; then
    echo -e "   ${RED}Invalid files: $count_error${NC}"
fi
echo ""

if [ $count_active -gt 0 ]; then
    echo "💰 Total prize pool active: $total_prize_active L-BTC"
    echo ""
    echo "🎮 Active puzzles ready to solve:"
    for puzzle in "${active_puzzles[@]}"; do
        echo "   - $puzzle"
    done
    echo ""
    echo "💡 To solve a puzzle:"
    echo "   cargo run --bin solve_puzzle -- <puzzle_file> <secret> <your_address>"
fi

if [ $count_solved -gt 0 ]; then
    echo ""
    echo "🏆 Recently solved puzzles:"
    for puzzle in "${solved_puzzles[@]}"; do
        if [ -f "$puzzle" ]; then
            echo "   - $puzzle (not archived yet)"
        fi
    done
fi

# Check archived folder
archived_count=$(ls -1 "$ARCHIVE_DIR"/puzzle_*.json 2>/dev/null | wc -l)
if [ $archived_count -gt 0 ]; then
    echo ""
    echo "🗄️ Archived puzzles: $archived_count"
    echo "   View with: ls -la $ARCHIVE_DIR/"
fi

if [ $count_total -eq 0 ]; then
    echo ""
    echo "❌ No puzzles found."
    echo ""
    echo "💡 To create a puzzle, use:"
    echo '   cargo run --bin create_puzzle -- "secret" 0.1 "Your hint here"'
fi

# Final message based on mode
if [ "$AUTO_MODE" = true ]; then
    echo ""
    echo "═══════════════════════════════════════"
    echo "   Auto-archive completed at $(date '+%H:%M:%S')"
    echo "═══════════════════════════════════════"
    echo ""
    # Log for cron
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Processed: $count_total puzzles, Archived: $count_solved, Active: $count_active"
else
    echo ""
    echo "═══════════════════════════════════════"
    echo "        Happy Puzzle Hunting! 🎯"
    echo "═══════════════════════════════════════"
    echo ""
fi