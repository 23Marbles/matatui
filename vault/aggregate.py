import os
from pathlib import Path

file = open('./total.txt', 'w')

for p in Path('.').iterdir():
    if p.is_file() and p.suffix == '.md':
        content = p.read_text(encoding='utf-8')
        file.write('\n\n# ' + p.name + '\n')
        file.write(content)