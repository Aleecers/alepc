#! /usr/bin/bash

MASTER_LABEL="☕ Released"
DEV_LABEL="💻 in dev"
REPO="github.com/aleecers/alepc"
PROJECT_NUMBER=1
TO_STATUS="Released"
INUMS=$(gh issue list -s closed -l "$DEV_LABEL" -L 150 --json number -R "$REPO" | jq -c ".[].number")
PNUMS=$(gh pr list -s closed -l "$DEV_LABEL" -L 150 --json number -R "$REPO" | jq -c ".[].number")

# install projects extension
gh extension install heaths/gh-projects --pin v0.6.1

_move_to_released () {
    gh projects edit $PROJECT_NUMBER --add-issue $1 -f Status="$TO_STATUS" -R "$REPO"
}

for NUM in $INUMS
do
    gh issue edit $NUM --remove-label "$DEV_LABEL" --add-label "$MASTER_LABEL" -R "$REPO"
    _move_to_released $NUM
done

for NUM in $PNUMS
do
    gh pr edit $NUM --remove-label "$DEV_LABEL" --add-label "$MASTER_LABEL" -R "$REPO"
    _move_to_released $NUM
done
