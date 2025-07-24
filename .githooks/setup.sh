#!/bin/bash

# Setup script for Git hooks
# This script configures Git to use the custom hooks directory

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Setting up Git hooks for conventional commits...${NC}"

HOOKS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

git config core.hooksPath "$HOOKS_ROOT"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Git hooks configured successfully!${NC}"
    echo -e "${GREEN}✓ Conventional commits validation is now active${NC}"
    echo ""
    echo -e "${YELLOW}To test the hook, try making a commit with an invalid format:${NC}"
    echo "  git commit -m \"invalid commit message\""
    echo ""
    echo -e "${YELLOW}Valid commit format:${NC}"
    echo "  git commit -m \"feat: add new feature\""
    echo "  git commit -m \"fix(auth): resolve login issue\""
else
    echo "✗ Failed to configure Git hooks"
    exit 1
fi
