## Cursor
At the start of selection the `Cursor::child_index`  is saved. When selecting certain cursor features are changed. This includes:
- Moving up and down now simply moves the cursor to the start or end of the current sequence,
- Moving out of a sequence moves into the parent container's parent sequence and pushes the current `SequenceId` and the parent `ContainerId` onto a stack.
- Containers in the current sequence are not descended into but moved over, unless they match the `ContainerId` on the top of the stack then they are descended into, the top of the stack popped and the top of the stack's `SequenceId` is descended into from the container.
## Insertion 
The insertion of containers will now take in the selected area as a new `Sequence` and create itself with that sequence already within it. Sequences that normally require confirmation (such as brackets) will be automatically confirmed.
## Rendering
If a container is selected it is rendered with the toggle `highlighted` set to `true`. If it is a text child and the cursor is partway through it then it is given the selected range.